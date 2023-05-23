#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::similar_names)]
#![allow(clippy::default_trait_access)]
#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]
//
#![feature(lint_reasons)]

mod circle;
mod config;
mod convert;
pub mod coords;
mod lerp;
mod macros;

pub use circle::points_in_circle;
pub use config::{EngineConfig, WorldGenConfig};
pub use convert::{deg_to_rad, rad_to_deg};
pub use coords::{ChunkOffset, ChunkPos, WorldPos};
pub use lerp::lerp;

#[macro_export]
macro_rules! wpos {
    ($x:literal, $y:literal, $z:literal) => {
        $crate::coords::WorldPos::new($x, $y, $z)
    };
}
#[macro_export]
macro_rules! cpos {
    ($x:literal, $y:literal, $z:literal) => {
        $crate::coords::ChunkPos::new($x, $y, $z)
    };
}
#[macro_export]
macro_rules! coff {
    ($x:literal, $y:literal, $z:literal) => {
        $crate::coords::ChunkOffset::new($x, $y, $z)
    };
}
