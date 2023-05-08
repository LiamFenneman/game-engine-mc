#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]
#![warn(clippy::pedantic)]
//
#![feature(btree_cursors)]

pub mod gen;
pub mod noise;
pub mod spline;
pub mod util;

mod types;
pub use types::*;
