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

        impl<T> From<cgmath::Vector3<T>> for $ty
        where
            T: Into<i32>,
        {
            fn from(value: cgmath::Vector3<T>) -> Self {
                return Self {
                    x: value.x.into(),
                    y: value.y.into(),
                    z: value.z.into(),
                };
            }
        }

        impl<T> From<cgmath::Point3<T>> for $ty
        where
            T: Into<i32>,
        {
            fn from(value: cgmath::Point3<T>) -> Self {
                return Self {
                    x: value.x.into(),
                    y: value.y.into(),
                    z: value.z.into(),
                };
            }
        }
        impl<T> From<(T, T, T)> for $ty
        where
            T: Into<i32>,
        {
            fn from(value: (T, T, T)) -> Self {
                return Self::new(value.0.into(), value.1.into(), value.2.into());
            }
        }

        impl<T> std::ops::Add<T> for $ty
        where
            T: Into<Self>,
        {
            type Output = Self;

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
            type Output = Self;

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
            type Output = Self;

            fn neg(self) -> Self::Output {
                return Self::new(-self.x, -self.y, -self.z);
            }
        }
    };
}

impl_common!(WorldPos);
impl_common!(ChunkPos);
impl_common!(ChunkOffset);
