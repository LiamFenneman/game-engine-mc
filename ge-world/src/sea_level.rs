use crate::ChunkTransformation;

#[derive(Debug, Clone, Copy)]
pub struct SeaLevel {
    sea_level: u32,
}

impl SeaLevel {
    #[must_use]
    pub fn new(sea_level: u32) -> Self {
        return Self { sea_level };
    }
}

impl ChunkTransformation for SeaLevel {
    fn transform(&mut self, chunk: &mut crate::Chunk) {
        chunk
            .blocks
            .iter_mut()
            .filter(|blk| return blk.ty == crate::BlockType::Air)
            .filter(|blk| return blk.position.y <= self.sea_level)
            .for_each(|blk| blk.ty = crate::BlockType::Water);
    }
}
