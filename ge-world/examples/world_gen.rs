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
    let noise_field = ge_world::noise::NoiseField::new(0, 5, 1.0, 10.0, 2.0, 0.5);
    let sea_level = Box::new(ge_world::sea_level::SeaLevel::new(95));
    let surface_painter = Box::new(ge_world::surface_painting::SimpleSurfacePainter);
    let world = FixedWorldGenerator::with_transformations(
        noise_field,
        CHUNK_COUNT,
        vec![sea_level, surface_painter],
    )
    .generate();
    TestRenderer(world).render(90..=90);
}
