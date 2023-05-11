use ge_util::coords::CHUNK_SIZE;
use crate::ChunkTransformation;

/// A naive surface painter that paints the top layer of blocks.
pub struct SimpleSurfacePainter;

impl ChunkTransformation for SimpleSurfacePainter {
    fn transform(&mut self, chunk: &mut crate::Chunk) {
        // start at the top of the chunk, and work our way down
        // the first non air block we encounter is the top layer
        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                if let Some(blk) = chunk.blocks.iter_mut().rev().find(|block| {
                    return block.position.x() == x
                        && block.position.y() == y
                        && block.ty != crate::BlockType::Air
                        && block.ty != crate::BlockType::Water;
                }) {
                    blk.ty = crate::BlockType::Grass;
                }
            }
        }
    }
}
