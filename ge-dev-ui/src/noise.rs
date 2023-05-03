use cgmath::Vector2;
use egui::{ColorImage, TextureHandle};
use ge_world::noise::NoiseField;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Noise2D {
    pub is_open: bool,

    seed: u64,
    octaves: u8,
    frequency: f64,
    amplitude: f64,
    lacunarity: f64,
    gain: f64,
    offset: Vector2<f64>,
    scale: f64,

    #[serde(skip)]
    noise_field: Option<NoiseField>,
    #[serde(skip)]
    image: Option<ColorImage>,
    #[serde(skip)]
    texture: Option<TextureHandle>,
}

impl Noise2D {
    pub fn generate_noise_field(&mut self) -> NoiseField {
        tracing::debug!("Generated noise field");
        return NoiseField::new(
            self.seed,
            self.octaves,
            self.frequency,
            self.amplitude,
            self.lacunarity,
            self.gain,
        );
    }

    pub fn generate_image(&mut self) -> ColorImage {
        const SIZE: usize = 256;

        debug_assert!(self.noise_field.is_some());

        let mut samples = Vec::with_capacity(SIZE * SIZE);
        for y in 0..SIZE {
            for x in 0..SIZE {
                samples.push(self.noise_field.as_ref().unwrap().sample_2d(
                    Vector2::new(x as f64, y as f64),
                    Some(self.offset),
                    Some(self.scale),
                ));
            }
        }

        let buffer = samples
            .iter()
            .flat_map(|s| {
                let r = ((s + 1.0) / 2.0 * 255.0) as u8;
                return [r, r, r];
            })
            .collect::<Vec<_>>();

        tracing::debug!("Generated noise image");
        return ColorImage::from_rgb([256, 256], &buffer);
    }
}

impl Default for Noise2D {
    fn default() -> Self {
        let mut s = Self {
            is_open: false,

            seed: 0,
            octaves: 5,
            frequency: 1.0,
            amplitude: 0.5,
            lacunarity: 2.0,
            gain: 0.5,
            offset: Vector2::new(0.0, 0.0),
            scale: 256.0,

            noise_field: None,
            image: None,
            texture: None,
        };

        s.noise_field = Some(s.generate_noise_field());
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
            let r_octa = ui.add(
                egui::Slider::new(&mut self.octaves, 1..=10)
                    .step_by(1.0)
                    .text("Octaves"),
            );
            let r_freq = ui.add(
                egui::Slider::new(&mut self.frequency, 0.1..=10.0)
                    .step_by(0.1)
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
                egui::Slider::new(&mut self.gain, 0.1..=1.0)
                    .step_by(0.05)
                    .text("Gain"),
            );
            let r_offx = ui.add(
                egui::Slider::new(&mut self.offset.x, 0.0..=1000.0)
                    .step_by(1.0)
                    .text("Offset X"),
            );
            let r_offy = ui.add(
                egui::Slider::new(&mut self.offset.y, 0.0..=1000.0)
                    .step_by(1.0)
                    .text("Offset Y"),
            );
            let r_scal = ui.add(
                egui::Slider::new(&mut self.scale, 1.0..=1000.0)
                    .step_by(1.0)
                    .text("Scale"),
            );

            if r_octa.changed()
                || r_freq.changed()
                || r_ampl.changed()
                || r_lacu.changed()
                || r_gain.changed()
                || r_offx.changed()
                || r_offy.changed()
                || r_scal.changed()
            {
                self.noise_field = Some(self.generate_noise_field());
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
