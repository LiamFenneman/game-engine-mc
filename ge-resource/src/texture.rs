use crate::block::BlockMeta;
use ge_world::BlockType;
use image::GenericImageView;
use std::{num::NonZeroU32, path::PathBuf, rc::Rc};

impl crate::ResourceManager {
    /// Load a texture array from disk. If the texture array has already been loaded, it will be
    /// returned from the cache.
    ///
    /// # Panics
    /// Panics if the textures don't exist.
    pub fn load_texture_array(
        &mut self,
        block_type: BlockType,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> &TextureArray {
        if !self.map.contains_key(&block_type) {
            let ta = self.load_from_disk(block_type, device, queue);
            self.map.insert(block_type, ta);
        }

        return self.map.get(&block_type).unwrap();
    }

    /// Loads a texture array from disk.
    ///
    /// # Panics
    /// Panics if the textures don't exist.
    #[must_use]
    pub fn load_from_disk(
        &self,
        block_type: BlockType,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> TextureArray {
        let block_meta = BlockMeta::load_from_disk(self, block_type).unwrap();

        let textures = block_meta
            .get_faces()
            .iter()
            .map(|s| {
                let s = format!("{s}.png");
                return Self::load_texture(self.asset_path.join(s), device, queue);
            })
            .collect::<Vec<_>>();

        assert!(
            !textures.is_empty(),
            "the texture array must contain at least 1 texture"
        );
        return TextureArray::new(device, textures, block_meta);
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

#[derive(Debug)]
pub struct TextureArray {
    pub textures: Vec<Texture>,
    pub bind_group_layout: wgpu::BindGroupLayout,
    pub bind_group: Rc<wgpu::BindGroup>,
    pub block_meta: BlockMeta,
}

impl TextureArray {
    /// Create a new [`TextureArray`] from a list of [`Texture`]s.
    ///
    /// # Panics
    /// Panics if the length of `textures` is 0.
    #[must_use]
    pub fn new(device: &wgpu::Device, textures: Vec<Texture>, block_meta: BlockMeta) -> Self {
        let texture_views = textures
            .iter()
            .map(|texture| return &texture.view)
            .collect::<Vec<_>>();

        let texture_samplers = textures
            .iter()
            .map(|texture| return &texture.sampler)
            .collect::<Vec<_>>();

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        multisampled: false,
                        view_dimension: wgpu::TextureViewDimension::D2,
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    },
                    count: Some(NonZeroU32::new(textures.len() as u32).unwrap()),
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: Some(NonZeroU32::new(textures.len() as u32).unwrap()),
                },
            ],
            label: Some("block_bind_group_layout"),
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureViewArray(&texture_views),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::SamplerArray(&texture_samplers),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        return Self {
            textures,
            bind_group_layout,
            bind_group: Rc::new(bind_group),
            block_meta,
        };
    }
}

#[derive(Debug)]
pub struct Texture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
}

impl Texture {
    pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;

    #[must_use]
    pub fn create_depth_texture(
        device: &wgpu::Device,
        config: &wgpu::SurfaceConfiguration,
        label: &str,
    ) -> Self {
        let size = wgpu::Extent3d {
            width: config.width,
            height: config.height,
            depth_or_array_layers: 1,
        };
        let desc = wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: Self::DEPTH_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        };
        let texture = device.create_texture(&desc);

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            compare: Some(wgpu::CompareFunction::LessEqual),
            lod_min_clamp: 0.0,
            lod_max_clamp: 100.0,
            ..Default::default()
        });

        return Self {
            texture,
            view,
            sampler,
        };
    }

    /// Create a new [`Texture`] from a list of bytes.
    ///
    /// # Errors
    /// Errors if the bytes cannot be decoded into an image.
    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label: &str,
        is_normal_map: bool,
    ) -> Result<Self, image::ImageError> {
        let img = image::load_from_memory(bytes)?;
        return Ok(Self::from_image(
            device,
            queue,
            &img,
            Some(label),
            is_normal_map,
        ));
    }

    #[must_use]
    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
        label: Option<&str>,
        is_normal_map: bool,
    ) -> Self {
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };
        let format = if is_normal_map {
            wgpu::TextureFormat::Rgba8Unorm
        } else {
            wgpu::TextureFormat::Rgba8UnormSrgb
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label,
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        queue.write_texture(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        return Self {
            texture,
            view,
            sampler,
        };
    }
}
