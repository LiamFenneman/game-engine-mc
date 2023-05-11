use ge_util::{ChunkOffset, WorldPos};

const CULLING: bool = false;

/// A `World` is a collection of `Block`s.
#[derive(Debug, Clone)]
pub struct World {
    pub chunks: Vec<Chunk>,
}

/// A `Chunk` is a collection of `Block`s with a fixed size.
#[derive(Debug, Clone)]
pub struct Chunk {
    pub blocks: Vec<Block>,
    pub position: ChunkOffset,
}

impl Chunk {
    pub const SIZE: cgmath::Vector3<u32> = cgmath::Vector3::new(16, 256, 16);

    #[allow(clippy::pedantic)] // TODO: remove this
    pub fn visible_blocks(&self) -> Vec<&Block> {
        let neighbour_offsets: [(i32, i32, i32); 6] = [
            (0, 0, 1),
            (0, 0, -1),
            (1, 0, 0),
            (-1, 0, 0),
            (0, 1, 0),
            (0, -1, 0),
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
                // TODO: fix culling
                // .map(|o| return blk.position + o)
                // .filter_map(|o| {
                //     return o.0.and_then(|x| {
                //         return o.1.and_then(|y| return o.2.map(|z| return (x, y, z)));
                //     });
                // })
                // .map(|p| return three_to_one(p.x(), p.y(), p.z(), Self::SIZE))
                .map(|p| return 0)
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
    pub position: WorldPos,
}
