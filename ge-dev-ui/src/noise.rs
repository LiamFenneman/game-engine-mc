use cgmath::Vector2;
use egui::{
    plot::{Line, Plot, PlotPoints},
    ColorImage, TextureHandle,
};
use ge_world::noise::Noise;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Noise2D {
    pub is_open: bool,
    size: usize,

    seed: u64,
    octaves: usize,
    frequency: f32,
    amplitude: f32,
    lacunarity: f32,
    persistence: f32,
    offset: Vector2<f32>,

    #[serde(skip)]
    noise: Option<Noise>,
    #[serde(skip)]
    image: Option<ColorImage>,
    #[serde(skip)]
    texture: Option<TextureHandle>,
}

impl Noise2D {
    pub fn generate_noise_field(&mut self) -> Noise {
        tracing::debug!("Generated noise field");
        return Noise::new(
            self.octaves,
            self.frequency,
            self.amplitude,
            self.lacunarity,
            self.persistence,
        );
    }

    pub fn generate_image(&mut self) -> ColorImage {
        debug_assert!(self.noise.is_some());

        let mut samples = Vec::with_capacity(self.size * self.size);
        for y in 0..self.size {
            for x in 0..self.size {
                #[allow(
                    clippy::cast_precision_loss,
                    reason = "sample uses f64 so we need to cast"
                )]
                samples.push(self.noise.as_ref().unwrap().fbm(
                    x as f32 + self.offset.x,
                    y as f32 + self.offset.y,
                    0.0,
                ));
            }
        }

        let buffer = samples
            .iter()
            .flat_map(|s| {
                #[allow(
                    clippy::cast_possible_truncation,
                    reason = "value must be within [0, 255]"
                )]
                #[allow(clippy::cast_sign_loss, reason = "value is shifted to be positive")]
                let r = ((s + 1.0) / 2.0 * 255.0) as u8;
                return [r, r, r];
            })
            .collect::<Vec<_>>();

        tracing::debug!("Generated noise image");
        return ColorImage::from_rgb([self.size, self.size], &buffer);
    }
}

impl Default for Noise2D {
    fn default() -> Self {
        let mut s = Self {
            is_open: false,
            size: 256,

            seed: 0,
            octaves: 5,
            frequency: 16.0,
            amplitude: 0.5,
            lacunarity: 2.0,
            persistence: 0.5,
            offset: Vector2::new(0.0, 0.0),

            noise: None,
            image: None,
            texture: None,
        };

        s.noise = Some(s.generate_noise_field());
        s.image = Some(s.generate_image());
        return s;
    }
}

impl std::fmt::Display for Noise2D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "2D Noise Generator");
    }
}

