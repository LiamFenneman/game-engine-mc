#[inline]
#[must_use]
pub fn deg_to_rad(degrees: f32) -> f32 {
    return degrees * std::f32::consts::PI / 180.0;
}

#[inline]
#[must_use]
pub fn rad_to_deg(radians: f32) -> f32 {
    return radians * 180.0 / std::f32::consts::PI;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(0.0, 0.0)]
    #[case(90.0, std::f32::consts::FRAC_PI_2)]
    #[case(180.0, std::f32::consts::PI)]
    #[case(270.0, std::f32::consts::PI + std::f32::consts::FRAC_PI_2)]
    #[case(360.0, std::f32::consts::PI * 2.0)]
    fn test_rad_deg(#[case] degrees: f32, #[case] expected_radians: f32) {
        assert!(deg_to_rad(degrees).eq(&expected_radians));
        assert!(rad_to_deg(expected_radians).eq(&degrees));
    }
}
