#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::default_trait_access)]

pub mod block;
pub mod data;
pub mod parse;
pub mod texture;

use std::{collections::HashMap, path::PathBuf};
use texture::TextureArray;
use ge_world::BlockType;

/// A resource manager that caches textures.
#[derive(Debug)]
pub struct ResourceManager {
    asset_path: PathBuf,
    data_path: PathBuf,
    map: HashMap<BlockType, TextureArray>,
}

impl Default for ResourceManager {
    fn default() -> Self {
        return Self {
            map: HashMap::new(),
            asset_path: PathBuf::from("assets"),
            data_path: PathBuf::from("data"),
        };
    }
}
