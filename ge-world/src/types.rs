use ge_util::{ChunkOffset, WorldPos};
use std::collections::HashMap;

const CULLING: bool = true;

/// A `World` is a collection of `Block`s.
#[derive(Debug, Clone)]
pub struct World {
    pub chunks: Vec<Chunk>,
}

/// A `Chunk` is a collection of `Block`s with a fixed size.
#[derive(Debug, Clone)]
pub struct Chunk {
    pub blocks: HashMap<WorldPos, Block>,
    pub position: ChunkOffset,
}

impl Chunk {
    pub const SIZE: cgmath::Vector3<u32> = cgmath::Vector3::new(16, 256, 16);

    #[must_use]
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
            return self.blocks.values().collect();
        }

        let mut visible_blocks = Vec::with_capacity(self.blocks.len());
        for blk in self.blocks.values() {
            // if the block is air then it is not visible
            if blk.ty == BlockType::Air {
                continue;
            }
            let neighbours = neighbour_offsets
                .iter()
                .map(|&o| return blk.position + WorldPos::from(o))
                .filter_map(|o| return self.blocks.get(&o))
                .collect::<Vec<_>>();

            // if the block is at the edge of the chunk then it is visible
            // or if the block neighbours an air block
            let num_visible = neighbours
                .iter()
                .filter(|b| return b.ty != BlockType::Air)
                .count();

            if num_visible < neighbours.len() {
                visible_blocks.push(blk);
            }
        }

        return visible_blocks;
    }

    #[must_use]
    pub fn apply_transformation(
        mut self,
        transform: &mut dyn ChunkTransformation,
        trace_name: &str,
    ) -> Self {
        transform.transform_timed(&mut self, trace_name);
        return self;
    }
}

pub trait ChunkTransformation {
    fn transform(&mut self, chunk: &mut Chunk);

    fn transform_timed(&mut self, chunk: &mut Chunk, trace_name: &str) {
        let start = std::time::Instant::now();
        self.transform(chunk);
        tracing::trace!(
            "{}: transform took {:?} ms",
            trace_name,
            start.elapsed().as_millis()
        );
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum BlockType {
    Dev,
    Air,
    Dirt,
    Grass,
    Stone,
    Water,
    Wood,
}

impl BlockType {
    /// Returns `true` if the block is (at least partially) transparent.
    #[must_use]
    pub fn is_transparent(&self) -> bool {
        return matches!(self, BlockType::Air | BlockType::Water);
    }

    /// Returns `true` if the block is fully opaque.
    #[must_use]
    pub fn is_opaque(&self) -> bool {
        return !self.is_transparent();
    }
}

impl std::fmt::Display for BlockType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            BlockType::Dev => write!(f, "0"),
            BlockType::Air => write!(f, " "),
            BlockType::Dirt => write!(f, "D"),
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
