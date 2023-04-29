mod camera;
mod engine;
mod renderer;
mod texture;

use std::time::Instant;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

pub async fn run() {
    let subscriber = tracing_subscriber::fmt()
        .compact()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(false)
        .with_target(false)
        .with_max_level(tracing::Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    let renderer = renderer::Renderer::new(&window, window.inner_size()).await;
    let mut engine = engine::Engine::new(window, renderer);
    let mut last_render_time = Instant::now();

    engine
        .renderer
        .add_drawable(Box::new(debug::DebugDrawable::new(&engine.renderer)));

    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(window_id) if window_id == engine.window.id() => {
            let now = Instant::now();
            let dt = now - last_render_time;
            last_render_time = now;
            engine.update(dt);
            match engine.render() {
                Ok(_) => {}
                // Reconfigure the surface if lost
                Err(wgpu::SurfaceError::Lost) => engine.resize(engine.renderer.size),
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::MainEventsCleared => {
            // RedrawRequested will only trigger once, unless we manually
            // request it.
            engine.window.request_redraw();
        }
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == engine.window.id() && !engine.input(event) => match event {
            #[cfg(not(target_arch = "wasm32"))]
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

mod debug {
    use crate::{
        renderer::{create_render_pipeline, Draw, Renderer, Vertex},
        texture::Texture,
    };
    use wgpu::util::DeviceExt;

    pub struct DebugDrawable {
        render_pipeline: wgpu::RenderPipeline,
        vertex_buffer: wgpu::Buffer,
    }
    #[repr(C)]
    #[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
    struct V {
        position: [f32; 3],
        color: [f32; 3],
    }
    impl V {
        const ATTRIBS: [wgpu::VertexAttribute; 2] =
            wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];
    }
    impl Vertex for V {
        fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
            use std::mem;

            wgpu::VertexBufferLayout {
                array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &Self::ATTRIBS,
            }
        }
    }
    impl DebugDrawable {
        #[rustfmt::skip]
        const VERTICES: &[V] = &[
        V { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
        V { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
        V { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
    ];

        pub fn new(renderer: &Renderer) -> Self {
            let layout = renderer
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Debug Pipeline Layout"),
                    bind_group_layouts: &[],
                    push_constant_ranges: &[],
                });

            let shader = wgpu::ShaderModuleDescriptor {
                label: Some("Debug Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
            };

            let render_pipeline = create_render_pipeline(
                renderer,
                &layout,
                Some(Texture::DEPTH_FORMAT),
                &[V::desc()],
                shader,
            );

            let vertex_buffer =
                renderer
                    .device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("Vertex Buffer"),
                        contents: bytemuck::cast_slice(Self::VERTICES),
                        usage: wgpu::BufferUsages::VERTEX,
                    });

            return Self {
                render_pipeline,
                vertex_buffer,
            };
        }
    }

    impl Draw for DebugDrawable {
        fn draw<'a>(
            &'a self,
            render_pass: &mut wgpu::RenderPass<'a>,
            uniforms: &'a wgpu::BindGroup,
        ) {
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(1, &uniforms, &[]);

            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
            render_pass.draw(0..3, 0..1);
        }
    }
}
