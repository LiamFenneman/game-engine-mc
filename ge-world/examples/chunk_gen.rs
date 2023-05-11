use std::ops::RangeInclusive;

use cgmath::vec2;
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
    // let mut chunk_gen = NoiseChunkGenerator::default();
    let noise_field = ge_world::noise::NoiseField::new(1, 5, 1.0, 10.0, 2.0, 0.5);
    let mut chunk_gen = NoiseChunkGenerator::with_noise_field(noise_field, 100);
    let mut sea_level = ge_world::sea_level::SeaLevel::new(95);
    let renderer = TestRenderer::new(
        chunk_gen
            .generate(vec2(0, 0))
            .apply_transformation(&mut sea_level)
            .apply_transformation(&mut ge_world::surface_painting::SimpleSurfacePainter),
    );
    renderer.render(90..=100);
}
