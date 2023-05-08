#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::similar_names)]
//
#![feature(btree_cursors)]
#![feature(lint_reasons)]

pub mod gen;
pub mod noise;
pub mod spline;
pub mod util;

mod types;
pub use types::*;
