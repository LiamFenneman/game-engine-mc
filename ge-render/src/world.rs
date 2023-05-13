use crate::{
    drawables::chunk::DrawChunk,
    renderer::{Draw, Renderer},
};
use cgmath::Vector2;
use ge_resource::ResourceManager;
use ge_util::ChunkOffset;
use std::collections::BTreeMap;

// const RENDER_DISTANCE: u32 = 2;

pub struct World {
    pub position: Vector2<i32>,
    pub chunks: BTreeMap<(i32, i32), DrawChunk>,
}

impl World {
    #[must_use]
    pub fn new(position: Vector2<i32>) -> Self {
        return Self {
            position,
            chunks: BTreeMap::new(),
        };
    }

    pub fn update(
        &mut self,
        new_pos: cgmath::Point3<f32>,
        renderer: &Renderer,
        resources: &mut ResourceManager,
        uniform_bind_group_layout: &wgpu::BindGroupLayout,
    ) {
        let new_pos = world_pos_to_chunk_pos(new_pos);
        self.position = new_pos;

        self.chunks.entry((0, 0)).or_insert_with(|| {
            let offset = ChunkOffset::default();
            return {
                DrawChunk::with_offset(offset, renderer, resources, uniform_bind_group_layout)
            };
        });
    }
}

impl Draw for World {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, uniforms: &'a wgpu::BindGroup) {
        self.chunks
            .iter()
            .for_each(|(_, c)| c.draw(render_pass, uniforms));
    }
}

fn world_pos_to_chunk_pos(pos: cgmath::Point3<f32>) -> Vector2<i32> {
    #[allow(
        clippy::cast_possible_truncation,
        reason = "we don't need exact position"
    )]
    return Vector2::new((pos.x / 16.0).floor() as i32, (pos.z / 16.0).floor() as i32);
}
