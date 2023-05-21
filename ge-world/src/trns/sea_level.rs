use crate::ChunkTransformation;

#[derive(Debug, Clone, Copy)]
pub struct SeaLevel {
    sea_level: i32,
}

impl SeaLevel {
    #[must_use]
    pub fn new(sea_level: i32) -> Self {
        return Self { sea_level };
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
            .filter(|(_, blk)| return blk.chunk_pos().z() == self.sea_level)
            .filter(|(_, blk)| return blk.ty() == crate::BlockType::Air)
            .for_each(|(_, blk)| blk.update(crate::BlockType::Water));
    }
}
