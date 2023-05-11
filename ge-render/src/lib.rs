#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::similar_names)]
#![allow(clippy::default_trait_access)]
//
#![feature(lint_reasons)]

pub mod block;
pub mod camera;
pub mod drawables;
pub mod engine;
pub mod renderer;
pub mod stats;
pub mod text;
pub mod world;

use winit::{
    event::{DeviceEvent, ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{CursorGrabMode, WindowBuilder},
};

/// The `main` function is the entry point of the game.
///
/// # Panics
/// Possible causes of panic include denied permission, incompatible system, and lack of memory.
pub async fn run() {
    setup_logging();

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_cursor_visible(false);
    window
        .set_cursor_grab(CursorGrabMode::Confined)
        .or_else(|_e| return window.set_cursor_grab(CursorGrabMode::Locked))
        .expect("could not grab cursor");
    tracing::trace!("created window");

    let renderer = renderer::Renderer::new(&window, window.inner_size()).await;
    let mut engine = engine::Engine::new(window, renderer);

    event_loop.run(move |event, _, control_flow| match event {
        Event::MainEventsCleared => {
            engine.update();
            match engine.render() {
                Ok(_) => {}
                Err(wgpu::SurfaceError::Lost) => engine.resize(engine.renderer.size),
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                Err(e) => tracing::error!("{:?}", e),
            }
        }
        Event::DeviceEvent {
            event: DeviceEvent::MouseMotion { delta },
            ..
        } => engine.camera_controller.process_mouse(delta.0, delta.1),
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == engine.window.id() && !engine.input(event) => match event {
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

fn setup_logging() {
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_max_level(tracing::Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}
