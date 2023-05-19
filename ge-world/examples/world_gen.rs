use ge_util::coords::CHUNK_SIZE;
use ge_world::{
    gen::{FixedWorldGenerator, WorldGenerator},
    World,
};
use std::ops::RangeInclusive;

const CHUNK_COUNT: (i32, i32) = (2, 2);

#[derive(Debug, Clone)]
pub struct TestRenderer(World);

impl TestRenderer {
    pub fn render(&self, z_range: RangeInclusive<i32>) {
        let blocks = self.0.into_world_blocks();
        let n = CHUNK_COUNT.1 * CHUNK_SIZE;
        let m = CHUNK_COUNT.0 * CHUNK_SIZE;
        for z in z_range {
            for y in -n..n {
                for x in -m..m {
                    if let Some(blk) = blocks.iter().find(|blk| {
                        blk.world_pos().x() == x
                            && blk.world_pos().y() == y
                            && blk.world_pos().z() == z
                    }) {
                        print!("{}", blk.ty());
                    }
                }
                println!();
            }
            println!();
        }
    }
}

fn main() {
    let noise = ge_world::noise::Noise::new(5, 1.0 / 16.0, 10.0, 2.0, 0.5);
    let sea_level = Box::new(ge_world::trns::SeaLevel::new(95));
    let surface_painter = Box::new(ge_world::trns::SimpleSurfacePainter);
    let world = FixedWorldGenerator::new(
        noise,
        CHUNK_COUNT,
        vec![sea_level, surface_painter],
        &ge_util::EngineConfig::default(),
    )
    .generate();
    TestRenderer(world).render(90..=90);
}
