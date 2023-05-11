use crate::ChunkTransformation;
use ge_util::{coords::CHUNK_SIZE, WorldPos, ChunkPos};

/// A naive surface painter that paints the top layer of blocks.
pub struct SimpleSurfacePainter;

impl ChunkTransformation for SimpleSurfacePainter {
    fn transform(&mut self, chunk: &mut crate::Chunk) {
        // start at the top of the chunk, and work our way down
        // the first non air block we encounter is the top layer
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                let z = chunk
                    .blocks
                    .iter()
                    .filter(|(p, _)| return p.x() == x && p.y() == y)
                    .filter(|(_, b)| {
                        return b.ty != crate::BlockType::Air && b.ty != crate::BlockType::Water;
                    })
                    .map(|(p, _)| return p.z())
                    .max()
                    .unwrap_or(0);
                chunk.blocks.insert(
                    WorldPos::new(x, y, z),
                    crate::Block {
                        ty: crate::BlockType::Grass,
                        position: ChunkPos::new(x, y, z).to_world_pos(chunk.position),
                    }
                );
            }
        }
    }
}
