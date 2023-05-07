use crate::{noise::NoiseField, Block, Chunk};
use cgmath::{vec2, Vector3};

/// A `ChunkGenerator` is a trait that generates a `Chunk`.
pub trait ChunkGenerator {
    /// Generate a block at a specific position.
    fn generate_at(&mut self, chunk_pos: Vector3<u32>) -> Block;

    /// Generate a `Chunk`.
    fn generate(&mut self) -> Chunk {
        let mut blocks = vec![];
        for y in 0..Chunk::SIZE.y {
            for z in 0..Chunk::SIZE.z {
                for x in 0..Chunk::SIZE.x {
                    blocks.push(self.generate_at(Vector3::new(x, y, z)));
                }
            }
        }

        return Chunk {
            blocks,
            position: None,
        };
    }
}

pub struct NoiseChunkGenerator {
    pub noise_field: NoiseField,
    base_y: u32,
}

impl ChunkGenerator for NoiseChunkGenerator {
    #[allow(clippy::cast_lossless, clippy::cast_sign_loss)]
    fn generate_at(&mut self, position: Vector3<u32>) -> Block {
        let sample_y = self.noise_field.sample_2d(
            vec2(position.x as f64, position.z as f64),
            None,
            Some(vec2(Chunk::SIZE.x as f64, Chunk::SIZE.z as f64)),
        );

        let surface_y = (self.base_y as f64 + sample_y) as u32;
        let ty = match surface_y {
            y if position.y > y => crate::BlockType::Air,
            _ => crate::BlockType::Stone,
        };

        return Block { ty, position };
    }
}

impl NoiseChunkGenerator {
    #[must_use]
    pub fn new(
        seed: u64,
        base_y: u32,
        octaves: u8,
        frequency: f64,
        amplitude: f64,
        lacunarity: f64,
        gain: f64,
    ) -> Self {
        let noise_field = NoiseField::new(seed, octaves, frequency, amplitude, lacunarity, gain);
        return Self {
            noise_field,
            base_y,
        };
    }
}

impl Default for NoiseChunkGenerator {
    fn default() -> Self {
        let noise_field = NoiseField::new(rand::random(), 5, 1.0, 10.0, 2.0, 0.5);
        return Self {
            noise_field,
            base_y: 100,
        };
    }
}
