use crate::{
    context::Context,
    drawables::chunk::DrawChunk,
    renderer::{Draw, Renderer},
};
use ge_resource::ResourceManager;
use ge_util::ChunkOffset;
use ge_world::Chunk;
use nalgebra::Vector3;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct DrawWorld {
    context: Context,
    camera_position: ChunkOffset,
    instances: HashMap<ChunkOffset, DrawChunk>,
    pub chunks: Vec<Chunk>,
    pub dirty: bool,
}

impl DrawWorld {
    #[must_use]
    pub fn new(cx: Context, camera_position: ChunkOffset) -> Self {
        let config = cx.lock().config;
        let cap = (config.world_gen.render_distance).pow(2);
        let instances = HashMap::with_capacity(cap);
        let chunks = Vec::with_capacity(cap);

        return Self {
            context: cx,
            camera_position,
            instances,
            chunks,
            dirty: true,
        };
    }

    /// Update the world!
    ///
    /// # Panics
    /// If the chunk offset is invalid.
    pub fn update(
        &mut self,
        new_pos: Vector3<f32>,
        renderer: &Renderer,
        resources: &mut ResourceManager,
    ) {
        let last_pos = self.camera_position;
        self.camera_position = ChunkOffset::from(new_pos);

        if last_pos != self.camera_position {
            trace!("camera position changed: {:?}", self.camera_position);
        }

        if !self.dirty {
            // TODO: replace self.dirty with a check if the pos is the same chunk as last frame
            return;
        }

        dbg_time! {
        self.instances = self
            .chunks
            .iter()
            .map(|chunk| {
                return (
                    chunk.position,
                    DrawChunk::new(self.context.clone(), chunk, renderer, resources),
                );
            })
            .collect::<HashMap<_, _>>();
        }

        self.dirty = false;
    }
}

impl Draw for DrawWorld {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, uniforms: &'a wgpu::BindGroup) {
        self.instances
            .iter()
            .for_each(|(_, d)| d.draw(render_pass, uniforms));
    }
}
