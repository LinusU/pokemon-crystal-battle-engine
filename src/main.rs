use std::sync::mpsc::{self, Sender, SyncSender};
use std::thread;
use winit::platform::pump_events::EventLoopExtPumpEvents;

use rgb2rust_output::runtime::{KeypadEvent, KeypadKey};

#[derive(Default)]
struct RenderOptions {
    linear_interpolation: bool,
}

fn main() {
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
    let cpu_thread = thread::spawn(move || run_game(sender, keypad_receiver));

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

        match receiver.recv() {
            Ok(data) => recalculate_screen(&display, &mut texture, &data, &render_options),
            Err(_) => break,
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
) {
    let routine = rgb2rust_output::entry_routine();
    let mut cpu = rgb2rust_output::runtime::Cpu::new_cgb(Some(update_screen), Some(keypad_events));
    cpu.interrupt_vectors = rgb2rust_output::interrupt_vectors();
    (routine.func)(&mut cpu);
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
