use cgmath::Vector3;
use rand::{distributions::Standard, prelude::Distribution};

/// A `World` is a collection of `Block`s.
#[derive(Debug, Clone)]
pub struct World {
    pub blocks: Vec<Block>,
    pub size: Vector3<u32>,
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

impl Distribution<BlockType> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> BlockType {
        return match rng.gen_range(0..5) {
            0 => BlockType::Air,
            1 => BlockType::Grass,
            2 => BlockType::Stone,
            3 => BlockType::Water,
            4 => BlockType::Wood,
            _ => unreachable!(),
        };
    }
}

/// A `Block` is a single cube in the world.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Block {
    pub ty: BlockType,
    pub position: Vector3<u32>,
}
