use crate::camera::Camera;
use ge_util::deg_to_rad;
use nalgebra::Vector3;
use std::f32::consts::FRAC_PI_2;
use winit::event::{ElementState, VirtualKeyCode};

const SAFE_FRAC_PI_2: f32 = FRAC_PI_2 - 0.0001;

#[derive(Debug, Clone, Copy)]
pub struct CameraController {
    amount_left: f32,
    amount_right: f32,
    amount_forward: f32,
    amount_backward: f32,
    amount_up: f32,
    amount_down: f32,
    rotate_horizontal: f32,
    rotate_vertical: f32,
    speed: f32,
    sensitivity: f32,
    aspect: f32,
}

impl CameraController {
    #[allow(clippy::cast_precision_loss, reason = "used for aspect ratio")]
    #[must_use]
    pub fn new(speed: f32, sensitivity: f32, width: u32, height: u32) -> Self {
        return Self {
            amount_left: 0.0,
            amount_right: 0.0,
            amount_forward: 0.0,
            amount_backward: 0.0,
            amount_up: 0.0,
            amount_down: 0.0,
            rotate_horizontal: 0.0,
            rotate_vertical: 0.0,
            speed,
            sensitivity,
            aspect: width as f32 / height as f32,
        };
    }

    pub fn process_keyboard(&mut self, key: VirtualKeyCode, state: ElementState) -> bool {
        let amount = if state == ElementState::Pressed {
            1.0
        } else {
            0.0
        };
        return match key {
            VirtualKeyCode::W | VirtualKeyCode::Up => {
                self.amount_forward = amount;
                true
            }
            VirtualKeyCode::S | VirtualKeyCode::Down => {
                self.amount_backward = amount;
                true
            }
            VirtualKeyCode::A | VirtualKeyCode::Left => {
                self.amount_left = amount;
                true
            }
            VirtualKeyCode::D | VirtualKeyCode::Right => {
                self.amount_right = amount;
                true
            }
            VirtualKeyCode::Space => {
                self.amount_up = amount;
                true
            }
            VirtualKeyCode::LShift => {
                self.amount_down = amount;
                true
            }
            _ => false,
        };
    }

    #[allow(
        clippy::cast_possible_truncation,
        reason = "needs standardisation: see #2"
    )]
    pub fn process_mouse(&mut self, dx: f64, dy: f64) {
        self.rotate_horizontal = dx as f32;
        self.rotate_vertical = dy as f32;
    }

    pub fn update_camera(&mut self, camera: &mut Camera, dt: f64) {
        #[allow(
            clippy::cast_possible_truncation,
            reason = "needs standardisation: see #2"
        )]
        let dt = dt as f32;

        // Move forward/backward and left/right
        let (yaw_sin, yaw_cos) = camera.yaw.sin_cos();
        let forward = Vector3::new(yaw_cos, yaw_sin, 0.0).normalize();
        let right = Vector3::new(yaw_sin, -yaw_cos, 0.0).normalize();
        // let forward = Vector3::new(yaw_sin, yaw_cos, 0.0).normalize();
        // let right = Vector3::new(-yaw_cos, yaw_sin, 0.0).normalize();
        camera.position += forward * (self.amount_forward - self.amount_backward) * self.speed * dt;
        camera.position += right * (self.amount_right - self.amount_left) * self.speed * dt;

        // Move up/down. Since we don't use roll, we can just
        // modify the y coordinate directly.
        camera.position.z += (self.amount_up - self.amount_down) * self.speed * dt;

        // Rotate
        camera.yaw += deg_to_rad(-self.rotate_horizontal) * self.sensitivity / self.aspect * dt;
        camera.pitch += deg_to_rad(self.rotate_vertical) * self.sensitivity * dt;

        // If process_mouse isn't called every frame, these values
        // will not get set to zero, and the camera will rotate
        // when moving in a non cardinal direction.
        self.rotate_horizontal = 0.0;
        self.rotate_vertical = 0.0;

        // Keep the camera's angle from going too high/low.
        if camera.pitch < -SAFE_FRAC_PI_2 {
            camera.pitch = -SAFE_FRAC_PI_2;
        } else if camera.pitch > SAFE_FRAC_PI_2 {
            camera.pitch = SAFE_FRAC_PI_2;
        }

        camera.yaw = camera.yaw.rem_euclid(2.0 * std::f32::consts::PI);
    }
}
