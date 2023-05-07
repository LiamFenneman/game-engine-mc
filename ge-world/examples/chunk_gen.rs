use std::ops::RangeInclusive;

use ge_world::{
    gen::{ChunkGenerator, NoiseChunkGenerator},
    Chunk,
};

#[derive(Debug, Clone)]
pub struct TestRenderer(Chunk);

impl TestRenderer {
    #[must_use]
    pub fn new(chunk: Chunk) -> Self {
        Self(chunk)
    }

    pub fn render(&self, y_range: RangeInclusive<u32>) {
        for y in y_range {
            for z in 0..Chunk::SIZE.z {
                for x in 0..Chunk::SIZE.x {
                    let block = self.0.blocks.iter().find(|block| {
                        block.position.x == x && block.position.y == y && block.position.z == z
                    });
                    match block {
                        Some(block) => print!("{}", block.ty),
                        None => print!(" "),
                    }
                }
                println!();
            }
            println!();
        }
    }
}

fn main() {
    let mut chunk_gen = NoiseChunkGenerator::default();
    let renderer = TestRenderer::new(chunk_gen.generate());
    renderer.render(90..=110);
}
