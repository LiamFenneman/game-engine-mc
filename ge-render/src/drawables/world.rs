use crate::{
    block::{Block, BlockVertex},
    renderer::{create_render_pipeline, Draw, Renderer, Vertex},
};
use cgmath::vec3;
use ge_resource::{texture::Texture, ResourceManager};
use ge_world::gen::{NoiseWorldGenerator, WorldGenerator};
use std::rc::Rc;
use wgpu::util::DeviceExt;

pub struct DrawWorld {
    render_pipeline: wgpu::RenderPipeline,
    bind_group: Rc<wgpu::BindGroup>,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    instance_buffer: wgpu::Buffer,
    num_instances: u32,
}

impl DrawWorld {
    /// Creates a new [`DrawWorld`].
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
            ge_world::BlockType::Stone,
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
            &[BlockVertex::desc(), InstanceRaw::desc()],
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

        let num_indices = block.get_indices().len() as u32;

        // instances

        let world = NoiseWorldGenerator::default().generate();
        let instances = world
            .blocks
            .iter()
            .filter(|&b| return b.position.y > 84)
            .filter(|&b| return b.ty != ge_world::BlockType::Air)
            .map(|&b| {
                let (x, y, z) = b.position.into();
                #[allow(clippy::cast_precision_loss)]
                return Instance {
                    position: vec3(x as f32, y as f32 - 84.0, z as f32),
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
        let num_instances = instances.len() as u32;

        return Self {
            render_pipeline,
            bind_group: Rc::clone(&textures.bind_group),
            vertex_buffer,
            index_buffer,
            num_indices,
            instance_buffer,
            num_instances,
        };
    }
}

impl Draw for DrawWorld {
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

#[derive(Copy, Clone)]
pub struct Instance {
    position: cgmath::Vector3<f32>,
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
            model: cgmath::Matrix4::from_translation(value.position).into(),
        };
    }
}
