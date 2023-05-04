#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::default_trait_access)]

#![feature(btree_cursors)]

pub mod gen;
pub mod noise;
pub mod spline;
pub mod util;

mod types;
pub use types::*;
