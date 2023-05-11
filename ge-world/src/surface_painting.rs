use crate::ChunkTransformation;

/// A naive surface painter that paints the top layer of blocks.
pub struct SimpleSurfacePainter;

impl ChunkTransformation for SimpleSurfacePainter {
    fn transform(&mut self, chunk: &mut crate::Chunk) {
        // start at the top of the chunk, and work our way down
        // the first non air block we encounter is the top layer
        for x in 0..crate::Chunk::SIZE.x {
            for z in 0..crate::Chunk::SIZE.z {
                let blk = chunk
                    .blocks
                    .iter_mut()
                    .rev()
                    .find(|block| {
                        return block.position.x == x
                            && block.position.z == z
                            && block.ty != crate::BlockType::Air;
                    })
                    .unwrap();
                blk.ty = crate::BlockType::Grass;
            }
        }
    }
}
