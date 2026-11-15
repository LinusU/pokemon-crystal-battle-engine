use rgb2rust_output::bootstrap::{self, BattleConfig, RunOptions};
use rgb2rust_output::runtime::Cpu;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cli_args = match parse_cli_args(&args[1..]) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("error: {e}");
            eprintln!(
                "usage: {} [<scenario.toml> | --random] [--seed <n>] [--record <out.toml>] [--play <recording.toml>] [--show-text] [--vs-ai [--ai-model <model.bin>] [--ai-player]]",
                args[0],
            );
            std::process::exit(1);
        }
    };

    // `--play` recovers the matchup from the recording's embedded `[scenario.*]`
    // block, which we replay directly. An explicit positional scenario overrides
    // it — re-running the recorded script against an edited matchup. The
    // recording's resolved seed always wins over any `seed = ...` on the scenario.
    let playback_recording = cli_args.play.as_deref().map(|p| match bootstrap::player::load(p) {
        Ok(rec) => rec,
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    });

    // Resolve the battle config. The resolved matchup is always embedded into a
    // `--record` output (see `run_with`), so a recording is self-contained
    // regardless of battle kind.
    let battle_config: Option<BattleConfig> = if cli_args.random {
        // Live `--random` run: pick a base seed (CLI > entropy) and generate the
        // matchup. The base seed splits into independent generation and engine
        // seeds inside `random_battle_config` — the engine seed lands in
        // `BattleHeader.seed`.
        let base_seed = cli_args.seed.unwrap_or_else(random_entropy_seed);
        Some(bootstrap::random::random_battle_config_verbose(base_seed))
    } else if let Some(path) = cli_args.scenario.clone() {
        // Explicit positional scenario. On a plain run this is the battle to
        // play; on `--play` it overrides the recording's embedded matchup so the
        // recorded script can be re-run against an edited one. Checked before the
        // embedded block so the override always wins.
        match bootstrap::load_battle(std::path::Path::new(&path)) {
            Ok(cfg) => Some(cfg),
            Err(e) => {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        }
    } else if let Some(scenario) = playback_recording.as_ref().and_then(|r| r.scenario.as_ref()) {
        // Self-contained recording: replay the embedded matchup verbatim.
        // `seed_override` below pins the engine seed.
        Some(scenario.clone())
    } else if playback_recording.is_some() {
        eprintln!(
            "error: --play recording has no embedded `[scenario.*]` block — pass a scenario.toml positionally"
        );
        std::process::exit(1);
    } else {
        None
    };
    let record_path = cli_args.record;

    // This binary runs a single battle; it never boots into the overworld.
    let Some(mut battle_config) = battle_config else {
        eprintln!(
            "error: no battle specified — pass a <scenario.toml>, --random, or --play <recording.toml>"
        );
        std::process::exit(1);
    };

    // `--vs-ai`: the trained AI plays the enemy trainer.
    let decider = if cli_args.ai.vs_ai {
        if battle_config.battle.kind != bootstrap::BattleKind::TrainerBattle {
            eprintln!("error: --vs-ai needs a trainer battle (a trainer_battle scenario or --random)");
            std::process::exit(1);
        }
        battle_config.battle.enemy_control = bootstrap::EnemyControl::External;
        match ai_decider(&cli_args.ai) {
            Ok(d) => Some(d),
            Err(e) => {
                eprintln!("error: {e}");
                std::process::exit(1);
            }
        }
    } else {
        None
    };

    #[cfg(feature = "gui")]
    gui::run(battle_config, record_path, playback_recording, cli_args.show_text, decider);

    #[cfg(not(feature = "gui"))]
    run_headless(battle_config, record_path, playback_recording, cli_args.show_text, decider);
}

/// Something that answers some of the battle's decisions (the AI).
type BoxedDecider = Box<dyn bootstrap::decision::Decider>;

