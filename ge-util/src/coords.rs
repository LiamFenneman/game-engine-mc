use thiserror::Error;

pub(crate) type Result<T> = std::result::Result<T, CoordError>;

pub const CHUNK_SIZE: i32 = 16;
pub const CHUNK_SIZE_MASK: i32 = CHUNK_SIZE - 1;
pub const CHUNK_HEIGHT: i32 = 256;
pub const CHUNK_HEIGHT_MASK: i32 = CHUNK_HEIGHT - 1;

/// A coordinate position in the world.
///
/// Note: `x` and `y` can contain any value, but `z` must be always in the range `0..256`.
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
/// Note: This is a relative position, so `x` and `y` are always in the range `0..16`.
/// However, `z` is always in the range `0..256`.
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
    ///
    /// # Errors
    /// If the `z` axis is not in the range `0..=255`.
    pub fn new(x: i32, y: i32, z: i32) -> Result<Self> {
        let s = Self { x, y, z };
        if !s.is_valid() {
            return Err(CoordError::WorldPosRange(s));
        }
        return Ok(s);
    }

    /// Converts this `WorldPos` to a `ChunkPos`.
    #[must_use]
    pub fn to_chunk_pos(&self) -> ChunkPos {
        return ChunkPos::new(
            self.x & CHUNK_SIZE_MASK,
            self.y & CHUNK_SIZE_MASK,
            self.z & CHUNK_HEIGHT_MASK,
        )
        .expect("chunk pos domain should be smaller than world pos");
    }

    #[inline]
    #[must_use]
    pub fn is_valid(&self) -> bool {
        return (0..CHUNK_HEIGHT).contains(&self.z);
    }
}

impl ChunkPos {
    /// Creates a new `ChunkPos`.
    ///
    /// # Errors
    /// If the position is out of range.
    pub fn new(x: i32, y: i32, z: i32) -> Result<Self> {
        let s = Self { x, y, z };
        if !s.is_valid() {
            return Err(CoordError::ChunkPosRange(s));
        }
        return Ok(s);
    }

