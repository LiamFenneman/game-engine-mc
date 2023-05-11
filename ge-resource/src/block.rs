use crate::{data::DataError, ResourceManager};
use ge_world::BlockType;

/// Meta data for a block type.
///
/// This is used to determine which textures to use for a block.
/// The order of the textures is as follows:
/// `[TOP, BOTTOM, LEFT, RIGHT, FRONT, BACK]`
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct BlockMeta {
    faces: [String; 6],
}

impl BlockMeta {
    #[must_use]
    pub fn new(faces: [String; 6]) -> Self {
        return Self { faces };
    }

    /// Load a block meta from disk.
    ///
    /// # Errors
    /// Errors if the file doesn't exist or the file cannot be parsed into `BlockMeta`.
    pub fn load_from_disk(rm: &ResourceManager, ty: BlockType) -> Result<Self, DataError> {
        let path = block_type_to_path(ty);
        return rm.load_data(&path);
    }

    #[must_use]
    pub fn get_faces(&self) -> &[String; 6] {
        return &self.faces;
    }
}

/// Convert a block type to a path.
///
/// # Panics
/// Panics if the block type is `BlockType::Air`. This is because air doesn't have textures.
#[must_use]
pub fn block_type_to_path(ty: BlockType) -> String {
    return match ty {
        BlockType::Dev => "dev.ron".to_owned(),
        BlockType::Air => unreachable!("air doesn't have textures"),
        BlockType::Dirt => "dirt.ron".to_owned(),
        BlockType::Grass => "grass.ron".to_owned(),
        BlockType::Stone => "stone.ron".to_owned(),
        BlockType::Water => "water.ron".to_owned(),
        BlockType::Wood => "wood.ron".to_owned(),
    };
}
