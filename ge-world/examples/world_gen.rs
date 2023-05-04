use std::ops::RangeInclusive;

use cgmath::Vector3;
use ge_world::{
    gen::{NoiseWorldGenerator, RandomWorldGenerator, WorldGenerator},
    util, World,
};
use rand::SeedableRng;

const WORLD_SIZE: u32 = 8;

#[derive(Debug, Clone)]
pub struct TestRenderer(World);

impl TestRenderer {
    #[must_use]
    pub fn new(world: World) -> Self {
        Self(world)
    }

    pub fn render(&self, y_range: RangeInclusive<u32>) {
        for y in y_range {
            for z in 0..self.0.size.z {
                for x in 0..self.0.size.x {
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
    #[allow(unused)]
    let mut world_gen = RandomWorldGenerator {
        world_size: WORLD_SIZE,
        rng: rand_chacha::ChaCha8Rng::seed_from_u64(0),
    };
    let mut world_gen = NoiseWorldGenerator::default();
    let renderer = TestRenderer::new(world_gen.generate());
    renderer.render(90..=110);

    let p = Vector3::new(1, 0, 1);
    let i = util::pos_to_idx(p, WORLD_SIZE);
    let t = renderer.0.blocks[i].ty;
    println!("pos = {p:?} -> idx = {i:?} -> {t}");

    let i = (WORLD_SIZE * WORLD_SIZE - 1) as usize;
    let p = util::idx_to_pos(i, WORLD_SIZE);
    let t = renderer.0.blocks[i].ty;
    println!("idx = {i:?} -> pos = {p:?} -> {t}");
}
