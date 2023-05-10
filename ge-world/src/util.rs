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

#[must_use]
pub fn three_to_one(x: u32, y: u32, z: u32, size: cgmath::Vector3<u32>) -> usize {
    return (x + y * size.x + z * size.x * size.y) as usize;
}

#[must_use]
pub fn one_to_three(index: usize, size: cgmath::Vector3<u32>) -> cgmath::Vector3<u32> {
    let x = index % size.x as usize;
    let y = (index / size.x as usize) % size.y as usize;
    let z = index / (size.x * size.y) as usize;
    return cgmath::vec3(
        u32::try_from(x).expect("index is out of bounds"),
        u32::try_from(y).expect("index is out of bounds"),
        u32::try_from(z).expect("index is out of bounds"),
    );
}

#[must_use]
pub fn two_to_one(x: u32, y: u32, size: cgmath::Vector2<u32>) -> usize {
    return (x + y * size.x) as usize;
}

#[must_use]
pub fn one_to_two(index: usize, size: cgmath::Vector2<u32>) -> cgmath::Vector2<u32> {
    let x = index % size.x as usize;
    let y = index / size.x as usize;
    return cgmath::vec2(
        u32::try_from(x).expect("index is out of bounds"),
        u32::try_from(y).expect("index is out of bounds"),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use cgmath::{vec2, vec3};

    #[test]
    fn test_two() {
        let size = vec2(3, 3);

        let coord = one_to_two(0, size);
        assert_eq!(coord, vec2(0, 0));
        let index = two_to_one(0, 0, size);
        assert_eq!(index, 0);

        let coord = one_to_two(4, size);
        assert_eq!(coord, vec2(1, 1));
        let index = two_to_one(1, 1, size);
        assert_eq!(index, 4);

        let coord = one_to_two(8, size);
        assert_eq!(coord, vec2(2, 2));
        let index = two_to_one(2, 2, size);
        assert_eq!(index, 8);
    }

    #[test]
    fn test_three() {
        let size = vec3(3, 3, 3);

        let coord = one_to_three(0, size);
        assert_eq!(coord, vec3(0, 0, 0));
        let index = three_to_one(0, 0, 0, size);
        assert_eq!(index, 0);

        let coord = one_to_three(13, size);
        assert_eq!(coord, vec3(1, 1, 1));
        let index = three_to_one(1, 1, 1, size);
        assert_eq!(index, 13);

        let coord = one_to_three(26, size);
        assert_eq!(coord, vec3(2, 2, 2));
        let index = three_to_one(2, 2, 2, size);
        assert_eq!(index, 26);
    }

    #[test]
    fn test_nest() {
        let size = vec3(3, 3, 3);
        let coord = three_to_one(1, 1, 1, size);
        let index = one_to_three(coord, size);
        assert_eq!(index, vec3(1, 1, 1));

        let size = vec2(3, 3);
        let coord = two_to_one(1, 1, size);
        let index = one_to_two(coord, size);
        assert_eq!(index, vec2(1, 1));
    }
}
