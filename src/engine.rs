use crate::{camera::Camera, renderer::Renderer};
use std::time::Duration;
use winit::{
    event::{DeviceEvent, KeyboardInput, WindowEvent},
    window::Window,
};

/// The `Engine` struct is the main entry point for the game engine.
pub struct Engine {
    pub window: Window,
    pub renderer: Renderer,
    pub camera: Camera,
}

impl Engine {
    pub fn new(window: Window, renderer: Renderer) -> Self {
        let camera = Camera::new(
            &renderer,
            (0.0, 5.0, 10.0),
            cgmath::Deg(-90.0),
            cgmath::Deg(-20.0),
        );

        return Self {
            window,
            renderer,
            camera,
        };
    }

    pub fn update(&mut self, dt: Duration) {
        self.camera.update(&self.renderer.queue, dt);
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        return self.renderer.render(&self.camera);
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.renderer.resize(new_size);
        self.camera.resize(new_size);
    }

    pub fn input(&mut self, event: &DeviceEvent) {
        match event {
            DeviceEvent::MouseMotion { delta } => {
                self.camera.controller.process_mouse(delta.0, delta.1)
            }
            _ => {}
        }
    }

    pub fn input_keyboard(&mut self, event: &WindowEvent) -> bool {
        match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        virtual_keycode: Some(key),
                        state,
                        ..
                    },
                ..
            } => self.camera.controller.process_keyboard(*key, *state),
            _ => false,
        }
    }
}
