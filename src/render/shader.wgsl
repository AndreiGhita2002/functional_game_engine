struct InstanceInput {
    @location(0) offset: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    tex_pos: vec2<f32>
}

@vertex
fn vs_main(
    @builtin(vertex_index) i: u32,
    instance: InstanceInput
) -> VertexOutput {
    var out: VertexOutput;
    let instance_pos = mat2x2<f32>(
        instance.sprite_matrix_0,
        instance.sprite_matrix_1,
    );
    let vert_pos = vec2<f32>(
        ((i >> 0) & 1) ^ ((i >> 1) & 1),
        (i >> 1) & 1,
    );
    out.tex_pos = vert_pos;
    out.position = vert_pos + instance.offset;
    return out;
}

// Fragment shader
@group(0) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
//    return vec4<f32>(vertex.position, 0.0, 1.0);
    return textureSample(t_diffuse, s_diffuse, in.tex_pos);
}