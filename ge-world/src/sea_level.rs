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
    fn transform(&mut self, chunk: &mut crate::Chunk) {
        chunk
            .blocks
            .iter_mut()
            .filter(|blk| return blk.ty == crate::BlockType::Air)
            .filter(|blk| return blk.position.z() <= self.sea_level)
            .for_each(|blk| blk.ty = crate::BlockType::Water);
    }
}
