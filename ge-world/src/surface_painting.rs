use crate::ChunkTransformation;
use ge_util::{coords::CHUNK_SIZE, ChunkPos, WorldPos};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

/// A naive surface painter that paints the top layer of blocks.
pub struct SimpleSurfacePainter;

impl ChunkTransformation for SimpleSurfacePainter {
    fn transform(&mut self, chunk: &mut crate::Chunk) {
        // loop over all x and z coordinates, take find the highest block at that x and z
        // coordinate, paint the block above that block
        let chunks = (0..CHUNK_SIZE)
            .flat_map(|x| {
                return (0..CHUNK_SIZE).map(move |y| {
                    return (x, y);
                });
            })
            .collect::<Vec<(i32, i32)>>()
            .into_par_iter()
            .map(|(x, y)| {
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
                return (
                    WorldPos::new(x, y, z),
                    crate::Block {
                        ty: crate::BlockType::Grass,
                        position: ChunkPos::new(x, y, z).to_world_pos(chunk.position),
                    },
                );
            })
            .collect::<Vec<_>>();
        chunk.blocks.extend(chunks);
    }
}
