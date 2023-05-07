use cgmath::{Vector2, Vector3};

/// A `World` is a collection of `Block`s.
#[derive(Debug, Clone)]
pub struct World {
    pub chunks: Vec<Chunk>,
}

/// A `Chunk` is a collection of `Block`s with a fixed size.
#[derive(Debug, Clone)]
pub struct Chunk {
    pub blocks: Vec<Block>,
    pub position: Vector2<u32>,
}

impl Chunk {
    pub const SIZE: Vector3<u32> = Vector3::new(16, 256, 16);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum BlockType {
    Air,
    Grass,
    Stone,
    Water,
    Wood,
}

impl std::fmt::Display for BlockType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            BlockType::Air => write!(f, " "),
            BlockType::Grass => write!(f, "G"),
            BlockType::Stone => write!(f, "S"),
            BlockType::Water => write!(f, "."),
            BlockType::Wood => write!(f, "W"),
        };
    }
}

/// A `Block` is a single cube in the world.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Block {
    pub ty: BlockType,
    pub position: Vector3<u32>,
}
