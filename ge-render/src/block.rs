use crate::renderer::Vertex;
use bytemuck::{Pod, Zeroable};
use nalgebra::{Vector2, Vector3};

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

#[derive(Debug, Clone, Copy)]
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
        let top = Face::new(FaceDirection::Top);
        let bottom = Face::new(FaceDirection::Bottom);
        let right = Face::new(FaceDirection::Right);
        let left = Face::new(FaceDirection::Left);
        let front = Face::new(FaceDirection::Front);
        let back = Face::new(FaceDirection::Back);

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

        for (face_counter, _) in self.faces.iter().enumerate() {
            indices.extend_from_slice(&Face::get_indices(u16::try_from(face_counter).unwrap_or(0)));
        }

        return indices;
    }
}

#[derive(Debug, Clone, Copy)]
enum FaceDirection {
    Top,
    Bottom,
    Left,
    Right,
    Front,
    Back,
}

macro_rules! vert {
    ($x:literal $y:literal $z:literal, $u:literal $v:literal, $i:literal) => {
        BlockVertex::new(
            nalgebra::Vector3::new($x, $y, $z),
            nalgebra::Vector2::new($u, $v),
            $i,
        )
    };
}

impl FaceDirection {
    pub fn get_vertices(self) -> [BlockVertex; 4] {
        return match self {
            FaceDirection::Top => [
                vert!(1.0 0.0 1.0, 1.0 0.0, 0),
                vert!(1.0 1.0 1.0, 1.0 1.0, 0),
                vert!(0.0 1.0 1.0, 0.0 1.0, 0),
                vert!(0.0 0.0 1.0, 0.0 0.0, 0),
            ],
            FaceDirection::Bottom => [
                vert!(0.0 0.0 0.0, 1.0 1.0, 1),
                vert!(0.0 1.0 0.0, 1.0 0.0, 1),
                vert!(1.0 1.0 0.0, 0.0 0.0, 1),
                vert!(1.0 0.0 0.0, 0.0 1.0, 1),
            ],
            FaceDirection::Left => [
                vert!(0.0 1.0 0.0, 0.0 1.0, 2),
                vert!(0.0 0.0 0.0, 1.0 1.0, 2),
                vert!(0.0 0.0 1.0, 1.0 0.0, 2),
                vert!(0.0 1.0 1.0, 0.0 0.0, 2),
            ],
            FaceDirection::Right => [
                vert!(1.0 1.0 1.0, 1.0 0.0, 3),
                vert!(1.0 0.0 1.0, 0.0 0.0, 3),
                vert!(1.0 0.0 0.0, 0.0 1.0, 3),
                vert!(1.0 1.0 0.0, 1.0 1.0, 3),
            ],
            FaceDirection::Front => [
                vert!(0.0 1.0 0.0, 1.0 1.0, 4),
                vert!(0.0 1.0 1.0, 1.0 0.0, 4),
                vert!(1.0 1.0 1.0, 0.0 0.0, 4),
                vert!(1.0 1.0 0.0, 0.0 1.0, 4),
            ],
            FaceDirection::Back => [
                vert!(0.0 0.0 1.0, 0.0 0.0, 5),
                vert!(0.0 0.0 0.0, 0.0 1.0, 5),
                vert!(1.0 0.0 0.0, 1.0 1.0, 5),
                vert!(1.0 0.0 1.0, 1.0 0.0, 5),
            ],
        };
    }
}

#[derive(Debug, Clone, Copy)]
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

    pub fn get_indices(i: u16) -> [u16; 6] {
        let displacement = i * 4;
        #[allow(clippy::identity_op, reason = "makes the code more readable")]
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
