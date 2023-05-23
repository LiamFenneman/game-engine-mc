use crate::{
    block::{Block, BlockVertex},
    context::Context,
    renderer::{create_render_pipeline, Draw, Renderer, Vertex},
};
use ge_resource::{
    texture::{Texture, TextureArray},
    ResourceManager,
};
use ge_world::{BlockType, Chunk};
use nalgebra::{Matrix4, Vector3};
use std::{collections::HashSet, sync::Arc};
use wgpu::util::DeviceExt;

#[derive(Debug)]
pub(crate) struct DrawChunk {
    instances: Vec<DrawInstancedBlocks>,
    #[allow(dead_code, reason = "TODO: add to debug info")]
    chunk: Chunk,
}

impl DrawChunk {
    pub fn new(
        cx: Context,
        chunk: Chunk,
        renderer: &Renderer,
        resources: &mut ResourceManager,
    ) -> Self {
        let config = cx.lock().config;
        let visible = chunk.visible_blocks(&config);

        // block types present in the chunk
        let present_blk_ty = visible
            .iter()
            .map(|blk| return blk.ty())
            .filter(|&ty| return ty != BlockType::Air)
            .collect::<HashSet<BlockType>>();

        // create instance buffer for each block type
        let mut instances: Vec<DrawInstancedBlocks> = Vec::with_capacity(present_blk_ty.len());
        for ty in present_blk_ty {
            let textures = resources.load_texture_array(ty, &renderer.device, &renderer.queue);
            let blocks = visible
                .iter()
                .filter(|blk| return blk.ty() == ty)
                .map(|blk| return **blk)
                .collect::<Vec<_>>();
            instances.push(DrawInstancedBlocks::new(
                cx.clone(),
                renderer,
                &blocks,
                textures,
            ));
        }

        return Self { instances, chunk };
    }
}

impl Draw for DrawChunk {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, uniforms: &'a wgpu::BindGroup) {
        self.instances.iter().for_each(|i| {
            i.draw(render_pass, uniforms);
        });
    }
}

#[derive(Debug)]
pub(crate) struct DrawInstancedBlocks {
    render_pipeline: wgpu::RenderPipeline,
    bind_group: Arc<wgpu::BindGroup>,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    instance_buffer: wgpu::Buffer,
    num_instances: u32,
}

impl DrawInstancedBlocks {
    pub fn new(
        cx: Context,
        renderer: &Renderer,
        blocks: &[ge_world::Block],
        textures: &TextureArray,
    ) -> Self {
        let block = Block::new();
        let vertex_buffer = renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&block.get_vertices()),
                usage: wgpu::BufferUsages::VERTEX,
            });
        let index_buffer = renderer
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(&block.get_indices()),
                usage: wgpu::BufferUsages::INDEX,
            });

        let num_indices = u32::try_from(block.get_indices().len()).unwrap_or_default();

        let layout = renderer
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Block Pipeline Layout"),
                bind_group_layouts: &[
                    &textures.bind_group_layout,
                    &cx.lock().uniform_bind_group_layout,
                ],
                push_constant_ranges: &[],
            });

        let shader = wgpu::include_wgsl!("../shaders/block.wgsl");

        let render_pipeline = create_render_pipeline(
            renderer,
            &layout,
            Some(Texture::DEPTH_FORMAT),
            &[BlockVertex::desc(), InstanceRaw::desc()],
            shader,
            &cx.lock().config,
        );

        let instances = blocks
            .iter()
            .filter(|&b| return b.ty() != ge_world::BlockType::Air)
            .map(|&b| {
                #[allow(clippy::cast_precision_loss, reason = "no other way")]
                return Instance {
                    position: Vector3::new(
                        b.world_pos().x() as f32,
                        b.world_pos().y() as f32,
                        b.world_pos().z() as f32,
                    ),
                };
            })
            .collect::<Vec<_>>();

        let instance_data: Vec<InstanceRaw> = instances.iter().map(|&i| return i.into()).collect();
        let instance_buffer =
            renderer
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("Instance Buffer"),
                    contents: bytemuck::cast_slice(&instance_data),
                    usage: wgpu::BufferUsages::VERTEX,
                });
        let num_instances = u32::try_from(instances.len()).unwrap_or_default();

        return Self {
            render_pipeline,
            bind_group: Arc::clone(&textures.bind_group),
            vertex_buffer,
            index_buffer,
            num_indices,
            instance_buffer,
            num_instances,
        };
    }
}

impl Draw for DrawInstancedBlocks {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, uniforms: &'a wgpu::BindGroup) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_bind_group(1, uniforms, &[]);

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_vertex_buffer(1, self.instance_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..self.num_instances);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Instance {
    position: Vector3<f32>,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct InstanceRaw {
    model: [[f32; 4]; 4],
}

impl InstanceRaw {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        return wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<InstanceRaw>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 7,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 8,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        };
    }
}

impl From<Instance> for InstanceRaw {
    fn from(value: Instance) -> Self {
        return Self {
            model: Matrix4::new_translation(&value.position).into(),
        };
    }
}
