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
