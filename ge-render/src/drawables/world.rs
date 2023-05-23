use crate::{
    context::Context,
    drawables::chunk::DrawChunk,
    renderer::{Draw, Renderer},
};
use ge_resource::ResourceManager;
use ge_util::ChunkOffset;
use ge_world::{
    gen::{FixedWorldGenerator, WorldGenerator},
    noise::Noise,
    trns::{SeaLevel, SimpleSurfacePainter, Transformation},
};
use nalgebra::Vector3;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct World {
    context: Context,
    camera_position: ChunkOffset,
    world_gen: FixedWorldGenerator,
    instances: HashMap<ChunkOffset, DrawChunk>,
    dirty: bool,
}

impl World {
    #[must_use]
    pub fn new(cx: Context, camera_position: ChunkOffset) -> Self {
        let config = cx.lock().config;
        let count = {
            #[allow(
                clippy::cast_possible_wrap,
                clippy::cast_possible_truncation,
                reason = "value should not be large enought to wrap or truncate"
            )]
            let rd = config.world_gen.render_distance as i32;
            (rd, rd)
        };
        let noise = Noise::from(&config);
        let trns: Vec<Transformation> =
            vec![SeaLevel::new(&config).into(), SimpleSurfacePainter.into()];
        let world_gen = FixedWorldGenerator::new(noise, count, trns, &config);

        let instances = HashMap::with_capacity((config.world_gen.render_distance).pow(2));

        return Self {
            context: cx,
            camera_position,
            world_gen,
            instances,
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

        let instances = self
            .world_gen
            .generate()
            .chunks
            .into_iter()
            .map(|chunk| {
                return (
                    chunk.position,
                    DrawChunk::new(self.context.clone(), chunk, renderer, resources),
                );
            })
            .collect::<HashMap<_, _>>();
        self.instances.extend(instances);

        self.dirty = false;
    }
}

impl Draw for World {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, uniforms: &'a wgpu::BindGroup) {
        self.instances
            .iter()
            .for_each(|(_, d)| d.draw(render_pass, uniforms));
    }
}
