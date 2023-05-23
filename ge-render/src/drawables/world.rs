use crate::{
    drawables::chunk::DrawChunk,
    renderer::{Draw, Renderer},
};
use ge_resource::ResourceManager;
use ge_util::{ChunkOffset, EngineConfig};
use ge_world::{
    gen::{FixedWorldGenerator, WorldGenerator},
    noise::Noise,
    trns::{SeaLevel, SimpleSurfacePainter, Transformation},
    Chunk,
};
use nalgebra::Vector3;
use std::collections::HashMap;

#[derive(Debug)]
pub(crate) struct World {
    camera_position: ChunkOffset,
    world_gen: FixedWorldGenerator,
    instances: HashMap<ChunkOffset, DrawChunk>,
    dirty: bool,
}

impl World {
    #[must_use]
    pub fn new(camera_position: ChunkOffset, config: &EngineConfig) -> Self {
        let count = {
            #[allow(
                clippy::cast_possible_wrap,
                clippy::cast_possible_truncation,
                reason = "value should not be large enought to wrap or truncate"
            )]
            let rd = config.world_gen.render_distance as i32;
            (rd, rd)
        };
        let noise = Noise::from(config);
        let trns: Vec<Transformation> =
            vec![SeaLevel::new(config).into(), SimpleSurfacePainter.into()];
        let world_gen = FixedWorldGenerator::new(noise, count, trns, config);

        let instances = HashMap::with_capacity((config.world_gen.render_distance).pow(2));

        return Self {
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
        uniform_bind_group_layout: &wgpu::BindGroupLayout,
        config: &ge_util::EngineConfig,
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
                    create_instance(
                        chunk,
                        renderer,
                        resources,
                        uniform_bind_group_layout,
                        config,
                    ),
                );
            })
            .collect::<HashMap<_, _>>();
        self.instances.extend(instances);

        self.dirty = false;
    }
}

fn create_instance(
    chunk: Chunk,
    renderer: &Renderer,
    resources: &mut ResourceManager,
    uniform_bind_group_layout: &wgpu::BindGroupLayout,
    config: &ge_util::EngineConfig,
) -> DrawChunk {
    return DrawChunk::new(
        chunk,
        renderer,
        resources,
        uniform_bind_group_layout,
        config,
    );
}

impl Draw for World {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, uniforms: &'a wgpu::BindGroup) {
        self.instances
            .iter()
            .for_each(|(_, d)| d.draw(render_pass, uniforms));
    }
}