impl Noise2D {
    pub fn window(&mut self, ctx: &egui::Context) {
        if !self.is_open {
            return;
        }

        egui::Window::new(format!("{self}")).show(ctx, |ui| {
            let r_seed = ui.add(
                egui::Slider::new(&mut self.seed, 0..=50)
                    .step_by(1.0)
                    .text("Seed"),
            );
            let r_size = ui.add(
                egui::Slider::new(&mut self.size, 16..=512)
                    .step_by(16.0)
                    .text("Size"),
            );

            ui.separator();

            let r_octa = ui.add(
                egui::Slider::new(&mut self.octaves, 1..=10)
                    .step_by(1.0)
                    .text("Octaves"),
            );
            let r_freq = ui.add(
                egui::Slider::new(&mut self.frequency, 1.0..=1000.0)
                    .step_by(1.0)
                    .text("Frequency"),
            );
            let r_ampl = ui.add(
                egui::Slider::new(&mut self.amplitude, 0.05..=2.0)
                    .step_by(0.05)
                    .text("Amplitude"),
            );
            let r_lacu = ui.add(
                egui::Slider::new(&mut self.lacunarity, 1.0..=10.0)
                    .step_by(0.1)
                    .text("Lacunarity"),
            );
            let r_gain = ui.add(
                egui::Slider::new(&mut self.persistence, 0.1..=1.0)
                    .step_by(0.05)
                    .text("Gain"),
            );
            let r_offx = ui.add(
                egui::Slider::new(&mut self.offset.x, -1000.0..=1000.0)
                    .step_by(1.0)
                    .text("Offset X"),
            );
            let r_offy = ui.add(
                egui::Slider::new(&mut self.offset.y, -1000.0..=1000.0)
                    .step_by(1.0)
                    .text("Offset Y"),
            );

            if r_seed.changed()
                || r_size.changed()
                || r_octa.changed()
                || r_freq.changed()
                || r_ampl.changed()
                || r_lacu.changed()
                || r_gain.changed()
                || r_offx.changed()
                || r_offy.changed()
            {
                self.noise = Some(self.generate_noise_field());
                self.image = Some(self.generate_image());
                self.texture = None;
            }

            ui.separator();

            if let Some(image) = self.image.as_ref() {
                let texture: &egui::TextureHandle = self.texture.get_or_insert_with(|| {
                    return ui.ctx().load_texture(
                        "perlin-nose-2d",
                        image.clone(),
                        Default::default(),
                    );
                });
                ui.image(texture, texture.size_vec2());
            }
        });
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Noise1D {
    pub is_open: bool,

    seed: u64,
    octaves: usize,
    frequency: f32,
    amplitude: f32,
    lacunarity: f32,
    persistence: f32,
    offset: f32,

    min: i32,
    max: i32,
    samples: u32,

    #[serde(skip)]
    noise_field: Option<Noise>,
}

impl Noise1D {
    pub fn generate_noise_field(&mut self) -> Noise {
        tracing::debug!("Generated noise field");
        return Noise::new(
            self.octaves,
            self.frequency,
            self.amplitude,
            self.lacunarity,
            self.persistence,
        );
    }

    pub fn window(&mut self, ctx: &egui::Context) {
        if !self.is_open {
            return;
        }

        egui::Window::new(format!("{self}")).show(ctx, |ui| {
            let r_seed = ui.add(
                egui::Slider::new(&mut self.seed, 0..=50)
                    .step_by(1.0)
                    .text("Seed"),
            );

            ui.separator();

            let r_octa = ui.add(
                egui::Slider::new(&mut self.octaves, 1..=10)
                    .step_by(1.0)
                    .text("Octaves"),
            );
            let r_freq = ui.add(
                egui::Slider::new(&mut self.frequency, 1.0..=100.0)
                    .step_by(1.0)
                    .text("Frequency"),
            );
            let r_ampl = ui.add(
                egui::Slider::new(&mut self.amplitude, 0.05..=2.0)
                    .step_by(0.05)
                    .text("Amplitude"),
            );
            let r_lacu = ui.add(
                egui::Slider::new(&mut self.lacunarity, 1.0..=10.0)
                    .step_by(0.1)
                    .text("Lacunarity"),
            );
            let r_gain = ui.add(
                egui::Slider::new(&mut self.persistence, 0.1..=1.0)
                    .step_by(0.05)
                    .text("Gain"),
            );
            let r_offs = ui.add(
                egui::Slider::new(&mut self.offset, -10.0..=10.0)
                    .step_by(0.1)
                    .text("Offset"),
            );

            if r_seed.changed()
                || r_octa.changed()
                || r_freq.changed()
                || r_ampl.changed()
                || r_lacu.changed()
                || r_gain.changed()
                || r_offs.changed()
            {
                self.noise_field = Some(self.generate_noise_field());
            }

            ui.separator();

            ui.add(
                egui::Slider::new(&mut self.min, -20..=0)
                    .step_by(0.1)
                    .text("Min"),
            );
            ui.add(
                egui::Slider::new(&mut self.max, 0..=20)
                    .step_by(0.1)
                    .text("Max"),
            );
            ui.add(
                egui::Slider::new(&mut self.samples, 10..=1000)
                    .step_by(10.0)
                    .text("Samples"),
            );

            ui.separator();

            if let Some(nf) = &self.noise_field {
                #[allow(
                    clippy::cast_possible_wrap,
                    clippy::cast_lossless,
                    clippy::cast_precision_loss,
                    reason = "values large enough to wrap will not be used"
                )]
                let points: PlotPoints = ((self.min * self.samples as i32)
                    ..=(self.max * self.samples as i32))
                    .map(|x| return x as f32 / self.samples as f32)
                    .map(|x| return [x as f64, nf.fbm(x + self.offset, 0.0, 0.0) as f64])
                    .collect();
                let line = Line::new(points);
                Plot::new("noise_1d").show(ui, |plot_ui| return plot_ui.line(line));
            }
        });
    }
}

impl Default for Noise1D {
    fn default() -> Self {
        let mut s = Self {
            is_open: false,

            seed: 0,
            octaves: 5,
            frequency: 16.0,
            amplitude: 1.0,
            lacunarity: 2.0,
            persistence: 0.5,
            offset: 0.0,

            min: -10,
            max: 10,
            samples: 100,

            noise_field: None,
        };

        s.noise_field = Some(s.generate_noise_field());
        return s;
    }
}

impl std::fmt::Display for Noise1D {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "1D Noise Generator");
    }
}
