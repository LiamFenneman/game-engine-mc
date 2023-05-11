use cgmath::{vec3, Vector2, Vector3};
use ge_util::three_to_one;

const CULLING: bool = true;

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

    #[allow(clippy::pedantic)] // TODO: remove this
    pub fn visible_blocks(&self) -> Vec<&Block> {
        let neighbour_offsets: [Vector3<i32>; 6] = [
            vec3(0, 0, 1),
            vec3(0, 0, -1),
            vec3(1, 0, 0),
            vec3(-1, 0, 0),
            vec3(0, 1, 0),
            vec3(0, -1, 0),
        ];

        if !CULLING {
            // if culling is disabled then return all blocks
            return self.blocks.iter().collect();
        }

        let mut visible_blocks = Vec::with_capacity(self.blocks.len());
        for (i, blk) in self.blocks.iter().enumerate() {
            // if the block is air then it is not visible
            if blk.ty == BlockType::Air {
                continue;
            }
            let neighbours = neighbour_offsets
                .iter()
                .map(|o| {
                    return (
                        u32::try_from(blk.position.x as i32 + o.x).ok(),
                        u32::try_from(blk.position.y as i32 + o.y).ok(),
                        u32::try_from(blk.position.z as i32 + o.z).ok(),
                    );
                })
                .filter_map(|o| {
                    return o.0.and_then(|x| {
                        return o.1.and_then(|y| return o.2.map(|z| return (x, y, z)));
                    });
                })
                .map(|p| return three_to_one(p.0, p.1, p.2, Self::SIZE))
                .filter_map(|o| return self.blocks.get(o))
                .collect::<Vec<_>>();

            // if the block is at the edge of the chunk then it is visible
            // or if the block neighbours an air block
            let num_visible = neighbours
                .iter()
                .filter(|b| return b.ty != BlockType::Air)
                .count();

            if num_visible < neighbours.len() {
                visible_blocks.push(&self.blocks[i]);
            }
        }

        return visible_blocks;
    }

    #[must_use]
    pub fn apply_transformation(mut self, transform: &mut dyn ChunkTransformation) -> Self {
        transform.transform(&mut self);
        return self;
    }
}

pub trait ChunkTransformation {
    fn transform(&mut self, chunk: &mut Chunk);
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