/// The AI opponent for `--vs-ai`: the built-in model, or `--ai-model`.
#[cfg(feature = "ai")]
fn ai_decider(args: &AiArgs) -> Result<BoxedDecider, String> {
    use rgb2rust_output::ai::{Agent, AiDecider};
    let agent = match &args.model {
        Some(path) => Agent::load(path)?,
        None => Agent::builtin(),
    };
    eprintln!("the trained AI plays the enemy trainer{}", if args.player { " and your side" } else { "" });
    Ok(Box::new(AiDecider { agent, player: args.player }))
}

#[cfg(not(feature = "ai"))]
fn ai_decider(_: &AiArgs) -> Result<BoxedDecider, String> {
    Err("--vs-ai needs the AI: build with `--features ai`".into())
}

/// Drive a single fully specified battle to completion on `cpu`. Shared by the
/// windowed runner (on its cpu thread) and the headless runner. The matchup is
/// already resolved; this only wires the recorder / playback knobs.
fn run_battle(
    cpu: &mut Cpu,
    config: &BattleConfig,
    record_path: Option<&std::path::Path>,
    playback: Option<bootstrap::player::Recording>,
    show_text: bool,
    decider: Option<BoxedDecider>,
) {
    cpu.decider = decider.map(std::cell::RefCell::new);
    let (seed_override, playback_steps) = match playback {
        Some(rec) => (Some(rec.seed), Some(rec.steps)),
        None => (None, None),
    };
    let options = RunOptions {
        record_path,
        seed_override,
        playback: playback_steps,
        quiet: false,
        show_text,
    };
    if let Err(e) = bootstrap::run_with(cpu, config, options) {
        eprintln!("battle bootstrap failed: {e}");
    }
}

/// Headless entry point (no `gui` feature): drive the battle to completion with
/// no window.
#[cfg(not(feature = "gui"))]
fn run_headless(
    config: BattleConfig,
    record_path: Option<std::path::PathBuf>,
    playback: Option<bootstrap::player::Recording>,
    show_text: bool,
    decider: Option<BoxedDecider>,
) {
    let mut cpu = Cpu::new_cgb(None, None);
    cpu.interrupt_vectors = rgb2rust_output::interrupt_vectors();
    run_battle(&mut cpu, &config, record_path.as_deref(), playback, show_text, decider);
}

/// Argv parser for `[<scenario.toml> | --random] [--seed <n>] [--record <out.toml>] [--play <rec.toml>]`.
/// The first non-flag positional is the scenario path. Flag order is
/// irrelevant. `--play` may be passed with or without a positional scenario —
/// when absent, the matchup is taken from the recording's embedded
/// `[scenario.*]` block.
/// `--random` is mutually exclusive with a positional scenario.toml and with
/// `--play` (a recorded random run is replayed by passing the recording to
/// `--play` alone; the recording carries the resolved matchup, so no
/// regeneration is needed).
struct CliArgs {
    scenario: Option<String>,
    record: Option<std::path::PathBuf>,
    play: Option<std::path::PathBuf>,
    random: bool,
    seed: Option<u64>,
    /// `--show-text`: print the original battle text as the engine prints it.
    show_text: bool,
    ai: AiArgs,
}

/// `--vs-ai`, `--ai-model <path>`, `--ai-player`.
#[derive(Default)]
struct AiArgs {
    /// The trained AI plays the enemy trainer (`enemy_control = "external"`).
    vs_ai: bool,
    /// A model file instead of the built-in one (implies `vs_ai`).
    model: Option<std::path::PathBuf>,
    /// The AI also plays the player's side: watch it play itself.
    player: bool,
}

