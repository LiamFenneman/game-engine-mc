use crate::util;
use std::{collections::BTreeMap, ops::Bound};

/// A spline is a set of points that can be interpolated between.
///
/// This implementation uses `u32` as the points and values.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Spline {
    points: BTreeMap<u32, u32>,
    max: (u32, u32),
    min: (u32, u32),
}

impl Spline {
    /// Create a new spline point.
    pub fn insert(&mut self, point: u32, value: u32) {
        self.points.insert(point, value);

        if point > self.max.0 {
            self.max = (point, value);
        }
        if point < self.min.0 {
            self.min = (point, value);
        }
    }

    /// Get the value of the spline at the given point.
    ///
    /// If the point is not a key in the spline, then the value is interpolated.
    /// If the point is out of bounds, then `None` is returned.
    #[must_use]
    pub fn get(&self, point: u32) -> u32 {
        return self.points.get(&point).copied().unwrap_or_else(|| {
            // get the point before and after the requested point
            let next = self.points.lower_bound(Bound::Included(&point));
            let Some(prev) = next.peek_prev() else {
                return self.min.1;
            };
            let Some(next) = next.key_value() else {
                return self.max.1;
            };

            // lerp between the two points
            let t = f64::from(point - prev.0) / f64::from(next.0 - prev.0);
            let r = util::lerp(f64::from(*prev.1), f64::from(*next.1), t);

            // return the value as u32
            return r.floor() as u32;
        });
    }

    /// Get the value of the spline at the given point, using a value between 0 and 1.
    /// Where 0 is the minimum point, and 1 is the maximum point.
    ///
    /// # Panics
    /// Panics if `t` is not within the range `0.0..=1.0`.
    #[must_use]
    pub fn get_with_t(&self, t: f64) -> u32 {
        assert!((0.0..=1.0).contains(&t), "t must be between 0 and 1");

        // a point was originally a u32, so negative *should*  never occur.
        let i = util::lerp(self.min.0.into(), self.max.0.into(), t).floor() as u32;
        return self.get(i);
    }
}

impl Default for Spline {
    fn default() -> Self {
        return Self {
            points: BTreeMap::new(),
            max: (u32::MIN, u32::MIN),
            min: (u32::MAX, u32::MAX),
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get() {
        let mut s = Spline::default();
        s.insert(1, 50);
        s.insert(3, 100);
        s.insert(5, 120);

        // basic cases
        assert_eq!(50, s.get(1));
        assert_eq!(100, s.get(3));
        assert_eq!(120, s.get(5));

        // lerp was used
        assert_eq!(75, s.get(2));
        assert_eq!(110, s.get(4));

        // keys out of bounds
        assert_eq!(s.get(1), s.get(0));
        assert_eq!(s.get(5), s.get(6));
    }

    #[test]
    fn get_with_t() {
        let mut s = Spline::default();
        s.insert(1, 50);
        s.insert(3, 100);
        s.insert(5, 120);

        // basic cases
        assert_eq!(50, s.get_with_t(0.0));
        assert_eq!(120, s.get_with_t(1.0));
        assert_eq!(100, s.get_with_t(0.5));

        // lerp was used
        assert_eq!(75, s.get_with_t(0.25));
        assert_eq!(110, s.get_with_t(0.75));
    }

    #[test]
    #[should_panic]
    fn get_with_t_panic() {
        let s = Spline::default();
        _ = s.get_with_t(2.0);
        _ = s.get_with_t(-1.0);
    }
}
