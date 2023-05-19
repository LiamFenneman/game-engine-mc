use crate::{
    camera::{
        controller::CameraController, projection::Projection, uniform::CameraUniform, Camera,
    },
    drawables::world::World,
    renderer::Renderer,
    stats::FrameStats,
};
use ge_resource::ResourceManager;
use ge_util::EngineConfig;
use std::{cell::RefCell, rc::Rc};
use wgpu::util::DeviceExt;
use winit::{
    event::{KeyboardInput, WindowEvent},
    window::Window,
};

/// The `Engine` struct is the main entry point for the game engine.
pub struct Engine {
    pub config: EngineConfig,

    pub window: Window,
    pub renderer: Renderer,
    pub resources: ResourceManager,

    pub world: Rc<RefCell<World>>,
    pub camera: Camera,
    pub projection: Projection,
    pub camera_controller: CameraController,

    pub camera_uniform: CameraUniform,
    pub uniform_buffer: wgpu::Buffer,
    pub uniform_bind_group: wgpu::BindGroup,
    pub uniform_bind_group_layout: wgpu::BindGroupLayout,

    pub stats: FrameStats,
}

impl Engine {
    pub fn new(window: Window, mut renderer: Renderer) -> Self {
        let resources = ResourceManager::default();
        let config: EngineConfig = resources.load_config("engine.toml").unwrap_or_default();

        let camera = Camera::new(
            config.camera.initial_position,
            cgmath::Deg(config.camera.initial_yaw_pitch[0]),
            cgmath::Deg(config.camera.initial_yaw_pitch[1]),
        );
        let projection = Projection::new(
            renderer.config.width,
            renderer.config.height,
            cgmath::Deg(config.camera.vertical_fov),
            config.camera.znear_zfar[0],
            config.camera.znear_zfar[1],
        );
        let camera_controller =
            CameraController::new(config.camera.speed, config.camera.sensitivity);

        let world = Rc::new(RefCell::new(World::new(cgmath::vec2(0, 0), &config)));
        renderer.set_world(Rc::clone(&world));

        let uniform_bind_group_layout =
            renderer
                .device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                    label: Some("uniform_bind_group_layout"),
                });

        let mut camera_uniform = CameraUniform::new();
        camera_uniform.update_view_proj(
            camera.position,
            camera.calc_matrix(),
            projection.calc_matrix(),
        );

        let uniform_buffer =
            renderer
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Uniform Buffer"),
                    contents: bytemuck::cast_slice(&[camera_uniform]),
                    usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                });

        let uniform_bind_group = renderer
            .device
            .create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &uniform_bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer.as_entire_binding(),
                }],
                label: Some("uniform_bind_group"),
            });

        let stats = FrameStats::default();

        tracing::trace!("created engine");
        return Self {
            config,

            window,
            renderer,
            resources,

            world,
            camera,
            projection,
            camera_controller,

            camera_uniform,
            uniform_buffer,
            uniform_bind_group,
            uniform_bind_group_layout,

            stats,
        };
    }

    pub fn update(&mut self) {
        self.stats.fps();
        self.renderer.debug_text.add_entry(
            "fps",
            250,
            format!(
                "FPS {} DT {}",
                self.stats.current_fps, self.stats.delta_time
            ),
        );
        self.renderer
            .debug_text
            .add_entry("camera", 200, format!("{}", self.camera));
        self.camera_controller
            .update_camera(&mut self.camera, self.stats.delta_time);
        self.camera_uniform.update_view_proj(
            self.camera.position,
            self.camera.calc_matrix(),
            self.projection.calc_matrix(),
        );
        self.renderer.queue.write_buffer(
            &self.uniform_buffer,
            0,
            bytemuck::cast_slice(&[self.camera_uniform]),
        );
        self.world.borrow_mut().update(
            self.camera.position,
            &self.renderer,
            &mut self.resources,
            &self.uniform_bind_group_layout,
            &self.config,
        );
    }

    /// Renders the game.
    ///
    /// # Errors
    /// Errors if the surface is lost.
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        return self.renderer.render(&self.uniform_bind_group, &self.config);
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.projection.resize(new_size.width, new_size.height);
            self.renderer.resize(new_size);
            self.renderer.debug_text.resize(new_size);
        }
    }

    pub fn input(&mut self, event: &WindowEvent) -> bool {
        return match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(key),
                        state,
                        ..
                    },
                ..
            } => self.camera_controller.process_keyboard(*key, *state),
            _ => false,
        };
    }
}
