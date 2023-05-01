use crate::util::{lerp, smoothstep2};
use rand::{Rng, SeedableRng};

/// A perlin noise generator.
pub struct Noise {
    seed: u64,
    frequency: f64,
    amplitude: f64,
    offset: f64,
    permutations: Vec<f64>,
    mask: usize,
}

impl Noise {
    /// Create a new noise generator.
    #[must_use]
    pub fn new(seed: u64, frequency: f64, amplitude: f64, offset: f64) -> Self {
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
        let permutations = (0..256)
            .map(|_| return rng.gen_range(0f64..=1f64))
            .collect::<Vec<_>>();
        let mask = permutations.len() - 1;

        return Self {
            seed,
            frequency,
            amplitude,
            offset,
            permutations,
            mask,
        };
    }

    /// Sample the noise at a given value, using the default smooth function.
    #[must_use]
    pub fn sample(&self, v: f64) -> f64 {
        return self.sample_with_fn(v, smoothstep2);
    }

    /// Sample the noise at a given value, using a custom smooth function.
    #[must_use]
    pub fn sample_with_fn<F>(&self, v: f64, smooth_fn: F) -> f64
    where
        F: Fn(f64) -> f64,
    {
        let v = v * self.frequency + self.offset;
        let i = v.floor() as isize;

        // get the frational part of v, if it is negative, then add 1
        // so that it is always positive and in the range (0..=1)
        let mut f = v.fract();
        if f < 0.0 {
            f += 1.0;
        }

        let t = smooth_fn(f);

        #[allow(clippy::cast_sign_loss)] // we know that i is positive
        let min = i as usize & self.mask;
        let max = (min + 1) & self.mask;
        let out = lerp(self.permutations[min], self.permutations[max], t);
        return out * self.amplitude;
    }

    /// Get the seed used to generate the noise.
    #[must_use]
    pub fn seed(&self) -> u64 {
        return self.seed;
    }
}

impl Default for Noise {
    fn default() -> Self {
        return Self::new(0, 1.0, 1.0, 0.0);
    }
}
