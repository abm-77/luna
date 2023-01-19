use anyhow::*;
use image::GenericImageView;
use winit::dpi::PhysicalSize;

use crate::math::geo::V2;
use crate::sys::resource_manager::TextureID;

#[derive(Copy, Clone)]
pub struct Sprite {
    pub texture_id: TextureID,
    pub origin: V2,
    pub scale: V2
}

impl Default for Sprite {
    fn default() -> Self {
        Self {
            texture_id: 0,
            origin: V2::new(0.0, 0.0),
            scale: V2::new(1.0, 1.0),
        }
    }
}

impl Sprite {
    pub fn new (texture_id: TextureID, origin: V2, scale: V2) -> Self {
        Self {
            texture_id,
            origin,
            scale,
        }
    }
}

pub struct Texture {
    pub id: TextureID,
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub size: PhysicalSize<u32>,
}

impl Texture {
    pub fn from_bytes(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bytes: &[u8],
        label: Option<&str>,
    ) -> Result<Self> {
       let img = image::load_from_memory(bytes)?;
        Self::from_image(device, queue, &img, label)
    }

    pub fn from_image(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        img: &image::DynamicImage,
        label: Option<&str>,
    ) -> Result<Self> {
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();

        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(
            &wgpu::TextureDescriptor {
                size: texture_size,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                label,
            }
        );

        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * dimensions.0),
                rows_per_image: std::num::NonZeroU32::new(dimensions.1),
            },
            texture_size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        Ok(Self { id: 0, texture, view, sampler, size: PhysicalSize::new(dimensions.0, dimensions.1) })
    }
}