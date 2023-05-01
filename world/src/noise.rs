use rand::{SeedableRng, Rng};

use crate::util::lerp;

#[allow(clippy::pedantic)]
pub fn perlin<F>(v: f64, smooth_fn: F, seed: u64, f: f64, a: f64, o: f64) -> f64
where
    F: Fn(f64) -> f64,
{
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
    let perms = (0..256).map(|_| return rng.gen_range(0f64..=1f64)).collect::<Vec<_>>();
    let mask = perms.len() as isize - 1;

    let v = v * f + o;
    let i = v.floor() as isize;

    // get the frational part of v, if it is negative, then add 1
    // so that it is always positive and in the range (0..=1)
    let mut f = v.fract();
    if f < 0.0 { f += 1.0 }

    let t = smooth_fn(f);

    let min = i & mask;
    let max = (min + 1) & mask;
    let out = lerp(perms[min as usize], perms[max as usize], t);
    return out * a;
}
