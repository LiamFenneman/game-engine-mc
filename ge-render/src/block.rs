use crate::renderer::Vertex;
use bytemuck::{Pod, Zeroable};
use cgmath::{Vector2, Vector3};

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
