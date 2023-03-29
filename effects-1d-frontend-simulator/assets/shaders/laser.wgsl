@group(1) @binding(0) var<storage> effect_data: array<u32>;
@group(1) @binding(1) var<uniform> widget_size: vec2<f32>;

struct FragmentInput {
    #import bevy_pbr::mesh_vertex_output
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    let count = arrayLength(&effect_data);
    let max_x = u32(i32(count) - 1);

    let x = clamp(u32(in.uv.x * f32(count)), u32(0), max_x);

    return vec4<f32>(f32(effect_data[x]) / 255.0, 0., 0., 1.);
}
