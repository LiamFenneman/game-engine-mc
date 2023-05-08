use crate::{
    block::{Block, BlockVertex},
    renderer::{create_render_pipeline, Draw, Renderer, Vertex},
};
use ge_resource::{texture::Texture, ResourceManager};
use std::rc::Rc;
use wgpu::util::DeviceExt;

pub struct DrawBlock {
    render_pipeline: wgpu::RenderPipeline,
    bind_group: Rc<wgpu::BindGroup>,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
}

impl DrawBlock {
    /// Creates a new [`DrawBlock`].
    ///
    /// # Panics
    ///
    /// Panics if a [`Texture`] fails to parse bytes.
    pub fn new(
        renderer: &Renderer,
        resources: &mut ResourceManager,
        block: &Block,
        uniform_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let textures = resources.load_texture_array(
            ge_world::BlockType::Grass,
            &renderer.device,
            &renderer.queue,
        );

        let layout = renderer
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Block Pipeline Layout"),
                bind_group_layouts: &[&textures.bind_group_layout, uniform_bind_group_layout],
                push_constant_ranges: &[],
            });

        let shader = wgpu::include_wgsl!("../shaders/block.wgsl");

        let render_pipeline = create_render_pipeline(
            renderer,
            &layout,
            Some(Texture::DEPTH_FORMAT),
            &[BlockVertex::desc()],
            shader,
        );

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

        return Self {
            render_pipeline,
            bind_group: Rc::clone(&textures.bind_group),
            vertex_buffer,
            index_buffer,
            num_indices,
        };
    }
}

impl Draw for DrawBlock {
    fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>, uniforms: &'a wgpu::BindGroup) {
        render_pass.set_pipeline(&self.render_pipeline);
        render_pass.set_bind_group(0, &self.bind_group, &[]);
        render_pass.set_bind_group(1, uniforms, &[]);

        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
    }
}
