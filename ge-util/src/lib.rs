#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::similar_names)]
#![allow(clippy::default_trait_access)]
//
#![feature(lint_reasons)]

pub mod coords;
mod macros;

pub use coords::{ChunkOffset, ChunkPos, WorldPos};

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
