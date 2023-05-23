// use rand::{Rng, SeedableRng};
use ge_util::EngineConfig;
use rand::{seq::SliceRandom, SeedableRng};

pub const DEFAULT_SEED: u64 = 0;
pub const MAX_OCTAVES: usize = 32;
pub const SIZE: usize = 256;
pub const MASK: usize = SIZE - 1;

#[derive(Debug, Clone, Copy)]
pub struct Noise {
    seed: u64,
    octaves: usize,
    frequency: f32,
    amplitude: f32,
    lacunarity: f32,
    persistence: f32,

    perm: [u8; 512],
}

impl Noise {
    #[must_use]
    pub fn new(
        seed: u64,
        octaves: usize,
        frequency: f32,
        amplitude: f32,
        lacunarity: f32,
        persistence: f32,
    ) -> Self {
        let perm = {
            let mut p = (0u8..=0xFF).collect::<Vec<_>>();
            p.shuffle(&mut rand_chacha::ChaCha8Rng::seed_from_u64(seed));
            let mut buf = [0; 512];
            for (i, v) in p.iter().enumerate() {
                buf[i] = *v;
                buf[i + 256] = *v;
            }
            buf
        };

        debug_assert!(octaves <= MAX_OCTAVES, "octaves too high");
        return Self {
            seed,
            octaves,
            frequency,
            amplitude,
            lacunarity,
            persistence,
            perm,
        };
    }

    #[inline]
    fn fade(t: f32) -> f32 {
        return t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
    }

    #[allow(clippy::match_same_arms, reason = "code is cleaner this way")]
    #[inline]
    fn grad(hash: u8, x: f32, y: f32, z: f32) -> f32 {
        return match hash & 0xF {
            0x0 => x + y,
            0x1 => -x + y,
            0x2 => x - y,
            0x3 => -x - y,
            0x4 => x + z,
            0x5 => -x + z,
            0x6 => x - z,
            0x7 => -x - z,
            0x8 => y + z,
            0x9 => -y + z,
            0xA => y - z,
            0xB => -y - z,
            0xC => y + x,
            0xD => -y + z,
            0xE => y - x,
            0xF => -y - z,
            _ => unreachable!(),
        };
    }

    #[inline]
    fn hash(&self, x: usize, y: usize, z: usize) -> u8 {
        return self.perm[self.perm[self.perm[x] as usize + y] as usize + z];
    }

    #[allow(
        clippy::many_single_char_names,
        clippy::cast_sign_loss,
        clippy::cast_possible_wrap,
        clippy::cast_possible_truncation
    )]
    fn eval(&self, x: f32, y: f32, z: f32) -> f32 {
        let xi0 = (x.floor() as isize & MASK as isize) as usize;
        let yi0 = (y.floor() as isize & MASK as isize) as usize;
        let zi0 = (z.floor() as isize & MASK as isize) as usize;

        let xi1 = (xi0 + 1) & MASK;
        let yi1 = (yi0 + 1) & MASK;
        let zi1 = (zi0 + 1) & MASK;

        let tx = x - x.floor();
        let ty = y - y.floor();
        let tz = z - z.floor();

        let u = Self::fade(tx);
        let v = Self::fade(ty);
        let w = Self::fade(tz);

        let x0 = tx;
        let x1 = tx - 1.0;
        let y0 = ty;
        let y1 = ty - 1.0;
        let z0 = tz;
        let z1 = tz - 1.0;

        let a = Self::grad(self.hash(xi0, yi0, zi0), x0, y0, z0);
        let b = Self::grad(self.hash(xi1, yi0, zi0), x1, y0, z0);
        let c = Self::grad(self.hash(xi0, yi1, zi0), x0, y1, z0);
        let d = Self::grad(self.hash(xi1, yi1, zi0), x1, y1, z0);
        let e = Self::grad(self.hash(xi0, yi0, zi1), x0, y0, z1);
        let f = Self::grad(self.hash(xi1, yi0, zi1), x1, y0, z1);
        let g = Self::grad(self.hash(xi0, yi1, zi1), x0, y1, z1);
        let h = Self::grad(self.hash(xi1, yi1, zi1), x1, y1, z1);

        let k0 = a;
        let k1 = b - a;
        let k2 = c - a;
        let k3 = e - a;
        let k4 = a + d - b - c;
        let k5 = a + f - b - e;
        let k6 = a + g - c - e;
        let k7 = b + c + e + h - a - d - f - g;

        return k0
            + k1 * u
            + k2 * v
            + k3 * w
            + k4 * u * v
            + k5 * u * w
            + k6 * v * w
            + k7 * u * v * w;
    }

    #[must_use]
    pub fn fbm(&self, x: f32, y: f32, z: f32) -> f32 {
        let mut freq = 1.0 / self.frequency;
        let mut amp = self.amplitude;
        let mut sum = 0.0;

        for _ in 0..self.octaves {
            sum += self.eval(x * freq, y * freq, z * freq) * amp;
            freq *= self.lacunarity;
            amp *= self.persistence;
        }

        return sum;
    }

    #[must_use]
    pub fn seed(&self) -> u64 {
        return self.seed;
    }
}

impl From<&EngineConfig> for Noise {
    fn from(value: &EngineConfig) -> Self {
        return Self::new(
            DEFAULT_SEED,
            value.world_gen.noise.octaves,
            value.world_gen.noise.frequency,
            value.world_gen.noise.amplitude,
            value.world_gen.noise.lacunarity,
            value.world_gen.noise.persistence,
        );
    }
}
