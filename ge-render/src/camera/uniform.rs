use cgmath::{Matrix4, Point3, SquareMatrix};

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform {
    view_position: [f32; 4],
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    #[must_use]
    pub fn new() -> Self {
        return Self {
            view_position: [0.0; 4],
            view_proj: Matrix4::identity().into(),
        };
    }

    pub fn update_view_proj(
        &mut self,
        position: Point3<f32>,
        camera_matrix: Matrix4<f32>,
        projection_matrix: Matrix4<f32>,
    ) {
        self.view_position = position.to_homogeneous().into();
        self.view_proj = (projection_matrix * camera_matrix).into();
    }
}

impl Default for CameraUniform {
    fn default() -> Self {
        return Self::new();
    }
}
