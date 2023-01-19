use std::cell::RefCell;
use std::num::NonZeroU32;
use std::rc::Rc;

use wgpu::{BindGroup, BindGroupLayout, include_wgsl, Sampler, TextureView};
use wgpu::util::DeviceExt;
use winit::dpi::PhysicalSize;

use crate::gfx::camera::{CameraUniform, OrthographicCamera};
use crate::gfx::geometry::{LunarVertex, Vertex2D};
use crate::gfx::graphics_subsystem::GraphicsSubsystem;
use crate::gfx::texture::{Sprite, Texture};
use crate::gfx::util;
use crate::gfx::util::{pixel_to_tex_coords, Uniform};
use crate::math::geo::{V2, v2_rotate_about_v2};
use crate::sys::resource_manager::{ResourceManager, TextureID, WHITE_TEXTURE_ID};

const MAX_QUADS: usize = 1024;
const MAX_INDICES: usize = MAX_QUADS * 6;
const MAX_VERTICES: usize = MAX_QUADS * 4;
const MAX_TEXTURES: usize = 16;

#[derive(Debug)]
pub struct Uniforms {
    camera_buffer: wgpu::Buffer,
    camera_uniform: CameraUniform,
    bind_group: wgpu::BindGroup,
    layout: wgpu::BindGroupLayout,
}

impl Uniforms {
    fn extract_texture_views_and_samplers<'a>(textures: &'a[&'a Texture]) -> (Vec<&'a TextureView>, Vec<&'a Sampler>) {
        let mut texture_views: Vec<&TextureView> = Vec::new();
        let mut texture_samplers: Vec<&Sampler> = Vec::new();

        for tx in textures.iter() {
            texture_views.push(&tx.view);
            texture_samplers.push(&tx.sampler);
        }

        return (texture_views, texture_samplers);
    }

    pub fn update_textures(&mut self, device: &wgpu::Device, textures: &[&Texture; MAX_TEXTURES]) {
        let (texture_views, texture_samplers)
            = Uniforms::extract_texture_views_and_samplers(textures);

        self.bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("renderer2d.uniforms.bind_group"),
            layout: &self.layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: self.camera_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureViewArray(&texture_views.as_slice()),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::SamplerArray(&texture_samplers.as_slice()),
                }
            ]
        });
    }

    pub fn new(
        device: &wgpu::Device,
        textures: &[&Texture; MAX_TEXTURES],
    ) -> Self {
        let layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: Some("renderer2d.uniforms.bind_group_layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float {
                                filterable: true,
                            },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: NonZeroU32::new(MAX_TEXTURES as u32),
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 2,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: NonZeroU32::new(MAX_TEXTURES as u32),
                    },
                ]
            }
        );

        let camera_uniform = CameraUniform::new();
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("renderer2d.uniforms.camera_buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let (texture_views, texture_samplers)
            = Uniforms::extract_texture_views_and_samplers(textures);

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("renderer2d.uniforms.bind_group"),
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: camera_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureViewArray(&texture_views.as_slice()),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::SamplerArray(&texture_samplers.as_slice()),
                }
            ]
        });

        Self {
            camera_buffer,
            camera_uniform,
            bind_group,
            layout,
        }
    }
}

impl Uniform for Uniforms {
    fn as_bind_group(&self) -> &BindGroup {
        &self.bind_group
    }
    fn bind_group_layout(&self) -> &BindGroupLayout {
        &self.layout
    }
}

pub struct Renderer2D {
    gfx: Rc<RefCell<GraphicsSubsystem>>,
    res: Rc<RefCell<ResourceManager>>,

    pub camera: OrthographicCamera,

    index_buffer: wgpu::Buffer,
    vertex_buffer: wgpu::Buffer,

    uniforms: Uniforms,

    vertex_data: [Vertex2D; MAX_VERTICES],
    n_quads_drawn: usize,

    pipeline: wgpu::RenderPipeline,

    texture_slots: [TextureID; MAX_TEXTURES],
    active_textures: usize,
}

