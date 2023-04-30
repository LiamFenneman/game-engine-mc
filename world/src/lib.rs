#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::default_trait_access)]

use cgmath::Vector3;
use rand::distributions::{Distribution, Standard};
use rand::prelude::*;
use std::fmt::Display;

pub mod util {
    use cgmath::Vector3;

    #[must_use]
    pub const fn pos_to_idx(pos: Vector3<u32>, size: u32) -> usize {
        return (pos.y * size.pow(2) + pos.z * size + pos.x) as usize;
    }

    #[must_use]
    pub const fn idx_to_pos(idx: usize, size: u32) -> Vector3<u32> {
        return Vector3::new(
            idx as u32 % size,
            idx as u32 / (size * size),
            (idx as u32 / size) % size,
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    Air,
    Grass,
    Stone,
    Water,
    Wood,
}

impl Display for BlockType {
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
        return match rng.gen_range(0..4) {
            0 => BlockType::Grass,
            1 => BlockType::Stone,
            2 => BlockType::Water,
            3 => BlockType::Wood,
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

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        // compare the x, y, z of the blocks
        return self
            .position
            .x
            .partial_cmp(&other.position.x)
            .or_else(|| return self.position.z.partial_cmp(&other.position.z))
            .or_else(|| return self.position.y.partial_cmp(&other.position.y));
    }
}

/// A `World` is a collection of `Block`s.
#[derive(Debug, Clone)]
pub struct World {
    pub blocks: Vec<Block>,
}

/// A `WorldGenerator` is a trait that generates a `World`.
pub trait WorldGenerator {
    fn generate(&self) -> World;
}

pub struct RandomWorldGenerator {
    pub world_size: u32,
}

impl WorldGenerator for RandomWorldGenerator {
    #[allow(clippy::cast_precision_loss)]
    fn generate(&self) -> World {
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(2);

        // generate a plane of grass
        let mut blocks = vec![];
        for y in 0..self.world_size {
            for z in 0..self.world_size {
                for x in 0..self.world_size {
                    blocks.push(Block {
                        ty: rng.gen(),
                        position: Vector3::new(x, y, z),
                    });
                }
            }
        }

        return World { blocks };
    }
}
