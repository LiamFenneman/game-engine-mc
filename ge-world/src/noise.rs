use crate::util::{lerp, smoothstep2};
use cgmath::{vec2, Vector2};
use rand::{seq::SliceRandom, Rng, SeedableRng};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct NoiseField {
    noises: Vec<Noise>,
    seed: u64,
}

impl NoiseField {
    #[must_use]
    pub fn new(
        seed: u64,
        octaves: u8,
        frequency: f64,
        amplitude: f64,
        lacunarity: f64,
        gain: f64,
    ) -> Self {
        let noises: Vec<Noise> = (0..octaves)
            .map(|i| {
                return Noise::new(
                    seed,
                    frequency * lacunarity.powi(i32::from(i)),
                    amplitude * gain.powi(i32::from(i)),
                );
            })
            .collect();

        return Self { noises, seed };
    }

    #[must_use]
    pub fn sample_1d(&self, position: f64, offset: Option<f64>, scale: Option<f64>) -> f64 {
        let scale = scale.unwrap_or(1.0);
        return self
            .noises
            .iter()
            .map(|noise| return noise.sample_1d(position / scale, offset))
            .sum::<f64>();
    }

    #[must_use]
    pub fn sample_2d(
        &self,
        position: Vector2<f64>,
        offset: Option<Vector2<f64>>,
        scale: Option<Vector2<f64>>,
    ) -> f64 {
        let scale = scale.unwrap_or(vec2(1.0, 1.0));
        return self
            .noises
            .iter()
            .map(|noise| {
                return noise.sample_2d(vec2(position.x / scale.x, position.y / scale.y), offset);
            })
            .sum::<f64>();
    }

    /// Get the seed used to generate the noise.
    #[must_use]
    pub fn seed(&self) -> u64 {
        return self.seed;
    }
}

impl Default for NoiseField {
    fn default() -> Self {
        return NoiseField::new(0, 5, 1.0, 0.5, 2.0, 0.5);
    }
}

/// A perlin noise generator.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Noise {
    frequency: f64,
    amplitude: f64,
    random_floats: Vec<f64>,
    mask: usize,
    permutation_table: Vec<usize>,
}

impl Noise {
    /// Create a new noise generator.
    #[must_use]
    pub fn new(seed: u64, frequency: f64, amplitude: f64) -> Self {
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
        let mut random_floats = Vec::with_capacity(256);
        let mut permutation_table = Vec::with_capacity(512);
        for k in 0..256 {
            random_floats.push(rng.gen_range(-1f64..=1f64));
            permutation_table.push(k);
        }
        let mask = random_floats.len() - 1;

        permutation_table.shuffle(&mut rng);
        for k in 0..256 {
            permutation_table.push(k);
        }

        return Self {
            frequency,
            amplitude,
            random_floats,
            mask,
            permutation_table,
        };
    }

    /// Sample the noise at a given value, using the default smooth function.
    #[must_use]
    pub fn sample_1d(&self, v: f64, offset: Option<f64>) -> f64 {
        let offset = offset.unwrap_or(0.0);
        return self.sample_1d_with_fn(v + offset, smoothstep2);
    }

    /// Sample the noise at a given value, using a custom smooth function.
    #[must_use]
    fn sample_1d_with_fn<F>(&self, v: f64, smooth_fn: F) -> f64
    where
        F: Fn(f64) -> f64,
    {
        let v = v * self.frequency;
        let i = v.floor() as isize;

        // get the frational part of v, if it is negative, then add 1
        // so that it is always positive and in the range (0..=1)
        let mut f = v.fract();
        if f < 0.0 {
            f += 1.0;
        }

        let t = smooth_fn(f);

        let min = i as usize & self.mask;
        let max = (min + 1) & self.mask;
        let out = lerp(self.random_floats[min], self.random_floats[max], t);
        return out * self.amplitude;
    }

    #[must_use]
    pub fn sample_2d(&self, v: Vector2<f64>, offset: Option<Vector2<f64>>) -> f64 {
        let offset = offset.unwrap_or(Vector2::new(0.0, 0.0));
        return self.sample_2d_with_fn(v + offset, smoothstep2);
    }

    fn sample_2d_with_fn<F>(&self, v: Vector2<f64>, smooth_fn: F) -> f64
    where
        F: Fn(f64) -> f64,
    {
        let v = v * self.frequency;

        let ix = v.x.floor() as isize;
        let iy = v.y.floor() as isize;
        let fx = v.x.fract();
        let fy = v.y.fract();

        let rx0 = ix as usize & self.mask;
        let rx1 = (rx0 + 1) & self.mask;
        let ry0 = iy as usize & self.mask;
        let ry1 = (ry0 + 1) & self.mask;

        let c00 = self.random_floats[self.permutation_table[self.permutation_table[rx0] + ry0]];
        let c10 = self.random_floats[self.permutation_table[self.permutation_table[rx1] + ry0]];
        let c01 = self.random_floats[self.permutation_table[self.permutation_table[rx0] + ry1]];
        let c11 = self.random_floats[self.permutation_table[self.permutation_table[rx1] + ry1]];

        let sx = smooth_fn(fx);
        let sy = smooth_fn(fy);

        let nx0 = lerp(c00, c10, sx);
        let nx1 = lerp(c01, c11, sx);

        return lerp(nx0, nx1, sy) * self.amplitude;
    }
}
