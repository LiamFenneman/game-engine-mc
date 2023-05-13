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

        for y in 0..=2 {
            for x in 0..=2 {
                self.chunks.entry((x, y)).or_insert_with(|| {
                    return create_chunk((x, y), renderer, resources, uniform_bind_group_layout);
                });
            }
        }
    }
}

fn create_chunk(
    offset: (i32, i32),
    renderer: &Renderer,
    resources: &mut ResourceManager,
    uniform_bind_group_layout: &wgpu::BindGroupLayout,
) -> DrawChunk {
    let offset = ChunkOffset::new(offset.0, offset.1, 0).unwrap();
    return DrawChunk::with_offset(offset, renderer, resources, uniform_bind_group_layout);
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
    return Vector2::new((pos.x / 16.0).floor() as i32, (pos.y / 16.0).floor() as i32);
}
