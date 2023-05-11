use std::ops::RangeInclusive;
use ge_util::coords::CHUNK_SIZE;
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

    pub fn render(&self, z_range: RangeInclusive<i32>) {
        for z in z_range {
            for y in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    let block = self.0.blocks.iter().find(|block| {
                        block.position.x() == x
                            && block.position.y() == y
                            && block.position.z() == z
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
            .generate((0, 0, 0))
            .apply_transformation(&mut sea_level)
            .apply_transformation(&mut ge_world::surface_painting::SimpleSurfacePainter),
    );
    renderer.render(90..=100);
}
