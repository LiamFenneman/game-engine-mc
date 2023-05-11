/// Returns a vector of points within a circle with radius `r`.
#[must_use]
pub fn points_in_circle(r: i32) -> Vec<(i32, i32)> {
    let mut pts = vec![];

    for x in -r..=r {
        for y in -r..=r {
            if f64::from(x).powi(2) + f64::from(y).powi(2) <= f64::from(r).powi(2) {
                pts.push((x, y));
            }
        }
    }

    return pts;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_0() {
        let r = 0;
        let expected = vec![(0, 0)];
        let actual = points_in_circle(r);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_circle_1() {
        let r = 1;
        let expected = vec![(-1, 0), (0, -1), (0, 0), (0, 1), (1, 0)];
        let actual = points_in_circle(r);
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_circle_2() {
        let r = 2;
        let expected = vec![
            (-2, 0),
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -2),
            (0, -1),
            (0, 0),
            (0, 1),
            (0, 2),
            (1, -1),
            (1, 0),
            (1, 1),
            (2, 0),
        ];
        let actual = points_in_circle(r);
        assert_eq!(expected, actual);
    }
}
