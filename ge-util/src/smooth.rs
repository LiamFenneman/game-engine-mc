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
