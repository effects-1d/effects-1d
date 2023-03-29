#import bevy_sprite::mesh2d_types
// The time since startup data is in the globals binding which is part of the mesh_view_bindings import
#import bevy_sprite::mesh2d_view_bindings

#import bevy_pbr::utils

@group(1) @binding(0) var<storage> effect_data: array<u32>;

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



// @fragment
// fn fragment(
//     #import bevy_pbr::mesh_vertex_output
// ) -> @location(0) vec4<f32> {
//     return vec4(0.2, 0.4, sin(globals.time * 5.0) * 0.5 + 0.5, 0.8);
// }
