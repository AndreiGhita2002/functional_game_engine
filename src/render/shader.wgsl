struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) tex_coords: vec2<f32>,
};

struct InstanceInput {
    @location(2) offset: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) tex_pos: vec2<f32>
}

@vertex
fn vs_main(
//    @builtin(vertex_index) i: u32,
    vertex: VertexInput,
    instance: InstanceInput,
) -> VertexOutput {
    var out: VertexOutput;
//    let int_vert_pos = vec2<u32>(
//        ((i >> 0) & 1) ^ ((i >> 1) & 1),
//        (i >> 1) & 1,
//    );
//    let vert_pos = vec2<f32>(int_vert_pos);
    out.tex_pos = vertex.tex_coords;
    out.position = vec4<f32>(vertex.position + instance.offset, 0.0, 1.0);
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