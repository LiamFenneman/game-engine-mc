use crate::{noise::NoiseField, Block, Chunk, World};
use cgmath::{vec2, vec3, Vector2, Vector3};

/// A `WorldGenerator` is a trait that generates a `World`.
pub trait WorldGenerator {
    fn generate(&mut self) -> World;
}

pub struct FixedWorldGenerator {
    pub noise_field: NoiseField,
    pub chunk_count: Vector2<u32>,
}

impl FixedWorldGenerator {
    pub fn generate_chunk(&mut self, chunk_pos: Vector2<u32>) -> Chunk {
        let mut chunk_gen = NoiseChunkGenerator::with_noise_field(self.noise_field.clone(), 100);
        return chunk_gen.generate(chunk_pos);
    }
}

impl WorldGenerator for FixedWorldGenerator {
    fn generate(&mut self) -> World {
        let mut chunks = vec![];
        for x in 0..self.chunk_count.x {
            for z in 0..self.chunk_count.y {
                chunks.push(self.generate_chunk(vec2(x, z)));
            }
        }
        return World { chunks };
    }
}

/// A `ChunkGenerator` is a trait that generates a `Chunk`.
pub trait ChunkGenerator {
    /// Generate a block at a specific position.
    fn generate_at(&mut self, block_pos: Vector3<u32>) -> Block;

    /// Generate a `Chunk`.
    fn generate(&mut self, chunk_pos: Vector2<u32>) -> Chunk {
        let mut blocks = vec![];
        for y in 0..Chunk::SIZE.y {
            for z in 0..Chunk::SIZE.z {
                for x in 0..Chunk::SIZE.x {
                    let offset = vec3(chunk_pos.x * Chunk::SIZE.x, 0, chunk_pos.y * Chunk::SIZE.z);
                    blocks.push(self.generate_at(vec3(x, y, z) + offset));
                }
            }
        }

        return Chunk {
            blocks,
            position: chunk_pos,
        };
    }
}

pub struct NoiseChunkGenerator {
    pub noise_field: NoiseField,
    base_y: u32,
}

impl ChunkGenerator for NoiseChunkGenerator {
    fn generate_at(&mut self, position: Vector3<u32>) -> Block {
        let sample_y = self.noise_field.sample_2d(
            vec2(f64::from(position.x), f64::from(position.z)),
            None,
            Some(vec2(f64::from(Chunk::SIZE.x), f64::from(Chunk::SIZE.z))),
        );

        #[allow(clippy::cast_possible_truncation, reason = "truncation is expected")]
        #[allow(clippy::cast_sign_loss, reason = "value should never be negative")]
        let surface_y = (f64::from(self.base_y) + sample_y) as u32;
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

    #[must_use]
    pub fn with_noise_field(noise_field: NoiseField, base_y: u32) -> Self {
        return Self {
            noise_field,
            base_y,
        };
    }
}

impl Default for NoiseChunkGenerator {
    fn default() -> Self {
        // let noise_field = NoiseField::new(rand::random(), 5, 1.0, 10.0, 2.0, 0.5);
        let noise_field = NoiseField::new(0, 5, 1.0, 10.0, 2.0, 0.5);
        return Self {
            noise_field,
            base_y: 100,
        };
    }
}
