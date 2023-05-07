use std::ops::RangeInclusive;

use cgmath::{vec2, Vector2};
use ge_world::{
    gen::{FixedWorldGenerator, WorldGenerator},
    Chunk, World,
};

const CHUNK_COUNT: Vector2<u32> = vec2(2, 2);

#[derive(Debug, Clone)]
pub struct TestRenderer(World);

impl TestRenderer {
    pub fn render(&self, y_range: RangeInclusive<u32>) {
        for y in y_range {
            for z in 0..(CHUNK_COUNT.y * Chunk::SIZE.z) {
                for x in 0..(CHUNK_COUNT.x * Chunk::SIZE.x) {
                    let block =
                        self.0
                            .chunks
                            .iter()
                            .flat_map(|chunk| &chunk.blocks)
                            .find(|block| {
                                block.position.x == x
                                    && block.position.y == y
                                    && block.position.z == z
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
    TestRenderer(world).render(90..=110);
}
