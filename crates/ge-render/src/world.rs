use crate::{context::Context, drawables::world::World};
use ge_util::ChunkOffset;
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
    pub fn new(_cx: Context, _state: WorldState) -> Self {
        let (tx, rx) = std::sync::mpsc::channel();
        let handle = std::thread::spawn(move || loop {
            let Ok(val) = rx.recv() else {
                break;
            };
            trace!("received chunk offset: {:?}", val);
        });

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
