use crate::coords::{WorldPos, ChunkPos, ChunkOffset};

#[rustfmt::skip]
macro_rules! impl_common {
    ($ty:ty) => {
        impl $ty {
            #[must_use] pub fn x(&self) -> i32 { return self.x; }
            #[must_use] pub fn y(&self) -> i32 { return self.y; }
            #[must_use] pub fn z(&self) -> i32 { return self.z; }

            // pub fn x_mut(&mut self) -> &mut i32 { return &mut self.x; }
            // pub fn y_mut(&mut self) -> &mut i32 { return &mut self.y; }
            // pub fn z_mut(&mut self) -> &mut i32 { return &mut self.z; }

            pub fn update(&mut self, x: i32, y: i32, z: i32) {
                self.x = x;
                self.y = y;
                self.z = z;
                assert!(self.is_valid(), "out of range");
            }
        }
    };
}

macro_rules! impl_from_cgmath {
    ($ty:ty) => {
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
    };
}

macro_rules! impl_from_tuple {
    ($ty:ty) => {
        impl<T> From<(T, T, T)> for $ty
        where
            T: Into<i32>,
        {
            fn from(value: (T, T, T)) -> Self {
                return Self::new(value.0.into(), value.1.into(), value.2.into());
            }
        }
    };
}

impl_common!(WorldPos);
impl_common!(ChunkPos);
impl_common!(ChunkOffset);

impl_from_cgmath!(WorldPos);
impl_from_cgmath!(ChunkPos);
impl_from_cgmath!(ChunkOffset);

impl_from_tuple!(WorldPos);
impl_from_tuple!(ChunkPos);
impl_from_tuple!(ChunkOffset);
