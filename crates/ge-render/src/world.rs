use crate::{context::Context, drawables::world::World};
use ge_util::ChunkOffset;
use ge_world::{
    gen::{AsyncWorldGenerator, WorldGenerator},
    noise::Noise,
    trns::{SeaLevel, SimpleSurfacePainter, Transformation},
};
use nalgebra::Vector3;
use std::{
    sync::{mpsc::Sender, Arc, Mutex},
    thread::JoinHandle,
};

#[derive(Debug)]
pub(crate) struct WorldSystem {
    #[allow(dead_code)]
    handle: JoinHandle<()>,
    tx: Sender<ChunkOffset>,
    last_pos: ChunkOffset,
}

pub(crate) type WorldState = Arc<Mutex<World>>;

impl WorldSystem {
    pub fn new(cx: Context, state: WorldState) -> Self {
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
        let mut world_gen = AsyncWorldGenerator::new(noise, count, trns, &cx.config);

        let (tx, rx) = std::sync::mpsc::channel::<ChunkOffset>();
        let handle = std::thread::spawn(move || loop {
            let Ok(val) = rx.recv() else {
                break;
            };
            trace!("received chunk offset: {:?}", val);

            // calculate which chunks to generate
            world_gen.center = (val.x(), val.y());

            // generate new chunks
            let world = world_gen.generate();

            // update the world state
            let mut state = state.lock().unwrap();
            state.chunks = world.chunks;
            state.dirty = true;
        });

        // send the initial position
        tx.send(ChunkOffset::default()).unwrap();

        let last_pos = ChunkOffset::default();

        return Self {
            handle,
            tx,
            last_pos,
        };
    }

    pub fn update(&mut self, camera_pos: Vector3<f32>) {
        let pos = ChunkOffset::from(camera_pos);
        if pos != self.last_pos {
            self.tx.send(pos).unwrap();
            self.last_pos = pos;
        }
    }
}
