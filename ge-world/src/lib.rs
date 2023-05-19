#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::similar_names)]
#![allow(clippy::default_trait_access)]
//
#![feature(btree_cursors)]
#![feature(lint_reasons)]

pub mod gen;
pub mod noise;
pub mod spline;
pub mod trns;

mod types;
pub use types::*;

#[macro_use]
extern crate ge_macros;
