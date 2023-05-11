pub mod controller;
pub mod projection;
pub mod uniform;

use cgmath::{InnerSpace, Matrix4, Point3, Rad, Vector3};

#[derive(Debug)]
pub struct Camera {
    pub position: Point3<f32>,
    yaw: Rad<f32>,
    pitch: Rad<f32>,
}

impl Camera {
    pub fn new<V: Into<Point3<f32>>, Y: Into<Rad<f32>>, P: Into<Rad<f32>>>(
        position: V,
        yaw: Y,
        pitch: P,
    ) -> Self {
        return Self {
            position: position.into(),
            yaw: yaw.into(),
            pitch: pitch.into(),
        };
    }

    #[must_use]
    pub fn calc_matrix(&self) -> Matrix4<f32> {
        let (sin_pitch, cos_pitch) = self.pitch.0.sin_cos();
        let (sin_yaw, cos_yaw) = self.yaw.0.sin_cos();

        return Matrix4::look_to_lh(
            self.position,
            Vector3::new(-cos_pitch * cos_yaw, -cos_pitch * sin_yaw, sin_pitch).normalize(),
            Vector3::unit_z(),
        );
    }
}

impl std::fmt::Display for Camera {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(
            f,
            "P: {:?} Y: {:?} P: {:?}",
            self.position, self.yaw, self.pitch
        );
    }
}