fn parse_cli_args(args: &[String]) -> Result<CliArgs, String> {
    let mut scenario: Option<String> = None;
    let mut record: Option<std::path::PathBuf> = None;
    let mut play: Option<std::path::PathBuf> = None;
    let mut random = false;
    let mut seed: Option<u64> = None;
    let mut show_text = false;
    let mut ai = AiArgs::default();
    let mut i = 0;
    while i < args.len() {
        let arg = &args[i];
        match arg.as_str() {
            "--record" => {
                let path = args
                    .get(i + 1)
                    .ok_or_else(|| "--record requires a path argument".to_string())?;
                record = Some(std::path::PathBuf::from(path));
                i += 2;
            }
            "--play" => {
                let path = args
                    .get(i + 1)
                    .ok_or_else(|| "--play requires a path argument".to_string())?;
                play = Some(std::path::PathBuf::from(path));
                i += 2;
            }
            "--show-text" => {
                show_text = true;
                i += 1;
            }
            "--vs-ai" => {
                ai.vs_ai = true;
                i += 1;
            }
            "--ai-player" => {
                ai.vs_ai = true;
                ai.player = true;
                i += 1;
            }
            "--ai-model" => {
                let path = args.get(i + 1).ok_or_else(|| "--ai-model requires a path argument".to_string())?;
                ai.vs_ai = true;
                ai.model = Some(std::path::PathBuf::from(path));
                i += 2;
            }
            "--random" => {
                random = true;
                i += 1;
            }
            "--seed" => {
                let raw = args
                    .get(i + 1)
                    .ok_or_else(|| "--seed requires a number (decimal or 0x-hex)".to_string())?;
                seed = Some(parse_seed_arg(raw).ok_or_else(|| {
                    format!("--seed: could not parse {raw:?} (expected decimal or 0x-hex u64)")
                })?);
                i += 2;
            }
            other if other.starts_with("--") => {
                return Err(format!("unknown flag: {other}"));
            }
            _ => {
                if scenario.is_some() {
                    return Err(format!("unexpected positional argument: {arg}"));
                }
                scenario = Some(arg.clone());
                i += 1;
            }
        }
    }
    if random && scenario.is_some() {
        return Err("--random cannot be combined with a scenario path".into());
    }
    if random && play.is_some() {
        return Err(
            "--random cannot be combined with --play — use `--play <recording.toml>` alone to replay a recorded random run".into(),
        );
    }
    if seed.is_some() && play.is_some() {
        return Err(
            "--seed cannot be combined with --play — playback always uses the recording's seed".into(),
        );
    }
    if seed.is_some() && !random {
        return Err(
            "--seed without --random has no effect — put `seed = ...` in the scenario TOML instead".into(),
        );
    }
    if ai.vs_ai && play.is_some() {
        return Err("--vs-ai cannot be combined with --play — a recording already holds the AI's decisions".into());
    }
    Ok(CliArgs { scenario, record, play, random, seed, show_text, ai })
}

/// Parse `--seed` as decimal or `0x...`/`0X...` hex. Accepts the same shape
/// the recorder writes into `[recording] seed = "0x..."`.
fn parse_seed_arg(raw: &str) -> Option<u64> {
    let trimmed = raw.trim();
    if let Some(rest) = trimmed.strip_prefix("0x").or_else(|| trimmed.strip_prefix("0X")) {
        u64::from_str_radix(rest, 16).ok()
    } else {
        trimmed.parse::<u64>().ok()
    }
}

/// Draw a u64 seed for `--random` runs from system entropy. Logged via the
/// generator's own eprintln so the user can re-run with `--seed 0x<value>`.
fn random_entropy_seed() -> u64 {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0);
    let pid = std::process::id() as u64;
    let mut seed = nanos ^ pid.wrapping_mul(0x9e37_79b9_7f4a_7c15);
    if seed == 0 {
        seed = 0x9e37_79b9_7f4a_7c15;
    }
    seed
}

/// Windowed battle viewer: a winit window + glium renderer that display the
/// battle while the cpu runs on its own thread. Compiled only with the `gui`
/// feature — a plain build is headless and never pulls in glium/winit.
#[cfg(feature = "gui")]
mod gui {
    use std::sync::mpsc::{self, Sender, SyncSender};
    use std::thread;
    use winit::platform::pump_events::EventLoopExtPumpEvents;

