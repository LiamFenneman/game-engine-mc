use std::collections::HashMap;

use crate::{
    drawables::chunk::DrawChunk,
    renderer::{Draw, Renderer},
};
use cgmath::Vector2;
use ge_resource::ResourceManager;
use ge_util::ChunkOffset;
use ge_world::{
    gen::{FixedWorldGenerator, WorldGenerator},
    noise::NoiseField,
    sea_level::SeaLevel,
    surface_painting::SimpleSurfacePainter,
    Chunk,
};

const RENDER_DISTANCE: i32 = 2;

pub struct World {
    position: Vector2<i32>,
    world_gen: FixedWorldGenerator,
    instances: HashMap<ChunkOffset, DrawChunk>,
    dirty: bool,
}

impl World {
    #[must_use]
    pub fn new(position: Vector2<i32>) -> Self {
        let chunk_count = (RENDER_DISTANCE, RENDER_DISTANCE);
        let noise_field = NoiseField::new(0, 5, 1.0, 10.0, 2.0, 0.5);
        let sea_level = Box::new(SeaLevel::new(90));
        let surface_painter = Box::new(SimpleSurfacePainter);
        let world_gen = FixedWorldGenerator::with_transformations(
            noise_field,
            chunk_count,
            vec![sea_level, surface_painter],
        );
        let instances = HashMap::with_capacity((RENDER_DISTANCE as usize).pow(2));

        return Self {
            position,
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
        new_pos: cgmath::Point3<f32>,
        renderer: &Renderer,
        resources: &mut ResourceManager,
        uniform_bind_group_layout: &wgpu::BindGroupLayout,
    ) {
        self.position = world_pos_to_chunk_pos(new_pos);
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
                    self.create_instance(chunk, renderer, resources, uniform_bind_group_layout),
                );
            })
            .collect::<HashMap<_, _>>();
        self.instances.extend(instances);

        self.dirty = false;
    }

    pub fn create_instance(
        &mut self,
        chunk: Chunk,
        renderer: &Renderer,
        resources: &mut ResourceManager,
        uniform_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> DrawChunk {
        return DrawChunk::with_chunk(chunk, renderer, resources, uniform_bind_group_layout);
    }
}

impl Draw for World {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, uniforms: &'a wgpu::BindGroup) {
        self.instances
            .iter()
            .for_each(|(_, d)| d.draw(render_pass, uniforms));
    }
}

fn world_pos_to_chunk_pos(pos: cgmath::Point3<f32>) -> Vector2<i32> {
    #[allow(
        clippy::cast_possible_truncation,
        reason = "we don't need exact position"
    )]
    return Vector2::new((pos.x / 16.0).floor() as i32, (pos.y / 16.0).floor() as i32);
}
