use cgmath::Vector3;

#[must_use]
pub const fn pos_to_idx(pos: Vector3<u32>, size: u32) -> usize {
    return (pos.y * size.pow(2) + pos.z * size + pos.x) as usize;
}

#[must_use]
pub const fn idx_to_pos(idx: usize, size: u32) -> Vector3<u32> {
    return Vector3::new(
        idx as u32 % size,
        idx as u32 / (size * size),
        (idx as u32 / size) % size,
    );
}

/// Linear interpolation.
#[must_use]
pub fn lerp(a: f64, b: f64, t: f64) -> f64 {
    return a * (1.0 - t) + b * t;
}

/// Inverse linear interpolation.
#[must_use]
pub fn inverse_lerp(a: f64, b: f64, t: f64) -> f64 {
    return (t - a) / (b - a);
}

/// Remaps a value from one range to another.
#[must_use]
pub fn remap(t: f64, a_min: f64, a_max: f64, b_min: f64, b_max: f64) -> f64 {
    return lerp(b_min, b_max, inverse_lerp(a_min, a_max, t));
}

#[must_use]
pub fn rand(a: f64) -> f64 {
    return (a.sin() * 100_000.0).fract();
}

#[must_use]
pub fn cosine_smooth(t: f64) -> f64 {
    return (1.0 - (std::f64::consts::PI * t).cos()) * 0.5;
}

/// Ken Perlin's original smoothstep function.
#[must_use]
pub fn smoothstep(t: f64) -> f64 {
    return t * t * (3.0 - 2.0 * t);
}

/// Ken Perlin's improved smoothstep function.
#[must_use]
pub fn smoothstep2(t: f64) -> f64 {
    return t * t * t * (t * (t * 6.0 - 15.0) + 10.0);
}
