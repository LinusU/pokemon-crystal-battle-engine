use std::sync::mpsc::{self, Sender, SyncSender};
use std::thread;
use winit::platform::pump_events::EventLoopExtPumpEvents;

use rgb2rust_output::bootstrap::{self, BattleConfig, RunOptions};
use rgb2rust_output::runtime::{KeypadEvent, KeypadKey};

#[derive(Default)]
struct RenderOptions {
    linear_interpolation: bool,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let cli_args = match parse_cli_args(&args[1..]) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("error: {e}");
            eprintln!(
                "usage: {} [<scenario.toml> | --random] [--seed <n>] [--record <out.toml>] [--play <recording.toml>]",
                args[0],
            );
            std::process::exit(1);
        }
    };

    // `--play` recovers the matchup from the recording: a generated (`--random`)
    // run carries an embedded `[scenario.*]` block we replay directly, while a
    // scenario-file run carries a `source_scenario` path we reload (or the
    // positional override). The recording's resolved seed always wins over any
    // `seed = ...` on the scenario.
    let playback_recording = cli_args.play.as_deref().map(|p| match bootstrap::player::load(p) {
        Ok(rec) => rec,
        Err(e) => {
            eprintln!("error: {e}");
            std::process::exit(1);
        }
    });

    // Resolve the battle config and, for recordings, the bookkeeping the header
    // needs: `embed_scenario` (write the resolved matchup into the recording)
    // and `source_scenario` (the file path it was driven from, for documentation
    // only). Generated battles embed; file-driven battles record the path.
    let mut embed_scenario = false;
    let mut source_scenario: Option<String> = None;
    let battle_config: Option<BattleConfig> = if cli_args.random {
        // Live `--random` run: pick a base seed (CLI > entropy) and generate the
        // matchup. The base seed splits into independent generation and engine
        // seeds inside `random_battle_config` — the engine seed lands in
        // `BattleHeader.seed`. Recording embeds the resolved matchup so playback
        // never re-runs the generator.
        let base_seed = cli_args.seed.unwrap_or_else(random_entropy_seed);
        embed_scenario = true;
        Some(bootstrap::random::random_battle_config(base_seed))
    } else if let Some(scenario) = playback_recording.as_ref().and_then(|r| r.scenario.as_ref()) {
        // Self-contained recording (e.g. a recorded `--random` run): replay the
        // embedded matchup verbatim. `seed_override` below pins the engine seed.
        Some(scenario.clone())
    } else {
        // A scenario-file run, replayed or live. Resolve the path from the
        // positional arg, else the recording's `source_scenario`.
        let path = cli_args.scenario.clone().or_else(|| {
            playback_recording
                .as_ref()
                .and_then(|rec| rec.source_scenario.as_ref())
                .map(|p| p.to_string_lossy().into_owned())
        });
        match path {
            Some(path) => match bootstrap::load_battle(std::path::Path::new(&path)) {
                Ok(cfg) => {
                    source_scenario = Some(path);
                    Some(cfg)
                }
                Err(e) => {
                    eprintln!("error: {e}");
                    std::process::exit(1);
                }
            },
            None => {
                if playback_recording.is_some() {
                    eprintln!(
                        "error: --play recording has neither an embedded scenario nor a `source_scenario` — pass a scenario.toml positionally"
                    );
                    std::process::exit(1);
                }
                None
            }
        }
    };
    let record_path = cli_args.record;
    let playback_recording = playback_recording;

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
            source_scenario,
            embed_scenario,
            playback_recording,
        )
    });

    event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);
    loop {
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

fn create_window_builder() -> winit::window::WindowBuilder {
    winit::window::WindowBuilder::new().with_title("rgb2rust")
}

fn run_game(
    update_screen: SyncSender<Vec<u8>>,
    keypad_events: mpsc::Receiver<KeypadEvent>,
    battle_config: Option<BattleConfig>,
    record_path: Option<std::path::PathBuf>,
    source_scenario: Option<String>,
    embed_scenario: bool,
    playback: Option<rgb2rust_output::bootstrap::player::Recording>,
) {
    let mut cpu = rgb2rust_output::runtime::Cpu::new_cgb(Some(update_screen), Some(keypad_events));
    cpu.interrupt_vectors = rgb2rust_output::interrupt_vectors();

    match battle_config {
        Some(config) => {
            let source_scenario_path = source_scenario.as_ref().map(std::path::Path::new);
            let (seed_override, playback_steps) = match playback {
                Some(rec) => (Some(rec.seed), Some(rec.steps)),
                None => (None, None),
            };
            let options = RunOptions {
                record_path: record_path.as_deref(),
                source_scenario: source_scenario_path,
                embed_scenario,
                seed_override,
                playback: playback_steps,
            };
            if let Err(e) = bootstrap::run_with(&mut cpu, &config, options) {
                eprintln!("battle bootstrap failed: {e}");
            }
        }
        None => {
            if record_path.is_some() {
                eprintln!("warning: --record requires a scenario TOML; ignoring");
            }
            let routine = rgb2rust_output::entry_routine();
            (routine.func)(&mut cpu);
        }
    }
}

/// Argv parser for `[<scenario.toml> | --random] [--seed <n>] [--record <out.toml>] [--play <rec.toml>]`.
/// The first non-flag positional is the scenario path. Flag order is
/// irrelevant. `--play` may be passed with or without a positional scenario —
/// when absent, the matchup is taken from the recording's embedded
/// `[scenario.*]` block, falling back to its `source_scenario` header path.
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
}

fn parse_cli_args(args: &[String]) -> Result<CliArgs, String> {
    let mut scenario: Option<String> = None;
    let mut record: Option<std::path::PathBuf> = None;
    let mut play: Option<std::path::PathBuf> = None;
    let mut random = false;
    let mut seed: Option<u64> = None;
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
    Ok(CliArgs { scenario, record, play, random, seed })
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