    /// Converts this `ChunkPos` to a `WorldPos` using a `ChunkOffset`.
    ///
    /// # Errors
    /// If the position is out of range.
    pub fn to_world_pos(&self, chunk_offset: impl Into<ChunkOffset>) -> WorldPos {
        let chunk_offset = chunk_offset.into();
        return WorldPos::new(
            chunk_offset.x * CHUNK_SIZE + self.x,
            chunk_offset.y * CHUNK_SIZE + self.y,
            self.z, // offset.z should always be 0
        )
        .expect("world pos constraints should be met");
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
    /// # Errors
    /// If the position is out of range.
    pub fn new(x: i32, y: i32, z: i32) -> Result<Self> {
        let s = Self { x, y, z };
        if !s.is_valid() {
            return Err(CoordError::ChunkOffsetRange(s));
        }
        return Ok(s);
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

#[derive(Debug, Clone, Copy, Error, PartialEq)]
pub enum CoordError {
    #[error("world position out of bounds: {0}")]
    WorldPosRange(WorldPos),
    #[error("chunk position out of bounds: {0}")]
    ChunkPosRange(ChunkPos),
    #[error("chunk offset out of bounds: {0}")]
    ChunkOffsetRange(ChunkOffset),
}

#[allow(clippy::pedantic)]
#[cfg(test)]
mod tests {
    mod convert {
        use crate::*;
        use rstest::rstest;

        #[rstest]
        #[case(wpos!(1, 2, 3)?, cpos!(1, 2, 3)?)]
        #[case(wpos!(15, 15, 15)?, cpos!(15, 15, 15)?)]
        #[case(wpos!(16, 16, 16)?, cpos!(0, 0, 16)?)]
        #[case(wpos!(-1, -1, 0)?, cpos!(15, 15, 0)?)]
        fn to_chunk_pos(
            #[case] pos: WorldPos,
            #[case] expected: ChunkPos,
        ) -> Result<(), crate::coords::CoordError> {
            assert_eq!(pos.to_chunk_pos(), expected);
            Ok(())
        }

        #[rstest]
        #[case(cpos!(1, 2, 3)?, wpos!(1, 2, 3)?, (0, 0))]
        #[case(cpos!(15, 15, 15)?, wpos!(31, 31, 15)?, (1, 1))]
        #[case(cpos!(5, 5, 5)?, wpos!(-11, -11, 5)?, (-1, -1))]
        #[case(cpos!(1, 1, 90)?, wpos!(49, 49, 90)?, (3, 3))]
        fn to_world_pos(
            #[case] pos: ChunkPos,
            #[case] expected: WorldPos,
            #[case] offset: (i32, i32),
        ) -> Result<(), crate::coords::CoordError> {
            let off = ChunkOffset::new(offset.0, offset.1, 0)?;
            assert_eq!(pos.to_world_pos(off), expected);
            Ok(())
        }
    }

    mod world_pos {
        #[allow(clippy::wildcard_imports)]
        use crate::coords::*;
        use rstest::rstest;

        #[rstest]
        #[case(0, 0, 0)]
        #[case(i32::MAX, i32::MAX, CHUNK_HEIGHT_MASK)]
        #[case(i32::MIN, i32::MIN, 0)]
        fn valid(#[case] x: i32, #[case] y: i32, #[case] z: i32) {
            assert!(WorldPos::new(x, y, z).unwrap().is_valid());
        }

        #[rstest]
        #[should_panic]
        #[case(0, 0, -1)]
        #[should_panic]
        #[case(0, 0, CHUNK_HEIGHT)]
        fn invalid(#[case] x: i32, #[case] y: i32, #[case] z: i32) {
            assert!(!WorldPos::new(x, y, z).unwrap().is_valid());
        }
    }

    mod chunk_pos {
        #[allow(clippy::wildcard_imports)]
        use crate::coords::*;
        use rstest::rstest;

        #[rstest]
        #[case(0, 0, 0)]
        #[case(CHUNK_SIZE_MASK, CHUNK_SIZE_MASK, CHUNK_HEIGHT_MASK)]
        fn valid(#[case] x: i32, #[case] y: i32, #[case] z: i32) {
            assert!(ChunkPos::new(x, y, z).unwrap().is_valid());
        }

        #[rstest]
        #[should_panic]
        #[case(-1, 0, 0)]
        #[should_panic]
        #[case(0, -1, 0)]
        #[should_panic]
        #[case(0, 0, -1)]
        #[should_panic]
        #[case(CHUNK_SIZE, 0, 0)]
        #[should_panic]
        #[case(0, CHUNK_SIZE, 0)]
        #[should_panic]
        #[case(0, 0, CHUNK_HEIGHT)]
        fn invalid(#[case] x: i32, #[case] y: i32, #[case] z: i32) {
            assert!(!ChunkPos::new(x, y, z).unwrap().is_valid());
        }
    }

    mod chunk_off {
        #[allow(clippy::wildcard_imports)]
        use crate::coords::*;
        use rstest::rstest;

        #[rstest]
        #[case(0, 0, 0)]
        #[case(i32::MAX/16, i32::MAX/16, 0)]
        #[case(i32::MIN/16, i32::MIN/16, 0)]
        fn valid(#[case] x: i32, #[case] y: i32, #[case] z: i32) {
            assert!(ChunkOffset::new(x, y, z).unwrap().is_valid());
        }

        #[rstest]
        #[should_panic]
        #[case(0, 0, -1)]
        #[should_panic]
        #[case(0, 0, CHUNK_HEIGHT)]
        fn invalid(#[case] x: i32, #[case] y: i32, #[case] z: i32) {
            assert!(!ChunkOffset::new(x, y, z).unwrap().is_valid());
        }
    }
}
