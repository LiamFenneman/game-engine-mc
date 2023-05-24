use ge_world::gen::{AsyncWorldGenerator, FixedWorldGenerator, WorldGenerator};

const CHUNK_COUNT: (i32, i32) = (8, 8);

fn main() {
    let noise = ge_world::noise::Noise::new(0, 5, 1.0 / 16.0, 10.0, 2.0, 0.5);
    let sea_level = ge_world::trns::SeaLevel::new(&Default::default());
    let surface_painter = ge_world::trns::SimpleSurfacePainter;

    rayon::join(
        || {
            let start = std::time::Instant::now();
            let world = AsyncWorldGenerator::new(
                noise,
                CHUNK_COUNT,
                vec![sea_level.into(), surface_painter.into()],
                &ge_util::EngineConfig::default(),
            )
            .generate();
            println!(
                "async: generated {} chunks in {:?}",
                world.chunks.len(),
                start.elapsed()
            );
        },
        || {
            let start = std::time::Instant::now();
            let world = FixedWorldGenerator::new(
                noise,
                CHUNK_COUNT,
                vec![sea_level.into(), surface_painter.into()],
                &ge_util::EngineConfig::default(),
            )
            .generate();
            println!(
                "fixed: generated {} chunks in {:?}",
                world.chunks.len(),
                start.elapsed()
            );
        },
    );
}
