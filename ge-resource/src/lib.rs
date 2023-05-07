#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::default_trait_access)]

pub mod parse;
pub mod texture;

use std::{collections::HashMap, path::PathBuf};
use texture::TextureArray;

/// A resource manager that caches textures.
#[derive(Debug)]
pub struct ResourceManager {
    root_path: PathBuf,
    map: HashMap<String, TextureArray>,
}

impl Default for ResourceManager {
    fn default() -> Self {
        return Self {
            map: HashMap::new(),
            root_path: std::env::current_dir().expect("failed to get current directory"),
        };
    }
}