    use rgb2rust_output::bootstrap::{self, BattleConfig};
    use rgb2rust_output::runtime::{Cpu, KeypadEvent, KeypadKey};

    #[derive(Default)]
    struct RenderOptions {
        linear_interpolation: bool,
    }

    /// Open the window and run until it is closed. The battle runs on a separate
    /// thread, streaming frames back over a channel and receiving keypad events.
    pub fn run(
        battle_config: BattleConfig,
        record_path: Option<std::path::PathBuf>,
        playback_recording: Option<bootstrap::player::Recording>,
        show_text: bool,
        decider: Option<super::BoxedDecider>,
    ) {
        let scale = 4;
        let (sender, receiver) = mpsc::sync_channel(1);
        let (keypad_sender, keypad_receiver) = mpsc::channel::<KeypadEvent>();

        let mut event_loop = winit::event_loop::EventLoop::new().unwrap();
        let window_builder = create_window_builder();
        let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
            .set_window_builder(window_builder)
            .build(&event_loop);
        set_window_size(&window, scale);

        let mut texture = glium::texture::texture2d::Texture2d::empty_with_format(
            &display,
            glium::texture::UncompressedFloatFormat::U8U8U8,
            glium::texture::MipmapsOption::NoMipmap,
            rgb2rust_output::runtime::SCREEN_W as u32,
            rgb2rust_output::runtime::SCREEN_H as u32,
        )
        .unwrap();

        let mut render_options = RenderOptions::default();
        let cpu_thread = thread::spawn(move || {
            run_game(
                sender,
                keypad_receiver,
                battle_config,
                record_path,
                playback_recording,
                show_text,
                decider,
            )
        });

        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
        loop {
            // The closure form is deprecated in winit 0.30 in favour of an
            // `ApplicationHandler`; it still pumps the same events.
            #[allow(deprecated)]
            let status = event_loop.pump_events(Some(std::time::Duration::ZERO), |event, elwt| {
                use winit::event::{ElementState, Event, WindowEvent};
                use winit::keyboard::{Key, NamedKey};

                if let Event::WindowEvent { event, .. } = event {
                    match event {
                        WindowEvent::CloseRequested => elwt.exit(),
                        WindowEvent::KeyboardInput { event, .. } => {
                            match (event.state, event.logical_key.as_ref()) {
                                (ElementState::Pressed, Key::Named(NamedKey::Escape)) => elwt.exit(),
                                (ElementState::Pressed, Key::Character("1")) => {
                                    set_window_size(&window, 1);
                                }
                                (ElementState::Pressed, Key::Character("r" | "R")) => {
                                    set_window_size(&window, scale);
                                }
                                (ElementState::Pressed, Key::Character("t" | "T")) => {
                                    render_options.linear_interpolation =
                                        !render_options.linear_interpolation;
                                }
                                (state, key) => {
                                    if let Some(keypad_key) = map_keypad_key(key) {
                                        send_keypad_event(&keypad_sender, state, keypad_key);
                                    }
                                }
                            }
                        }
                        WindowEvent::Resized(size) => display.resize(size.into()),
                        _ => {}
                    }
                }
            });

            if let winit::platform::pump_events::PumpStatus::Exit(_) = status {
                break;
            }

            match receiver.recv_timeout(std::time::Duration::from_millis(16)) {
                Ok(data) => recalculate_screen(&display, &mut texture, &data, &render_options),
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => break,
            }
        }

        drop(receiver);
        drop(keypad_sender);
        let _ = cpu_thread.join();
    }

