#![deny(clippy::implicit_return)]
#![allow(clippy::needless_return)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::default_trait_access)]

pub mod parse;
pub mod util;

use ge_render::texture::TextureArray;
use std::collections::HashMap;

/// A resource manager that caches textures.
#[derive(Debug, Default)]
pub struct ResourceManager {
    map: HashMap<String, TextureArray>,
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
            let ta = util::load_from_disk(texture_name, device, queue);
            self.map.insert(texture_name.to_string(), ta);
        }

        return self.map.get(texture_name).unwrap();
    }
}
