use crate::gfx::texture::Texture;
use crate::math::geo::V2;

pub trait Uniform {
    fn as_bind_group(&self) -> &wgpu::BindGroup;
    fn bind_group_layout(&self) -> &wgpu::BindGroupLayout;
}

pub fn make_pipeline (
    device: &wgpu::Device,
    bind_groups: &[&wgpu::BindGroupLayout],
    vertex_buffers: &[wgpu::VertexBufferLayout],
    vertex_shader: &wgpu::ShaderModule,
    fragment_shader: &wgpu::ShaderModule,
    output_format: wgpu::TextureFormat,
) -> wgpu::RenderPipeline {
    let render_pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("render_pipeline_layout"),
            bind_group_layouts: bind_groups,
            push_constant_ranges: &[],
        });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("render_pipeline"),
                layout: Some(&render_pipeline_layout),
                vertex: wgpu::VertexState {
                    module: vertex_shader,
                    entry_point: "vs_main",
                    buffers: vertex_buffers,
                },
                fragment: Some(wgpu::FragmentState {
                    module: fragment_shader,
                    entry_point: "fs_main",
                    targets: &[Some(wgpu::ColorTargetState {
                        format: output_format,
                        blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask: wgpu::ColorWrites::ALL,
                    })]
                }),
                depth_stencil: None,
                primitive: wgpu::PrimitiveState {
                    topology: wgpu::PrimitiveTopology::TriangleList,
                    polygon_mode: wgpu::PolygonMode::Fill,
                    ..Default::default()
                },
                multisample: wgpu::MultisampleState::default(),
                multiview: None,
            }
        )
}

pub fn make_render_pass<'a>(
    encoder: &'a mut wgpu::CommandEncoder,
    target: &'a wgpu::TextureView,
    clear_color: wgpu::Color
) -> wgpu::RenderPass<'a> {
   encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
       label: Some("render_pass"),
       color_attachments: &[Some(wgpu::RenderPassColorAttachment {
           view: target,
           ops: wgpu::Operations {
               load: wgpu::LoadOp::Clear(clear_color),
               store: true,
           },
           resolve_target: None,
       })],
       depth_stencil_attachment: None,
   })
}

pub fn pixel_to_tex_coords(coords: &V2, texture: &Texture) -> V2 {
    V2::new(coords.x / texture.size.width as f32, coords.y / texture.size.height as f32)
}

