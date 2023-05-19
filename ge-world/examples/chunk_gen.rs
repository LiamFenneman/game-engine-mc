use ge_util::{coords::CHUNK_SIZE, ChunkOffset};
use ge_world::{
    gen::{ChunkGenerator, NoiseChunkGenerator},
    Chunk,
};
use std::ops::RangeInclusive;

#[derive(Debug, Clone)]
pub struct TestRenderer(Chunk);

impl TestRenderer {
    #[must_use]
    pub fn new(chunk: Chunk) -> Self {
        Self(chunk)
    }

    pub fn render(&self, z_range: RangeInclusive<i32>) {
        for z in z_range {
            for y in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    if let Some((_, block)) = self
                        .0
                        .blocks
                        .iter()
                        .find(|(p, _)| p.x() == x && p.y() == y && p.z() == z)
                    {
                        print!("{}", block.ty())
                    }
                }
                println!();
            }
            println!();
        }
    }
}

fn main() {
    let noise = ge_world::noise::Noise::new(5, 1.0, 10.0, 2.0, 0.5);
    let mut chunk_gen = NoiseChunkGenerator::with_noise(noise, 100);
    let renderer = TestRenderer::new(
        chunk_gen
            .generate(ChunkOffset::default())
            .apply_transformation(&ge_world::sea_level::SeaLevel::new(95))
            .apply_transformation(&ge_world::surface_painting::SimpleSurfacePainter),
    );
    renderer.render(90..=100);
}
