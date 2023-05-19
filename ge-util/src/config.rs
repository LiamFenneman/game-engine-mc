use std::time::Duration;

#[derive(Default, Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct EngineConfig {
    pub renderer: RendererConfig,
    pub camera: CameraConfig,
    pub world_gen: WorldGenConfig,
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct RendererConfig {
    pub target_fps: u32,
    pub wireframe_mode: bool,
    pub render_distance: u32,
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct CameraConfig {
    pub initial_position: [f32; 3],
    pub initial_yaw_pitch: [f32; 2],
    pub znear_zfar: [f32; 2],
    pub vertical_fov: f32,
    pub speed: f32,
    pub sensitivity: f32,
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct WorldGenConfig {
    pub base_height: i32,
    pub sea_level: i32,
    pub culling: bool,
    pub cull_border: bool,
    pub noise: NoiseConfig,
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct NoiseConfig {
    pub octaves: usize,
    pub frequency: f32,
    pub amplitude: f32,
    pub lacunarity: f32,
    pub persistence: f32,
}

impl RendererConfig {
    pub fn target_frame_time(self) -> Duration {
        return Duration::from_micros(1_000_000 / u64::from(self.target_fps));
    }

    pub fn polygon_mode(self) -> wgpu::PolygonMode {
        return if self.wireframe_mode {
            wgpu::PolygonMode::Line
        } else {
            wgpu::PolygonMode::Fill
        };
    }
}

impl Default for RendererConfig {
    fn default() -> Self {
        return Self {
            target_fps: 60,
            wireframe_mode: false,
            render_distance: 3,
        };
    }
}

impl Default for CameraConfig {
    fn default() -> Self {
        return Self {
            initial_position: [0.0, 15.0, 105.0],
            initial_yaw_pitch: [0.0, 15.0],
            znear_zfar: [0.1, 3000.0],
            vertical_fov: 60.0,
            speed: 5.0,
            sensitivity: 0.5,
        };
    }
}

impl Default for WorldGenConfig {
    fn default() -> Self {
        return Self {
            base_height: 100,
            sea_level: 90,
            culling: true,
            cull_border: false,
            noise: Default::default(),
        };
    }
}

impl Default for NoiseConfig {
    fn default() -> Self {
        return Self {
            octaves: 1,
            frequency: 1.0,
            amplitude: 1.0,
            lacunarity: 2.0,
            persistence: 0.5,
        };
    }
}
