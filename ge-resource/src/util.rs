use crate::{parse, texture::{TextureArray, Texture}};
use std::{
    env,
    fs::read_dir,
    path::{Path, PathBuf},
};

/// Loads a texture array from disk.
///
/// # Panics
/// Panics if the textures don't exist.
#[must_use]
pub fn load_from_disk(
    texture_name: &str,
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> TextureArray {
    let mut array = Vec::new();

    let dir = read_dir(Path::new(&env::var("ASSET_DIR").unwrap())).unwrap();
    for entry in dir.flatten() {
        // parse the file name
        let file_name = entry.file_name();
        let (ext, (name, _index)) = parse::parse_file_name(file_name.to_str().unwrap()).unwrap();

        // if the texture name matches the name of the file, load the texture
        if texture_name == name && ext == ".png" {
            array.push(load_texture(entry.path(), device, queue));
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