impl Renderer2D {
    pub fn init(gfx: Rc<RefCell<GraphicsSubsystem>>, res: Rc<RefCell<ResourceManager>>) -> Self{
        let g = (*gfx).borrow();

        let mut indices_data: [u16; MAX_INDICES] = [0u16; MAX_INDICES];
        for i in 0..MAX_QUADS {
            indices_data[i * 6 + 0] = (i * 4 + 0) as u16;
            indices_data[i * 6 + 1] = (i * 4 + 1) as u16;
            indices_data[i * 6 + 2] = (i * 4 + 3) as u16;
            indices_data[i * 6 + 3] = (i * 4 + 1) as u16;
            indices_data[i * 6 + 4] = (i * 4 + 2) as u16;
            indices_data[i * 6 + 5] = (i * 4 + 3) as u16;
        }

        let indices = g.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("renderer2d.index_buffer"),
            contents: bytemuck::cast_slice(&indices_data),
            usage: wgpu::BufferUsages::INDEX,
        });

        let vertices_data = [Vertex2D::default(); MAX_VERTICES];

        let vertices = g.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("renderer2d.vertex_buffer"),
            contents: bytemuck::cast_slice(&vertices_data),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });

        let ortho = OrthographicCamera {
            pos: (0.0, 0.0, 1.0).into(),
            dir: (0.0, 0.0, -1.0).into(),
            up: cgmath::Vector3::unit_y(),
            size: (800, 600).into(),
            znear: 0.1,
            zfar: 100.0,
        };

        let shader_module = g.device
            .create_shader_module(include_wgsl!("shader2d.wgsl"));

        let r = (*res).borrow();
        let white_texture = r.get_texture(WHITE_TEXTURE_ID);
        let uniforms = Uniforms::new(&g.device, &[white_texture; MAX_TEXTURES]);
        std::mem::drop(r);

        let pipeline = util::make_pipeline(
            &g.device,
            &[&uniforms.bind_group_layout()],
            &[Vertex2D::desc()],
            &shader_module,
            &shader_module,
            g.surface_format,
        );
        std::mem::drop(g);

        Self {
            gfx,
            res,
            camera: ortho,
            index_buffer: indices,
            vertex_buffer: vertices,
            pipeline,
            uniforms,
            vertex_data: vertices_data,
            n_quads_drawn: 0,
            texture_slots: [WHITE_TEXTURE_ID; MAX_TEXTURES],
            active_textures: 1,
        }
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        let mut gfx = (*self.gfx).borrow_mut();
        if size.width > 0 && size.height > 0 {
            gfx.config.width = size.width;
            gfx.config.height = size.height;
            gfx.surface.configure(&gfx.device, &gfx.config);
        }
    }

    pub fn render (&mut self) ->  Result<(), wgpu::SurfaceError> {
        self.flush()
    }

    pub fn draw_sprite(&mut self,  sprite: &Sprite, pos: &V2) {
        let res = self.res.clone();
        let res = (*res).borrow_mut();

        let texture = res.get_texture(sprite.texture_id);

        self.draw_quad_texture(
            pos,
            &V2::new(
                texture.size.width as f32 * sprite.scale.x,
                texture.size.height as f32 * sprite.scale.y,
            ),
            texture,
        );
    }

    pub fn draw_sprite_ext(
        &mut self,
        sprite: &Sprite,
        pos: &V2,
        size: &V2,
        src_pos: &V2,
        src_size: &V2,
        rotation: Option<f32>,
    )  {
        let res = self.res.clone();
        let res = (*res).borrow_mut();
        let texture = res.get_texture(sprite.texture_id);

        self.draw_quad_texture_ext(
            pos,
            &V2::new(
                size.x * sprite.scale.x,
                size.y as f32 * sprite.scale.y,
            ),
            texture,
            src_pos,
            src_size,
            &sprite.origin,
            rotation,
        );
    }

    pub fn draw_quad_texture (
        &mut self,
        pos: &V2,
        size: &V2,
        texture: &Texture
    ) {
        self.draw_quad_texture_ext(
            pos,
            size,
            texture,
            &V2::new(0.0, 0.0),
            &V2::new(texture.size.width as f32, texture.size.height as f32),
            &V2::new(0.0, 0.0),
            None,
        );
    }

    pub fn draw_quad_texture_ext (
        &mut self,
        pos: &V2,
        size: &V2,
        texture: &Texture,
        src_pos: &V2,
        src_size: &V2,
        origin: &V2,
        rotation: Option<f32>,
    ) {
        if self.n_quads_drawn == MAX_QUADS { self.flush().unwrap(); }

        let mut tex_idx: i32 = -1;
        for i in 1..self.active_textures {
            if self.texture_slots[i] == texture.id {
                tex_idx = texture.id as i32;
                break;
            }
        }

        if tex_idx < 0 {
            tex_idx = self.active_textures as i32;
            self.texture_slots[self.active_textures] = texture.id;
            self.active_textures += 1;
        }

        let tex_coords = pixel_to_tex_coords(src_pos, &texture);
        let tex_size = pixel_to_tex_coords(src_size, &texture);

        let l = pos.x - (origin.x * size.x);
        let r = pos.x + ((1.0 - origin.x) * size.x);
        let t = pos.y - (origin.y * size.y);
        let b = pos.y + ((1.0 - origin.y) * size.y);

        let mut ltp = V2::new(l, t);
        let mut rtp = V2::new(r, t);
        let mut lbt = V2::new(l, b);
        let mut rbt = V2::new(r, b);

        if let Some(angle) = rotation {
            let s = angle.sin();
            let c = angle.cos();

            ltp = v2_rotate_about_v2(&ltp, pos, s, c);
            rtp = v2_rotate_about_v2(&rtp, pos, s, c);
            lbt = v2_rotate_about_v2(&lbt, pos, s, c);
            rbt = v2_rotate_about_v2(&rbt, pos, s, c);
        }

        let tl = tex_coords.x;
        let tr = tex_coords.x + tex_size.x;
        let tt = tex_coords.y;
        let tb = tex_coords.y + tex_size.y;

        let offset = self.n_quads_drawn * 4;
        self.vertex_data[0 + offset] = Vertex2D { pos: ltp.into(), tex_coords: [tl, tt], tex_idx };
        self.vertex_data[1 + offset] = Vertex2D { pos: rtp.into(), tex_coords: [tr, tt], tex_idx };
        self.vertex_data[2 + offset] = Vertex2D { pos: rbt.into(), tex_coords: [tr, tb], tex_idx };
        self.vertex_data[3 + offset] = Vertex2D { pos: lbt.into(), tex_coords: [tl, tb], tex_idx };
        self.n_quads_drawn += 1;
    }

    pub fn flush (&mut self) -> Result<(), wgpu::SurfaceError> {
        let gfx = (*self.gfx).borrow();

        self.uniforms.camera_uniform.update_view_proj(&mut self.camera);
        gfx.queue.write_buffer(
            &self.uniforms.camera_buffer,
            0,
            bytemuck::cast_slice(&[self.uniforms.camera_uniform]),
        );

        gfx.queue.write_buffer(
            &self.vertex_buffer,
            0 as wgpu::BufferAddress,
            bytemuck::cast_slice(
                &self.vertex_data[0..(self.n_quads_drawn * 4)],
            )
        );

        let surface_texture = gfx.surface.get_current_texture().unwrap();
        let surface_view = surface_texture.texture.create_view(
            &wgpu::TextureViewDescriptor::default()
        );
        let mut encoder = gfx.device.create_command_encoder(
            &wgpu::CommandEncoderDescriptor {
                label: Some("renderer2d.command_encoder")
            },
        );

        {
            let mut rp = util::make_render_pass(
                &mut encoder,
                &surface_view,
                wgpu::Color {
                    r: 0.02,
                    g: 0.02,
                    b: 0.04,
                    a: 1.0,
                },
            );

            rp.set_pipeline(&self.pipeline);
            rp.set_vertex_buffer(
                0,
                self.vertex_buffer.slice(
                    ..self.n_quads_drawn as u64 * 4 * std::mem::size_of::<Vertex2D>() as u64,
                ),
            );

            rp.set_index_buffer(
                self.index_buffer.slice(
                    ..self.n_quads_drawn as u64 * 6 * std::mem::size_of::<u16> as u64,
                ),
                wgpu::IndexFormat::Uint16,
            );

            let res =(*self.res).borrow();
            let white_texture = res.get_texture(WHITE_TEXTURE_ID);
            let mut textures = [white_texture; MAX_TEXTURES];
            for i in 0..self.active_textures { textures[i] = res.get_texture(self.texture_slots[i]) }
            self.uniforms.update_textures(&gfx.device, &textures);

            rp.set_bind_group(0,self.uniforms.as_bind_group(), &[]);

            rp.draw_indexed(0..(self.n_quads_drawn as u32 * 6), 0, 0..1);
        }

        gfx.queue.submit(std::iter::once(encoder.finish()));

        surface_texture.present();

        self.n_quads_drawn = 0;

        Ok(())
    }
}