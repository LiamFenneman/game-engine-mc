pub const CHUNK_SIZE: i32 = 16;
pub const CHUNK_SIZE_MASK: i32 = CHUNK_SIZE - 1;
pub const CHUNK_HEIGHT: i32 = 256;
pub const CHUNK_HEIGHT_MASK: i32 = CHUNK_HEIGHT - 1;

/// A coordinate position in the world.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct WorldPos {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) z: i32,
}

/// A coordinate position within the chunk.
///
/// This is a relative position, so `x` and `y` are always in the range `0..16`. However,
/// `z` is always in the range `0..256`.
///
/// # Panics
/// If the position is out of range.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct ChunkPos {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) z: i32,
}

/// A coordinate position of a chunk in the world.
///
/// This is effectively a `WorldPos` that has been rounded down to the nearest chunk.
/// The valid range of `x`, `y`, values is `-2^28..2^28`. The `z` value must always be `0`.
/// This is due to the fact chunks are only concerned with the `x` and `y` axes.
///
/// # Panics
/// If the position is out of range.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub struct ChunkOffset {
    pub(crate) x: i32,
    pub(crate) y: i32,
    pub(crate) z: i32,
}

impl WorldPos {
    /// Creates a new `WorldPos`.
    #[must_use]
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        return Self { x, y, z };
    }

    /// Converts this `WorldPos` to a `ChunkPos`.
    #[must_use]
    pub fn to_chunk_pos(&self) -> ChunkPos {
        return ChunkPos::new(
            self.x & CHUNK_SIZE_MASK,
            self.y & CHUNK_SIZE_MASK,
            self.z & CHUNK_HEIGHT_MASK,
        );
    }

    #[inline]
    #[must_use]
    pub fn is_valid(&self) -> bool {
        return true;
    }
}

impl ChunkPos {
    /// Creates a new `ChunkPos`.
    ///
    /// # Panics
    /// If the position is out of range.
    #[must_use]
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        let s = Self { x, y, z };
        assert!(s.is_valid(), "chunk position out of range");
        return s;
    }

    /// Converts this `ChunkPos` to a `WorldPos` using a `ChunkOffset`.
    #[must_use]
    pub fn to_world_pos(&self, chunk_offset: impl Into<ChunkOffset>) -> WorldPos {
        let chunk_offset = chunk_offset.into();
        return WorldPos::new(
            chunk_offset.x * CHUNK_SIZE + self.x,
            chunk_offset.y * CHUNK_SIZE + self.y,
            // chunk_offset.z * CHUNK_HEIGHT + self.z, -- offset.z should always be 0
            self.z,
        );
    }

    /// Returns whether this `ChunkPos` is valid.
    #[inline]
    #[must_use]
    pub fn is_valid(&self) -> bool {
        return (0..CHUNK_SIZE).contains(&self.x)
            && (0..CHUNK_SIZE).contains(&self.y)
            && (0..CHUNK_HEIGHT).contains(&self.z);
    }
}

impl ChunkOffset {
    /// Creates a new `ChunkPos`.
    ///
    /// # Panics
    /// If the position is out of range.
    #[must_use]
    pub fn new(x: i32, y: i32, _: i32) -> Self {
        let s = Self { x, y, z: 0 };
        assert!(s.is_valid(), "chunk offset out of range");
        return s;
    }

    /// Returns whether this `ChunkOffset` is valid.
    #[inline]
    #[must_use]
    pub fn is_valid(&self) -> bool {
        return (-(2i32.pow(28))..2i32.pow(28)).contains(&self.x)
            && (-(2i32.pow(28))..2i32.pow(28)).contains(&self.y)
            && (self.z == 0);
    }
}

#[cfg(test)]
mod tests {
    mod convert {
        use crate::{cpos, wpos};

        #[test]
        fn to_chunk_pos() {
            let pos = wpos!(1, 2, 3);
            assert_eq!(pos.to_chunk_pos(), cpos!(1, 2, 3));

            let pos = wpos!(15, 15, 15);
            assert_eq!(pos.to_chunk_pos(), cpos!(15, 15, 15));

            let pos = wpos!(16, 16, 16);
            assert_eq!(pos.to_chunk_pos(), cpos!(0, 0, 0));

            let pos = wpos!(-1, -1, -1);
            assert_eq!(pos.to_chunk_pos(), cpos!(15, 15, 15));
        }

        #[test]
        fn to_world_pos() {
            let pos = cpos!(1, 2, 3);
            assert_eq!(pos.to_world_pos((0, 0, 0)), wpos!(1, 2, 3));

            let pos = cpos!(15, 15, 15);
            assert_eq!(pos.to_world_pos((1, 1, 0)), wpos!(31, 31, 15));

            let pos = cpos!(5, 5, 5);
            assert_eq!(pos.to_world_pos((-1, -1, 0)), wpos!(-11, -11, 5));
        }
    }

    mod chunk_pos {
        use crate::cpos;
        #[test]
        fn valid() {
            _ = cpos!(0, 0, 0);
            _ = cpos!(1, 2, 3);
            _ = cpos!(15, 15, 15);
        }

        #[test]
        #[should_panic]
        fn panic_large() {
            _ = cpos!(16, 16, 16);
        }

        #[test]
        #[should_panic]
        fn panic_negative() {
            _ = cpos!(-1, -1, -1);
        }
    }

    mod chunk_off {
        use crate::{coff, ChunkOffset};
        #[test]
        fn valid() {
            _ = coff!(-100, -10_000, -1_000_000);
            _ = coff!(0, 0, 0);
            _ = coff!(1, 2, 3);
            _ = coff!(15, 15, 15);
        }

        #[test]
        #[should_panic]
        fn panic_large() {
            _ = ChunkOffset::new(i32::MAX, i32::MAX, i32::MAX);
        }

        #[test]
        #[should_panic]
        fn panic_negative() {
            _ = ChunkOffset::new(i32::MIN, i32::MIN, i32::MIN);
        }
    }
}
