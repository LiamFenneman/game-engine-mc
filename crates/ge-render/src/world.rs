use crate::{context::Context, drawables::world::DrawWorld};
use ge_util::ChunkOffset;
use ge_world::{
    gen::{AsyncWorldGenerator, WorldGenerator},
    noise::Noise,
    trns::{SeaLevel, SimpleSurfacePainter, Transformation},
};
use nalgebra::Vector3;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub(crate) struct WorldSystem {
    pool: rayon::ThreadPool,
    state: WorldState,
    world_gen: AsyncWorldGenerator,
    last_pos: ChunkOffset,
}

pub(crate) type WorldState = Arc<Mutex<DrawWorld>>;

impl WorldSystem {
    pub fn new(cx: Context, state: WorldState) -> Self {
        let num_cpus = num_cpus::get();
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(num_cpus)
            .build()
            .unwrap();

        let cx = cx.lock();
        let count = {
            #[allow(
                clippy::cast_possible_wrap,
                clippy::cast_possible_truncation,
                reason = "value should not be large enought to wrap or truncate"
            )]
            let rd = cx.config.world_gen.render_distance as i32;
            (rd, rd)
        };
        let noise = Noise::from(&cx.config);
        let trns: Vec<Transformation> = vec![
            SeaLevel::new(&cx.config).into(),
            SimpleSurfacePainter.into(),
        ];
        let world_gen = AsyncWorldGenerator::new(noise, count, trns, &cx.config);
        let last_pos = ChunkOffset::default();
        return Self {
            pool,
            state,
            world_gen,
            last_pos,
        };
    }

    pub fn update(&mut self, camera_pos: Vector3<f32>) {
        let pos = ChunkOffset::from(camera_pos);
        if pos != self.last_pos {
            self.pool.install(|| {
                let val = pos;
                trace!("received chunk offset: {:?}", val);

                // calculate which chunks to generate
                self.world_gen.center = (val.x(), val.y());

                // generate new chunks
                let world = self.world_gen.generate();

                // update the world state
                let mut state = self.state.lock().unwrap();
                state.chunks = world.chunks;
                state.dirty = true;
            });
            self.last_pos = pos;
        }
    }
}
