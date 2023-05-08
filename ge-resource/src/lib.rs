#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::similar_names)]
#![allow(clippy::default_trait_access)]
//
#![feature(lint_reasons)]

pub mod block;
pub mod config;
pub mod data;
pub mod parse;
pub mod texture;

use ge_world::BlockType;
use std::{collections::HashMap, path::PathBuf};
use texture::TextureArray;

/// A resource manager that caches textures.
#[derive(Debug)]
pub struct ResourceManager {
    asset_path: PathBuf,
    config_path: PathBuf,
    data_path: PathBuf,

    map: HashMap<BlockType, TextureArray>,
}

impl Default for ResourceManager {
    fn default() -> Self {
        return Self {
            asset_path: PathBuf::from("assets"),
            config_path: PathBuf::from("config"),
            data_path: PathBuf::from("data"),

            map: HashMap::new(),
        };
    }
}
