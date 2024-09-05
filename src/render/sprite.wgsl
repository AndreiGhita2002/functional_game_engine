struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct InstanceInput {
    @location(2) offset: vec2<f32>,
    @location(3) matrix_0: vec2<f32>,
    @location(4) matrix_1: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_pos: vec2<f32>
}

@vertex
fn vs_main(
    vertex: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
    let sprite_matrix = mat2x2<f32>(
        instance.matrix_0,
        instance.matrix_1,
    );

    let p = sprite_matrix * (vertex.position + instance.offset);

    out.position = vec4<f32>(p, 0.0, 1.0);

    out.tex_pos = vertex.tex_coords;

    return out;
}

// Fragment shader
@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(t_diffuse, s_diffuse, in.tex_pos);
}