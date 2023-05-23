use nalgebra::Matrix4;
use nalgebra_glm::perspective;

/// The camera projection.
///
/// The projection only really needs to change if the window resizes.
#[derive(Debug, Clone, Copy)]
pub struct Projection {
    aspect: f32,
    fovy: f32,
    znear: f32,
    zfar: f32,
}

impl Projection {
    #[must_use]
    pub fn new(width: u32, height: u32, fovy: f32, znear: f32, zfar: f32) -> Self {
        #[allow(clippy::cast_precision_loss, reason = "values should be small")]
        return Self {
            aspect: width as f32 / height as f32,
            fovy,
            znear,
            zfar,
        };
    }

    #[allow(clippy::cast_precision_loss, reason = "values should be small")]
    pub fn resize(&mut self, width: u32, height: u32) {
        self.aspect = width as f32 / height as f32;
    }

    #[must_use]
    pub fn calc_matrix(&self) -> Matrix4<f32> {
        return perspective(self.aspect, self.fovy, self.znear, self.zfar);
    }
}
