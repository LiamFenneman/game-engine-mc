use crate::{context::Context, world::WorldState};
use ge_resource::texture::Texture;
use ge_util::EngineConfig;
use std::sync::Arc;
use winit::window::Window;

/// The `Draw` trait is implemented by types that can be drawn by the `Renderer`.
pub trait Draw {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, uniforms: &'a wgpu::BindGroup);
}

/// The `Vertex` trait is implemented by types that can be used as vertices in a mesh.
pub trait Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a>;
}

/// The `Renderer` struct is responsible for rendering the game.
#[derive(Debug)]
pub(crate) struct Renderer {
    surface: wgpu::Surface,
    pub config: wgpu::SurfaceConfiguration,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub size: winit::dpi::PhysicalSize<u32>,
    pub depth_texture: Texture,
    world: Option<WorldState>,

    pub staging_belt: wgpu::util::StagingBelt,
    pub debug_text: crate::text::TextRenderer,
}

impl Renderer {
    #[allow(clippy::implicit_return)]
    fn features() -> wgpu::Features {
        use wgpu::Features;
        Features::TEXTURE_BINDING_ARRAY
            | Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING
            | Features::POLYGON_MODE_LINE
    }

    pub async fn new(window: &Window, size: winit::dpi::PhysicalSize<u32>) -> Self {
        // the instance is a handle to our GPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::VULKAN,
            dx12_shader_compiler: Default::default(),
        });

        // # Safety
        //
        // The surface needs to live as long as the window that created it. Engine owns the window,
        // and Engine lifes for the lifetime of the game, so this should be safe.
        let surface =
            unsafe { instance.create_surface(&window) }.expect("Failed to create surface");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to find an appropriate adapter");

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: Self::features(),
                    limits: wgpu::Limits::default(),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        let surface_caps = surface.get_capabilities(&adapter);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_caps.formats[0],
            width: size.width,
            height: size.height,
            // present_mode: surface_caps.present_modes[0],
            present_mode: wgpu::PresentMode::AutoNoVsync,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);

        let depth_texture = Texture::create_depth_texture(&device, &config, "depth_texture");

        let staging_belt = wgpu::util::StagingBelt::new(1024);
        let debug_text = crate::text::TextRenderer::new(
            "Debug Info",
            (config.width, config.height),
            &device,
            surface_caps.formats[0],
        );

        trace!("created renderer");
        return Self {
            surface,
            config,
            device,
            queue,
            size,
            depth_texture,
            world: None,

            staging_belt,
            debug_text,
        };
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.config.width = new_size.width;
        self.config.height = new_size.height;
        self.surface.configure(&self.device, &self.config);
        self.depth_texture =
            Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
    }

    /// Renders the scene to the screen.
    ///
    /// # Errors
    /// Errors if the surface is lost. Which should never happen.
    pub fn render(&mut self, cx: Context) -> Result<(), wgpu::SurfaceError> {
        let begin_time = std::time::Instant::now();

        // get the frame to draw to
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        let world = self.world.as_ref().unwrap().lock().unwrap();
        let cx = cx.lock();

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            // render the world
            world.draw(&mut render_pass, &cx.uniform_bind_group);
        }

        // render text
        self.debug_text
            .draw(&self.device, &mut self.staging_belt, &mut encoder, &view);

        // limit the FPS to the target FPS
        let frame_time = begin_time.elapsed();
        if frame_time < cx.config.renderer.target_frame_time() {
            std::thread::sleep(cx.config.renderer.target_frame_time() - frame_time);
        }

        // render finished, submit to the queue
        self.staging_belt.finish();
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
        return Ok(());
    }

    pub fn set_world(&mut self, world: &WorldState) {
        self.world = Some(Arc::clone(world));
    }
}

pub(crate) fn create_render_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    format: wgpu::TextureFormat,
    depth_format: Option<wgpu::TextureFormat>,
    vertex_layouts: &[wgpu::VertexBufferLayout],
    shader: wgpu::ShaderModuleDescriptor,
    config: &EngineConfig,
) -> wgpu::RenderPipeline {
    let shader = device.create_shader_module(shader);

    return device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main",
            buffers: vertex_layouts,
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: Some(wgpu::BlendState {
                    alpha: wgpu::BlendComponent::REPLACE,
                    color: wgpu::BlendComponent::REPLACE,
                }),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: config.renderer.polygon_mode(),
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: depth_format.map(|format| {
            return wgpu::DepthStencilState {
                format,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            };
        }),
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    });
}
