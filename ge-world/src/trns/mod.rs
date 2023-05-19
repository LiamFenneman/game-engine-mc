mod sea_level;
pub mod surface;

pub use sea_level::SeaLevel;
pub use surface::SimpleSurfacePainter;

use crate::ChunkTransformation;

#[derive(Debug, Clone, Copy)]
pub enum Transformation {
    SeaLevel(SeaLevel),
    SurfacePainter(SimpleSurfacePainter),
}

impl ChunkTransformation for Transformation {
    fn name(&self) -> &'static str {
        match self {
            Self::SeaLevel(t) => return t.name(),
            Self::SurfacePainter(t) => return t.name(),
        }
    }

    fn transform(&self, chunk: &mut crate::Chunk) {
        match self {
            Self::SeaLevel(t) => return t.transform(chunk),
            Self::SurfacePainter(t) => return t.transform(chunk),
        }
    }
}

impl_from_trns!(SeaLevel for SeaLevel);
impl_from_trns!(SimpleSurfacePainter for SurfacePainter);
