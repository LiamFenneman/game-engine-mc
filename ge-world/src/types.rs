use ge_util::{ChunkOffset, ChunkPos, WorldPos};
use std::collections::HashMap;

const CULLING: bool = true;

/// A `World` is a collection of `Block`s.
#[derive(Debug, Clone)]
pub struct World {
    pub chunks: Vec<Chunk>,
}

impl World {
    /// Flatten all the chunks into a list of blocks using `WorldPos`.
    #[must_use]
    pub fn into_world_blocks(&self) -> Vec<Block> {
        return self
            .chunks
            .iter()
            .flat_map(|c| {
                // dbg!(c.position);
                return c.blocks.values().map(|b| return *b);
            })
            .collect();
    }
}

/// A `Chunk` is a collection of `Block`s with a fixed size.
#[derive(Debug, Clone)]
pub struct Chunk {
    pub blocks: HashMap<ChunkPos, Block>,
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
                .filter_map(|&o| {
                    return ChunkPos::new(
                        blk.chunk_pos.x() + o.0,
                        blk.chunk_pos.y() + o.1,
                        blk.chunk_pos.z() + o.2,
                    )
                    .ok();
                })
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
    pub fn apply_transformation(mut self, transform: &dyn ChunkTransformation) -> Self {
        transform.transform_timed(&mut self, transform.name());
        return self;
    }
}

pub trait ChunkTransformation {
    fn name(&self) -> &'static str;

    fn transform(&self, chunk: &mut Chunk);

    fn transform_timed(&self, chunk: &mut Chunk, trace_name: &str) {
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
    ty: BlockType,
    chunk_pos: ChunkPos,
    chunk_offset: ChunkOffset,
}

impl Block {
    /// Create a new block.
    pub fn new(
        ty: BlockType,
        chunk_pos: impl Into<ChunkPos>,
        chunk_offset: impl Into<ChunkOffset>,
    ) -> Self {
        return Self {
            ty,
            chunk_pos: chunk_pos.into(),
            chunk_offset: chunk_offset.into(),
        };
    }

    /// Get the type of this block.
    #[must_use]
    pub fn ty(&self) -> BlockType {
        return self.ty;
    }

    /// Get the chunk-relative position of this block.
    #[must_use]
    pub fn chunk_pos(&self) -> ChunkPos {
        return self.chunk_pos;
    }

    /// Get the world-relative position of this block.
    #[must_use]
    pub fn world_pos(&self) -> WorldPos {
        return self.chunk_pos.to_world_pos(self.chunk_offset);
    }

    /// Get the chunk offset this block belongs to.
    #[must_use]
    pub fn chunk_offset(&self) -> ChunkOffset {
        return self.chunk_offset;
    }

    /// Update the block type of this block.
    pub fn update(&mut self, ty: BlockType) {
        self.ty = ty;
    }
}
