@group(1) @binding(0) var<storage> effect_data: array<u32>;
@group(1) @binding(1) var<uniform> widget_size: vec2<f32>;

struct FragmentInput {
    #import bevy_pbr::mesh_vertex_output
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    let pixel_size = 1.0 / widget_size.x;

    // TODO iterate over multiple pixels to catch all

    let count = arrayLength(&effect_data);
    let max_x = u32(i32(count) - 1);

    let x = clamp(u32(in.uv.x * f32(count)), u32(0), max_x);

    let color = effect_data[x];
    let color_r = (color >> 0u) & 0xffu;
    let color_g = (color >> 8u) & 0xffu;
    let color_b = (color >> 16u) & 0xffu;

    return vec4<f32>(
        f32(color_r) / 255.0,
        f32(color_g) / 255.0,
        f32(color_b) / 255.0,
        1.
    );
}
