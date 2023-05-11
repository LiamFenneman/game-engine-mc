use std::ops::RangeInclusive;

use ge_util::coords::CHUNK_SIZE;
use ge_world::{
    gen::{FixedWorldGenerator, WorldGenerator},
    World,
};

const CHUNK_COUNT: (i32, i32) = (2, 2);

#[derive(Debug, Clone)]
pub struct TestRenderer(World);

impl TestRenderer {
    pub fn render(&self, z_range: RangeInclusive<i32>) {
        for z in z_range {
            for y in 0..(CHUNK_COUNT.1 * CHUNK_SIZE) {
                for x in 0..(CHUNK_COUNT.0 * CHUNK_SIZE) {
                    if let Some((_, block)) = self
                        .0
                        .chunks
                        .iter()
                        .flat_map(|chunk| &chunk.blocks)
                        .find(|(p, _)| p.x() == x && p.y() == y && p.z() == z)
                    {
                        print!("{}", block.ty);
                    }
                }
                println!();
            }
            println!();
        }
    }
}

fn main() {
    let noise_field = ge_world::noise::NoiseField::new(0, 5, 1.0, 10.0, 2.0, 0.5);
    let world = FixedWorldGenerator {
        noise_field,
        chunk_count: CHUNK_COUNT,
    }
    .generate();
    TestRenderer(world).render(90..=100);
}
