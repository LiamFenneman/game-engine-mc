use crate::{
    renderer::{create_render_pipeline, Draw, Renderer, Vertex},
    texture::{Texture, TextureArray},
};
use bytemuck::{Pod, Zeroable};
use cgmath::{Vector2, Vector3};
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct BlockVertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
    tex_index: u32,
}

impl BlockVertex {
    #[must_use]
    pub fn new(position: Vector3<f32>, tex_coords: Vector2<f32>, tex_index: u32) -> Self {
        return Self {
            position: [position.x, position.y, position.z],
            tex_coords: [tex_coords.x, tex_coords.y],
            tex_index,
        };
    }
}

impl Vertex for BlockVertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        return wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<BlockVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Uint32,
                },
            ],
        };
    }
}

#[allow(unused)]
pub struct DrawBlock {
    render_pipeline: wgpu::RenderPipeline,
    bind_group: wgpu::BindGroup,
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,
    num_indices: u32,
    block: Block,
}

impl DrawBlock {
    /// Creates a new [`DrawBlock`].
    ///
    /// # Panics
    ///
    /// Panics if a [`Texture`] fails to parse bytes.
    pub fn new(
        renderer: &Renderer,
        block: Block,
        uniform_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> Self {
        let textures = TextureArray::new(
            renderer,
            vec![
                Texture::from_bytes(
                    &renderer.device,
                    &renderer.queue,
                    include_bytes!("../../assets/grass_0.png"),
                    "block",
                    false,
                )
                .unwrap(),
                Texture::from_bytes(
                    &renderer.device,
                    &renderer.queue,
                    include_bytes!("../../assets/grass_1.png"),
                    "block",
                    false,
                )
                .unwrap(),
                Texture::from_bytes(
                    &renderer.device,
                    &renderer.queue,
                    include_bytes!("../../assets/grass_2.png"),
                    "block",
                    false,
                )
                .unwrap(),
            ],
            "block_texture",
        );

        let layout = renderer
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Block Pipeline Layout"),
                bind_group_layouts: &[&textures.bind_group_layout, uniform_bind_group_layout],
                push_constant_ranges: &[],
            });

        let shader = wgpu::ShaderModuleDescriptor {
            label: Some("Block Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        };

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

        let num_indices = block.get_indices().len() as u32;

        return Self {
            render_pipeline,
            bind_group: textures.bind_group,
            vertex_buffer,
            index_buffer,
            num_indices,
            block,
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

pub struct Block {
    faces: [Face; 6],
}

impl Default for Block {
    fn default() -> Self {
        return Self::new();
    }
}

impl Block {
    #[must_use]
    pub fn new() -> Self {
        let faces = Self::generate_faces();

        return Self { faces };
    }

    fn generate_faces() -> [Face; 6] {
        let top = Face::new(FaceDirection::TOP);
        let bottom = Face::new(FaceDirection::BOTTOM);
        let right = Face::new(FaceDirection::RIGHT);
        let left = Face::new(FaceDirection::LEFT);
        let front = Face::new(FaceDirection::FRONT);
        let back = Face::new(FaceDirection::BACK);

        return [top, bottom, right, left, front, back];
    }

    #[must_use]
    pub fn get_vertices(&self) -> Vec<BlockVertex> {
        let mut vertices = Vec::new();

        for face in &self.faces {
            vertices.extend_from_slice(&face.vertices);
        }

        return vertices;
    }

    #[must_use]
    pub fn get_indices(&self) -> Vec<u16> {
        let mut indices = Vec::new();

        for (face_counter, face) in self.faces.iter().enumerate() {
            indices.extend_from_slice(&face.get_indices(face_counter as u16));
        }

        return indices;
    }
}

#[allow(clippy::upper_case_acronyms)]
enum FaceDirection {
    TOP,
    BOTTOM,
    RIGHT,
    LEFT,
    FRONT,
    BACK,
}

impl FaceDirection {
    pub fn get_vertices(&self) -> [BlockVertex; 4] {
        use cgmath::{vec2, vec3};
        return match self {
            FaceDirection::TOP => [
                BlockVertex::new(vec3(0.0, 1.0, 0.0), vec2(0.0, 0.0), 0),
                BlockVertex::new(vec3(0.0, 1.0, 1.0), vec2(0.0, 1.0), 0),
                BlockVertex::new(vec3(1.0, 1.0, 1.0), vec2(1.0, 1.0), 0),
                BlockVertex::new(vec3(1.0, 1.0, 0.0), vec2(1.0, 0.0), 0),
            ],
            FaceDirection::BOTTOM => [
                BlockVertex::new(vec3(0.0, 0.0, 1.0), vec2(0.0, 0.0), 2),
                BlockVertex::new(vec3(0.0, 0.0, 0.0), vec2(0.0, 1.0), 2),
                BlockVertex::new(vec3(1.0, 0.0, 0.0), vec2(1.0, 1.0), 2),
                BlockVertex::new(vec3(1.0, 0.0, 1.0), vec2(1.0, 0.0), 2),
            ],
            FaceDirection::RIGHT => [
                BlockVertex::new(vec3(1.0, 1.0, 1.0), vec2(0.0, 0.0), 1),
                BlockVertex::new(vec3(1.0, 0.0, 1.0), vec2(0.0, 1.0), 1),
                BlockVertex::new(vec3(1.0, 0.0, 0.0), vec2(1.0, 1.0), 1),
                BlockVertex::new(vec3(1.0, 1.0, 0.0), vec2(1.0, 0.0), 1),
            ],
            FaceDirection::LEFT => [
                BlockVertex::new(vec3(0.0, 1.0, 0.0), vec2(0.0, 0.0), 1),
                BlockVertex::new(vec3(0.0, 0.0, 0.0), vec2(0.0, 1.0), 1),
                BlockVertex::new(vec3(0.0, 0.0, 1.0), vec2(1.0, 1.0), 1),
                BlockVertex::new(vec3(0.0, 1.0, 1.0), vec2(1.0, 0.0), 1),
            ],
            FaceDirection::FRONT => [
                BlockVertex::new(vec3(0.0, 1.0, 1.0), vec2(0.0, 0.0), 1),
                BlockVertex::new(vec3(0.0, 0.0, 1.0), vec2(0.0, 1.0), 1),
                BlockVertex::new(vec3(1.0, 0.0, 1.0), vec2(1.0, 1.0), 1),
                BlockVertex::new(vec3(1.0, 1.0, 1.0), vec2(1.0, 0.0), 1),
            ],
            FaceDirection::BACK => [
                BlockVertex::new(vec3(1.0, 1.0, 0.0), vec2(0.0, 0.0), 1),
                BlockVertex::new(vec3(1.0, 0.0, 0.0), vec2(0.0, 1.0), 1),
                BlockVertex::new(vec3(0.0, 0.0, 0.0), vec2(1.0, 1.0), 1),
                BlockVertex::new(vec3(0.0, 1.0, 0.0), vec2(1.0, 0.0), 1),
            ],
        };
    }
}

#[allow(unused)]
struct Face {
    vertices: [BlockVertex; 4],
    direction: FaceDirection,
}

impl Face {
    pub fn new(direction: FaceDirection) -> Self {
        return Self {
            vertices: direction.get_vertices(),
            direction,
        };
    }

    #[allow(clippy::identity_op, clippy::unused_self)]
    pub fn get_indices(&self, i: u16) -> [u16; 6] {
        let displacement = i * 4;
        return [
            0 + displacement,
            1 + displacement,
            2 + displacement,
            2 + displacement,
            3 + displacement,
            0 + displacement,
        ];
    }
}
