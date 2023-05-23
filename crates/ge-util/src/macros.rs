use crate::coords::{ChunkOffset, ChunkPos, WorldPos};

macro_rules! impl_common {
    ($ty:ty) => {
        impl $ty {
            #[must_use]
            pub fn x(&self) -> i32 {
                return self.x;
            }
            #[must_use]
            pub fn y(&self) -> i32 {
                return self.y;
            }
            #[must_use]
            pub fn z(&self) -> i32 {
                return self.z;
            }
            pub fn update(&mut self, x: i32, y: i32, z: i32) {
                self.x = x;
                self.y = y;
                self.z = z;
                assert!(self.is_valid(), "out of range");
            }
        }

        impl TryFrom<nalgebra::Vector3<i32>> for $ty {
            type Error = crate::coords::CoordError;

            fn try_from(value: nalgebra::Vector3<i32>) -> crate::coords::Result<Self> {
                return Self::new(value.x.into(), value.y.into(), value.z.into());
            }
        }

        impl<T> TryFrom<[T; 3]> for $ty
        where
            T: Into<i32> + Copy,
        {
            type Error = crate::coords::CoordError;

            fn try_from(value: [T; 3]) -> crate::coords::Result<Self> {
                return Self::new(value[0].into(), value[1].into(), value[2].into());
            }
        }

        impl<T> TryFrom<(T, T, T)> for $ty
        where
            T: Into<i32>,
        {
            type Error = crate::coords::CoordError;

            fn try_from(value: (T, T, T)) -> crate::coords::Result<Self> {
                return Self::new(value.0.into(), value.1.into(), value.2.into());
            }
        }

        impl<T> std::ops::Add<T> for $ty
        where
            T: Into<Self>,
        {
            type Output = crate::coords::Result<Self>;

            fn add(self, rhs: T) -> Self::Output {
                let rhs = rhs.into();
                return Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z);
            }
        }

        impl<T> std::ops::AddAssign<T> for $ty
        where
            T: Into<Self>,
        {
            fn add_assign(&mut self, rhs: T) {
                let rhs = rhs.into();
                self.x += rhs.x;
                self.y += rhs.y;
                self.z += rhs.z;
                assert!(self.is_valid(), "out of range");
            }
        }

        impl<T> std::ops::Sub<T> for $ty
        where
            T: Into<Self>,
        {
            type Output = crate::coords::Result<Self>;

            fn sub(self, rhs: T) -> Self::Output {
                let rhs = rhs.into();
                return Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z);
            }
        }

        impl<T> std::ops::SubAssign<T> for $ty
        where
            T: Into<ChunkOffset>,
        {
            fn sub_assign(&mut self, rhs: T) {
                let rhs = rhs.into();
                self.x -= rhs.x * $crate::coords::CHUNK_SIZE;
                self.y -= rhs.y * $crate::coords::CHUNK_SIZE;
                self.z -= rhs.z * $crate::coords::CHUNK_SIZE;
                assert!(self.is_valid(), "out of range");
            }
        }

        impl std::ops::Neg for $ty {
            type Output = crate::coords::Result<Self>;

            fn neg(self) -> Self::Output {
                return Self::new(-self.x, -self.y, -self.z);
            }
        }

        impl std::ops::Mul for $ty {
            type Output = crate::coords::Result<Self>;

            fn mul(self, rhs: Self) -> Self::Output {
                return Self::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z);
            }
        }

        impl std::ops::Div for $ty {
            type Output = crate::coords::Result<Self>;

            fn div(self, rhs: Self) -> Self::Output {
                return Self::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z);
            }
        }

        impl<T> std::ops::Mul<T> for $ty
        where
            T: Into<i32>,
        {
            type Output = crate::coords::Result<Self>;

            fn mul(self, rhs: T) -> Self::Output {
                let rhs = rhs.into();
                return Self::new(self.x * rhs, self.y * rhs, self.z * rhs);
            }
        }

        impl std::fmt::Display for $ty {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                return write!(f, "({}, {}, {})", self.x, self.y, self.z);
            }
        }

        impl Default for $ty {
            fn default() -> Self {
                return Self { x: 0, y: 0, z: 0 };
            }
        }
    };
}

impl_common!(WorldPos);
impl_common!(ChunkPos);
impl_common!(ChunkOffset);
