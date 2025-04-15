// Vertex input for 3D model
struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct ModelInput {
    @location(2) model_0: vec4<f32>,
    @location(3) model_1: vec4<f32>,
    @location(4) model_2: vec4<f32>,
    @location(5) model_3: vec4<f32>,
}

// Uniforms for transformation matrices
struct Uniforms {
    view: mat4x4<f32>,
    projection: mat4x4<f32>,
};
@group(0) @binding(0)
var<uniform> uniforms: Uniforms;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_pos: vec2<f32>
};

@vertex
fn vs_main(vertex: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    let model = mat4x4(ModelInput.model_0, ModelInput.model_1, ModelInput.model_2, ModelInput.model_3);

    let model_pos = model * vec4<f32>(vertex.position, 1.0);
    let view_pos = uniforms.view * model_pos;
    out.position = uniforms.projection * view_pos;

    out.tex_pos = vertex.tex_coords;

    return out;
}

// Fragment shader (same as before)
@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_pos);
}
