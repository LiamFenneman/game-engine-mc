use crate::{noise::NoiseField, Block, Chunk, ChunkTransformation, World};
use ge_util::{
    coords::{CHUNK_HEIGHT, CHUNK_SIZE},
    ChunkOffset, ChunkPos, WorldPos,
};

/// A `WorldGenerator` is a trait that generates a `World`.
pub trait WorldGenerator {
    fn generate(&mut self) -> World;
}

pub struct FixedWorldGenerator {
    gen: NoiseChunkGenerator,
    chunk_count: (i32, i32),
    transformations: Vec<Box<dyn ChunkTransformation>>,
}

impl FixedWorldGenerator {
    #[must_use]
    pub fn new(noise_field: NoiseField, chunk_count: (i32, i32)) -> Self {
        return Self::with_transformations(noise_field, chunk_count, Vec::new());
    }

    #[must_use]
    pub fn with_transformations(
        noise_field: NoiseField,
        chunk_count: (i32, i32),
        transformations: Vec<Box<dyn ChunkTransformation>>,
    ) -> Self {
        let gen = NoiseChunkGenerator::with_noise_field(noise_field, 100);
        return Self {
            gen,
            chunk_count,
            transformations,
        };
    }

    pub fn generate_chunk(&mut self, chunk_offset: impl Into<ChunkOffset> + Copy) -> Chunk {
        let mut chunk = self.gen.generate(chunk_offset);
        for trns in &self.transformations {
            trns.transform(&mut chunk);
        }
        return chunk;
    }
}

impl WorldGenerator for FixedWorldGenerator {
    fn generate(&mut self) -> World {
        let mut chunks = vec![];
        for x in 0..self.chunk_count.0 {
            for y in 0..self.chunk_count.1 {
                let off = ChunkOffset::new(x, y, 0).unwrap();
                chunks.push(self.generate_chunk(off));
            }
        }
        return World { chunks };
    }
}

/// A `ChunkGenerator` is a trait that generates a `Chunk`.
pub trait ChunkGenerator {
    /// Generate a block at a specific position.
    fn generate_at(&mut self, pos: impl Into<WorldPos>) -> Block;

    /// Generate a `Chunk`.
    fn generate(&mut self, chunk_offset: impl Into<ChunkOffset> + Copy) -> Chunk {
        let start = std::time::Instant::now();
        let mut blocks = std::collections::HashMap::new();
        let offset: ChunkOffset = chunk_offset.into();
        // TODO: parallelize this
        for z in 0i32..CHUNK_HEIGHT {
            for y in 0i32..CHUNK_SIZE {
                for x in 0i32..CHUNK_SIZE {
                    let chunk_pos = ChunkPos::new(x, y, z).unwrap();
                    let world_pos = chunk_pos.to_world_pos(offset);
                    let blk = self.generate_at(world_pos);
                    blocks.insert(world_pos, blk);
                }
            }
        }

        tracing::trace!(
            "generated chunk at {:?} in {}ms",
            chunk_offset.into(),
            start.elapsed().as_millis()
        );
        return Chunk {
            blocks,
            position: chunk_offset.into(),
        };
    }
}

pub struct NoiseChunkGenerator {
    pub noise_field: NoiseField,
    base_z: i32,
}

impl ChunkGenerator for NoiseChunkGenerator {
    fn generate_at(&mut self, pos: impl Into<WorldPos>) -> Block {
        let position: WorldPos = pos.into();
        let sample_z = self.noise_field.sample_2d(
            cgmath::vec2(f64::from(position.x()), f64::from(position.y())),
            None,
            Some(cgmath::vec2(f64::from(CHUNK_SIZE), f64::from(CHUNK_SIZE))),
        );

        #[allow(clippy::cast_possible_truncation, reason = "truncation is expected")]
        #[allow(clippy::cast_sign_loss, reason = "value should never be negative")]
        let surface_z = (f64::from(self.base_z) + sample_z) as i32;
        let ty = match surface_z {
            z if position.z() > z => crate::BlockType::Air,
            _ => crate::BlockType::Stone,
        };

        return Block { ty, position };
    }
}

impl NoiseChunkGenerator {
    #[must_use]
    pub fn new(
        seed: u64,
        base_z: i32,
        octaves: u8,
        frequency: f64,
        amplitude: f64,
        lacunarity: f64,
        gain: f64,
    ) -> Self {
        let noise_field = NoiseField::new(seed, octaves, frequency, amplitude, lacunarity, gain);
        return Self {
            noise_field,
            base_z,
        };
    }

    #[must_use]
    pub fn with_noise_field(noise_field: NoiseField, base_z: i32) -> Self {
        return Self {
            noise_field,
            base_z,
        };
    }
}

impl Default for NoiseChunkGenerator {
    fn default() -> Self {
        // let noise_field = NoiseField::new(rand::random(), 5, 1.0, 10.0, 2.0, 0.5);
        let noise_field = NoiseField::new(0, 5, 1.0, 10.0, 2.0, 0.5);
        return Self {
            noise_field,
            base_z: 100,
        };
    }
}
