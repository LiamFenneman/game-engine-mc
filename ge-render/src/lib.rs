#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::default_trait_access)]

pub mod block;
pub mod camera;
pub mod engine;
pub mod renderer;
pub mod stats;
pub mod texture;

use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, CursorGrabMode},
};

/// The `main` function is the entry point of the game.
///
/// # Panics
/// Possible causes of panic include denied permission, incompatible system, and lack of memory.
pub async fn run() {
    let file_appender = tracing_appender::rolling::hourly("logs", "engine.log");
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_writer(file_appender)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(false)
        .with_target(false)
        .with_max_level(tracing::Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_cursor_visible(false);
    window
        .set_cursor_grab(CursorGrabMode::Confined)
        .or_else(|_e| return window.set_cursor_grab(CursorGrabMode::Locked))
        .unwrap();

    let renderer = renderer::Renderer::new(&window, window.inner_size()).await;
    let mut engine = engine::Engine::new(window, renderer);

    let block = block::DrawBlock::new(
        &engine.renderer,
        block::Block::new(),
        &engine.camera.uniform_bind_group_layout,
    );
    engine.renderer.add_drawable(Box::new(block));

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(window_id) if window_id == engine.window.id() => {
            engine.update();
            match engine.render() {
                Ok(_) => {}
                // Reconfigure the surface if lost
                Err(wgpu::SurfaceError::Lost) => engine.resize(engine.renderer.size),
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => tracing::error!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            // RedrawRequested will only trigger once, unless we manually
            // request it.
            engine.window.request_redraw();
        }
        Event::DeviceEvent { ref event, .. } => {
            engine.input(event);
        }
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == engine.window.id() && !engine.input_keyboard(event) => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => *control_flow = ControlFlow::Exit,
            WindowEvent::Resized(physical_size) => {
                engine.resize(*physical_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                engine.resize(**new_inner_size);
            }
            _ => {}
        },
        _ => {}
    });
}
