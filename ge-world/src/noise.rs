// use rand::{Rng, SeedableRng};

pub const MAX_OCTAVES: usize = 32;
pub const DEFAULT_OCTAVES: usize = 1;
pub const DEFAULT_FREQUENCY: f32 = 1.0;
pub const DEFAULT_AMPLITUDE: f32 = 1.0;
pub const DEFAULT_LACUNARITY: f32 = 2.0;
pub const DEFAULT_PERSISTENCE: f32 = 0.5;

pub const SIZE: usize = 256;
pub const MASK: usize = SIZE - 1;
pub const PERM: [u8; 512] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180, // -- Repeated -- //
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180,
];

#[derive(Debug, Clone, Copy)]
pub struct Noise {
    octaves: usize,
    frequency: f32,
    amplitude: f32,
    lacunarity: f32,
    persistence: f32,
}

impl Default for Noise {
    fn default() -> Self {
        return Self::new(
            DEFAULT_OCTAVES,
            DEFAULT_FREQUENCY,
            DEFAULT_AMPLITUDE,
            DEFAULT_LACUNARITY,
            DEFAULT_PERSISTENCE,
        );
    }
}

impl Noise {
    #[must_use]
    pub fn new(
        octaves: usize,
        frequency: f32,
        amplitude: f32,
        lacunarity: f32,
        persistence: f32,
    ) -> Self {
        debug_assert!(octaves <= MAX_OCTAVES, "octaves too high");
        return Self {
            octaves,
            frequency,
            amplitude,
            lacunarity,
            persistence,
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
    fn hash(x: usize, y: usize, z: usize) -> u8 {
        return PERM[PERM[PERM[x] as usize + y] as usize + z];
    }

    #[allow(
        clippy::many_single_char_names,
        clippy::cast_sign_loss,
        clippy::cast_possible_wrap,
        clippy::cast_possible_truncation
    )]
    fn eval(x: f32, y: f32, z: f32) -> f32 {
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

        let a = Self::grad(Self::hash(xi0, yi0, zi0), x0, y0, z0);
        let b = Self::grad(Self::hash(xi1, yi0, zi0), x1, y0, z0);
        let c = Self::grad(Self::hash(xi0, yi1, zi0), x0, y1, z0);
        let d = Self::grad(Self::hash(xi1, yi1, zi0), x1, y1, z0);
        let e = Self::grad(Self::hash(xi0, yi0, zi1), x0, y0, z1);
        let f = Self::grad(Self::hash(xi1, yi0, zi1), x1, y0, z1);
        let g = Self::grad(Self::hash(xi0, yi1, zi1), x0, y1, z1);
        let h = Self::grad(Self::hash(xi1, yi1, zi1), x1, y1, z1);

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
        let mut freq = self.frequency;
        let mut amp = self.amplitude;
        let mut sum = 0.0;

        for _ in 0..self.octaves {
            sum += Self::eval(x * freq, y * freq, z * freq) * amp;
            freq *= self.lacunarity;
            amp *= self.persistence;
        }

        return sum;
    }
}
