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
