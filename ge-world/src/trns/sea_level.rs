use ge_util::EngineConfig;

use crate::ChunkTransformation;

#[derive(Debug, Clone, Copy)]
pub struct SeaLevel {
    sea_level: i32,
    fill_water: bool,
}

impl SeaLevel {
    #[must_use]
    pub fn new(config: &EngineConfig) -> Self {
        let sea_level = config.world_gen.sea_level;
        let fill_water = config.world_gen.fill_water;
        return Self {
            sea_level,
            fill_water,
        };
    }
}

impl ChunkTransformation for SeaLevel {
    fn name(&self) -> &'static str {
        return "sea level";
    }

    fn transform(&self, chunk: &mut crate::Chunk) {
        chunk
            .blocks
            .iter_mut()
            .filter(|(_, blk)| {
                return if self.fill_water {
                    blk.chunk_pos().z() <= self.sea_level
                } else {
                    blk.chunk_pos().z() == self.sea_level
                };
            })
            .filter(|(_, blk)| return blk.ty() == crate::BlockType::Air)
            .for_each(|(_, blk)| blk.update(crate::BlockType::Water));
    }
}
