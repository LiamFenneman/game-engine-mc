pub mod controller;
pub mod projection;
pub mod uniform;

use crate::text::DrawText;
use nalgebra::{Matrix4, Vector3};
use nalgebra_glm::look_at_lh;

#[derive(Debug)]
pub struct Camera {
    pub position: Vector3<f32>,
    yaw: f32,
    pitch: f32,
}

impl Camera {
    pub fn new(position: impl Into<Vector3<f32>>, yaw: f32, pitch: f32) -> Self {
        return Self {
            position: position.into(),
            yaw,
            pitch,
        };
    }

    #[must_use]
    pub fn calc_matrix(&self) -> Matrix4<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.sin_cos();

        let look_dir =
            &Vector3::new(cos_pitch * cos_yaw, cos_pitch * sin_yaw, -sin_pitch).normalize();
        let target = self.position - look_dir;

        return look_at_lh(&self.position, &target, &Vector3::z_axis());
    }
}

impl DrawText for Camera {
    #[inline]
    fn name(&self) -> &'static str {
        return "camera";
    }

    #[inline]
    fn priority(&self) -> u8 {
        return 200;
    }

    #[inline]
    fn text(&self) -> String {
        return format!(
            "P: {:?} Y: {:?} P: {:?}",
            self.position, self.yaw, self.pitch
        );
    }
}
