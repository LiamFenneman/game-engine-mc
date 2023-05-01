use crate::{noise::Noise, Block, World};
use cgmath::Vector3;
use rand::Rng;

/// A `WorldGenerator` is a trait that generates a `World`.
pub trait WorldGenerator {
    /// The size of the world to generate.
    fn size(&self) -> Vector3<u32>;

    /// Generate a block at a specific position.
    fn generate_at(&mut self, position: Vector3<u32>) -> Block;

    /// Generate a `World`.
    fn generate(&mut self) -> World {
        let mut blocks = vec![];
        for y in 0..self.size().y {
            for z in 0..self.size().z {
                for x in 0..self.size().x {
                    blocks.push(self.generate_at(Vector3::new(x, y, z)));
                }
            }
        }
        return World {
            blocks,
            size: self.size(),
        };
    }
}

pub struct NoiseWorldGenerator {
    pub noise: Noise,
    pub size: Vector3<u32>,
}

impl WorldGenerator for NoiseWorldGenerator {
    #[allow(clippy::cast_lossless, clippy::cast_sign_loss)]
    fn generate_at(&mut self, position: Vector3<u32>) -> Block {
        let sample_y = self
            .noise
            .sample_2d(cgmath::Vector2::new(position.x as f64, position.z as f64));

        let surface_y = 10 + (sample_y * 20.0) as u32;
        let ty = match surface_y / 10 {
            y if position.y > y => crate::BlockType::Air,
            _ => crate::BlockType::Stone,
        };

        return Block { ty, position };
    }

    fn size(&self) -> Vector3<u32> {
        return self.size;
    }
}

impl Default for NoiseWorldGenerator {
    fn default() -> Self {
        let noise = Noise::new(rand::random(), 5, 1.0, 1.5, 0.0);
        let size = Vector3::new(16, 256, 16);
        return Self { noise, size };
    }
}

pub struct RandomWorldGenerator {
    pub world_size: u32,
    pub rng: rand_chacha::ChaCha8Rng,
}

impl WorldGenerator for RandomWorldGenerator {
    fn generate_at(&mut self, position: Vector3<u32>) -> Block {
        return Block {
            ty: self.rng.gen(),
            position,
        };
    }

    fn size(&self) -> Vector3<u32> {
        return Vector3::new(self.world_size, self.world_size, self.world_size);
    }
}
