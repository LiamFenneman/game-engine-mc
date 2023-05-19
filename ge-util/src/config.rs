use std::time::Duration;

#[derive(Default, Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct EngineConfig {
    pub renderer: RendererConfig,
    pub world_gen: WorldGenConfig,
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct RendererConfig {
    pub target_fps: u32,
    pub wireframe_mode: bool,
    pub render_distance: u32,
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct WorldGenConfig {
    pub base_height: i32,
    pub sea_level: i32,
    pub culling: bool,
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

impl Default for WorldGenConfig {
    fn default() -> Self {
        return Self {
            base_height: 100,
            sea_level: 90,
            culling: true,
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
