#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::similar_names)]
#![allow(clippy::default_trait_access)]
//
#![feature(lint_reasons)]

mod circle;
mod convert;
mod coords;
mod lerp;
mod macros;
mod smooth;

pub use circle::points_in_circle;
pub use convert::{one_to_three, three_to_one};
pub use coords::{ChunkOffset, ChunkPos, WorldPos};
pub use lerp::{inverse_lerp, lerp, remap};
pub use smooth::{cosine_smooth, smoothstep, smoothstep2};

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
