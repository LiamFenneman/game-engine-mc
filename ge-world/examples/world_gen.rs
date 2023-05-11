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
                    let block =
                        self.0
                            .chunks
                            .iter()
                            .flat_map(|chunk| &chunk.blocks)
                            .find(|block| {
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
    let noise_field = ge_world::noise::NoiseField::new(0, 5, 1.0, 10.0, 2.0, 0.5);
    let world = FixedWorldGenerator {
        noise_field,
        chunk_count: CHUNK_COUNT,
    }
    .generate();
    TestRenderer(world).render(90..=100);
}
