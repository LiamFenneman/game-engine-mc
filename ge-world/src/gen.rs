use crate::{noise::Noise, Block, Chunk, ChunkTransformation, World};
use ge_util::{
    coords::{CHUNK_HEIGHT, CHUNK_SIZE},
    ChunkOffset, ChunkPos, EngineConfig,
};

/// A `WorldGenerator` is a trait that generates a `World`.
pub trait WorldGenerator {
    fn generate(&mut self) -> World;
}

pub struct FixedWorldGenerator {
    gen: NoiseChunkGenerator,
    pub count: (i32, i32),
    transformations: Vec<Box<dyn ChunkTransformation>>,
}

impl FixedWorldGenerator {
    #[must_use]
    pub fn new(
        noise: Noise,
        count: (i32, i32),
        transformations: Vec<Box<dyn ChunkTransformation>>,
        config: &EngineConfig,
    ) -> Self {
        let gen = NoiseChunkGenerator::with_noise(noise, config.world_gen.base_height);
        return Self {
            gen,
            count,
            transformations,
        };
    }

    fn generate_chunk(&mut self, chunk_offset: impl Into<ChunkOffset> + Copy) -> Chunk {
        return self.gen.generate(chunk_offset);
    }
}

impl WorldGenerator for FixedWorldGenerator {
    fn generate(&mut self) -> World {
        let mut chunks = vec![];
        for x in (1 - self.count.0)..self.count.0 {
            for y in (1 - self.count.1)..self.count.1 {
                let off = ChunkOffset::new(x, y, 0).unwrap();
                let mut chunk = self.generate_chunk(off);
                for trns in &self.transformations {
                    trns.transform(&mut chunk);
                }
                chunks.push(chunk);
            }
        }
        return World { chunks };
    }
}

/// A `ChunkGenerator` is a trait that generates a `Chunk`.
pub trait ChunkGenerator {
    /// Generate a block at a specific position.
    fn generate_at(
        &mut self,
        chunk_pos: impl Into<ChunkPos>,
        chunk_offset: impl Into<ChunkOffset> + Copy,
    ) -> Block;

    /// Generate a `Chunk`.
    fn generate(&mut self, chunk_offset: impl Into<ChunkOffset> + Copy) -> Chunk {
        let start = std::time::Instant::now();
        let mut blocks = std::collections::HashMap::new();
        // TODO: parallelize this
        for z in 0i32..CHUNK_HEIGHT {
            for y in 0i32..CHUNK_SIZE {
                for x in 0i32..CHUNK_SIZE {
                    let chunk_pos = ChunkPos::new(x, y, z).unwrap();
                    let blk = self.generate_at(chunk_pos, chunk_offset);
                    blocks.insert(chunk_pos, blk);
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
    pub noise: Noise,
    base_z: i32,
}

#[allow(clippy::cast_precision_loss, reason = "precisions is not important")]
impl ChunkGenerator for NoiseChunkGenerator {
    fn generate_at(
        &mut self,
        chunk_pos: impl Into<ChunkPos>,
        chunk_offset: impl Into<ChunkOffset> + Copy,
    ) -> Block {
        let chunk_pos: ChunkPos = chunk_pos.into();
        let world_pos = chunk_pos.to_world_pos(chunk_offset);
        let sample_z = self
            .noise
            .fbm(world_pos.x() as f32, world_pos.y() as f32, 0.0);

        #[allow(clippy::cast_possible_truncation, reason = "truncation is expected")]
        #[allow(clippy::cast_sign_loss, reason = "value should never be negative")]
        let surface_z = (self.base_z as f32 + sample_z) as i32;
        let ty = match surface_z {
            z if chunk_pos.z() > z => crate::BlockType::Air,
            _ => crate::BlockType::Stone,
        };

        return Block::new(ty, chunk_pos, chunk_offset);
    }
}

impl NoiseChunkGenerator {
    #[must_use]
    pub fn new(
        #[allow(unused)] seed: u64,
        base_z: i32,
        octaves: usize,
        frequency: f32,
        amplitude: f32,
        lacunarity: f32,
        persistence: f32,
    ) -> Self {
        let noise = Noise::new(octaves, frequency, amplitude, lacunarity, persistence);
        return Self { noise, base_z };
    }

    #[must_use]
    pub fn with_noise(noise: Noise, base_z: i32) -> Self {
        return Self { noise, base_z };
    }
}