    fn run_game(
        update_screen: SyncSender<Vec<u8>>,
        keypad_events: mpsc::Receiver<KeypadEvent>,
        config: BattleConfig,
        record_path: Option<std::path::PathBuf>,
        playback: Option<bootstrap::player::Recording>,
        show_text: bool,
        decider: Option<super::BoxedDecider>,
    ) {
        let mut cpu = Cpu::new_cgb(Some(update_screen), Some(keypad_events));
        cpu.interrupt_vectors = rgb2rust_output::interrupt_vectors();
        super::run_battle(&mut cpu, &config, record_path.as_deref(), playback, show_text, decider);
    }

    fn map_keypad_key(key: winit::keyboard::Key<&str>) -> Option<KeypadKey> {
        use winit::keyboard::{Key, NamedKey};

        match key {
            Key::Named(NamedKey::ArrowRight) => Some(KeypadKey::Right),
            Key::Named(NamedKey::ArrowLeft) => Some(KeypadKey::Left),
            Key::Named(NamedKey::ArrowUp) => Some(KeypadKey::Up),
            Key::Named(NamedKey::ArrowDown) => Some(KeypadKey::Down),
            Key::Named(NamedKey::Enter) => Some(KeypadKey::Start),
            Key::Named(NamedKey::Space) => Some(KeypadKey::Select),
            Key::Named(NamedKey::Backspace) => Some(KeypadKey::Select),
            Key::Character("z" | "Z") => Some(KeypadKey::A),
            Key::Character("x" | "X") => Some(KeypadKey::B),
            Key::Character("a" | "A") => Some(KeypadKey::A),
            Key::Character("s" | "S") => Some(KeypadKey::B),
            _ => None,
        }
    }

    fn send_keypad_event(
        sender: &Sender<KeypadEvent>,
        state: winit::event::ElementState,
        key: KeypadKey,
    ) {
        let event = match state {
            winit::event::ElementState::Pressed => KeypadEvent::Down(key),
            winit::event::ElementState::Released => KeypadEvent::Up(key),
        };
        let _ = sender.send(event);
    }

    fn create_window_builder() -> winit::window::WindowAttributes {
        winit::window::WindowAttributes::default().with_title("rgb2rust")
    }

    fn recalculate_screen<T>(
        display: &glium::Display<T>,
        texture: &mut glium::texture::texture2d::Texture2d,
        data: &[u8],
        render_options: &RenderOptions,
    ) where
        T: glium::glutin::surface::SurfaceTypeTrait
            + glium::glutin::surface::ResizeableSurface
            + 'static,
    {
        use glium::Surface;

        let interpolation_type = if render_options.linear_interpolation {
            glium::uniforms::MagnifySamplerFilter::Linear
        } else {
            glium::uniforms::MagnifySamplerFilter::Nearest
        };

        let raw_image = glium::texture::RawImage2d {
            data: std::borrow::Cow::Borrowed(data),
            width: rgb2rust_output::runtime::SCREEN_W as u32,
            height: rgb2rust_output::runtime::SCREEN_H as u32,
            format: glium::texture::ClientFormat::U8U8U8,
        };
        texture.write(
            glium::Rect {
                left: 0,
                bottom: 0,
                width: rgb2rust_output::runtime::SCREEN_W as u32,
                height: rgb2rust_output::runtime::SCREEN_H as u32,
            },
            raw_image,
        );

        let target = display.draw();
        let (target_w, target_h) = target.get_dimensions();
        texture.as_surface().blit_whole_color_to(
            &target,
            &glium::BlitTarget {
                left: 0,
                bottom: target_h,
                width: target_w as i32,
                height: -(target_h as i32),
            },
            interpolation_type,
        );
        target.finish().unwrap();
    }

    fn set_window_size(window: &winit::window::Window, scale: u32) {
        let _ = window.request_inner_size(winit::dpi::LogicalSize::<u32>::from((
            rgb2rust_output::runtime::SCREEN_W as u32 * scale,
            rgb2rust_output::runtime::SCREEN_H as u32 * scale,
        )));
    }
}
