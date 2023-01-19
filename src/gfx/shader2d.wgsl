// VERTEX SHADER
struct CameraUniform {
    view_proj: mat4x4<f32>,
}

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
    @location(2) tex_idx: i32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
    @location(1) tex_idx: i32,
};

@group(0) @binding(0)
var<uniform> camera: CameraUniform;

@vertex
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = camera.view_proj * vec4<f32>(in.position, 0.0, 1.0);
    out.tex_coords = in.tex_coords;
    out.tex_idx = in.tex_idx;
    return out;
}

// FRAGMENT SHADER
@group(0) @binding(1)
var textures: binding_array<texture_2d<f32>>;

@group(0) @binding(2)
var texture_samplers: binding_array<sampler>;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(textures[in.tex_idx], texture_samplers[in.tex_idx], in.tex_coords);
}