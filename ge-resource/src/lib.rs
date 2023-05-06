#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::default_trait_access)]

pub mod parse;
pub mod texture;

use std::{collections::HashMap, path::PathBuf};
use texture::{TextureArray, Texture};

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

impl ResourceManager {
    /// Load a texture array from disk. If the texture array has already been loaded, it will be
    /// returned from the cache.
    ///
    /// # Panics
    /// Panics if the textures don't exist.
    pub fn load_texture_array(
        &mut self,
        texture_name: &str,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> &TextureArray {
        if !self.map.contains_key(texture_name) {
            let ta = self.load_from_disk(texture_name, device, queue);
            self.map.insert(texture_name.to_string(), ta);
        }

        return self.map.get(texture_name).unwrap();
    }

    /// Loads a texture array from disk.
    ///
    /// # Panics
    /// Panics if the textures don't exist.
    #[must_use]
    pub fn load_from_disk(
        &self,
        texture_name: &str,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> TextureArray {
        let mut array = Vec::new();

        let dir = std::fs::read_dir(self.root_path.join("assets")).unwrap();
        for entry in dir.flatten() {
            // parse the file name
            let file_name = entry.file_name();
            let (ext, (name, _index)) =
                parse::parse_file_name(file_name.to_str().unwrap()).unwrap();

            // if the texture name matches the name of the file, load the texture
            if texture_name == name && ext == ".png" {
                array.push(Self::load_texture(entry.path(), device, queue));
            }
        }

        assert!(
            !array.is_empty(),
            "the texture array must contain at least 1 texture"
        );
        return TextureArray::new(device, array, "block_texture");
    }

    /// Loads a single texture from disk.
    ///
    /// # Panics
    /// Panics if the texture doesn't exist.
    #[must_use]
    pub fn load_texture(path: PathBuf, device: &wgpu::Device, queue: &wgpu::Queue) -> Texture {
        let bytes = std::fs::read(path).unwrap();
        return Texture::from_bytes(device, queue, &bytes, "block", false).unwrap();
    }
}
