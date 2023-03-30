#import bevy_sprite::mesh2d_types
// The time since startup data is in the globals binding which is part of the mesh_view_bindings import
#import bevy_sprite::mesh2d_view_bindings

fn pcg_hash(data: u32) -> u32
{
    let state = data * 747796405u + 2891336453u;
    let word = ((state >> ((state >> 28u) + 4u)) ^ state) * 277803737u;
    return (word >> 22u) ^ word;
}

fn pcg_hash_f(data: f32) -> u32
{
    return pcg_hash(bitcast<u32>(data));
}

const RAND_PCG_MAX: u32 = 0xFFFFFFFFu;
fn rand_pcg(rng_state: ptr<function, u32>) -> u32
{
    let state = *rng_state;
    *rng_state = *rng_state * 747796405u + 2891336453u;
    let word = ((state >> ((state >> 28u) + 4u)) ^ state) * 277803737u;
    return (word >> 22u) ^ word;
}

fn rand(rng_state: ptr<function, u32>) -> f32
{
    let raw_value = rand_pcg(rng_state);
    return f32(raw_value) / f32(RAND_PCG_MAX);
}

fn init_rand_state(uv: vec2<f32>) -> u32 {
    var rng_state: u32 = 0u;
    rng_state ^= pcg_hash_f(globals.time);
    rng_state ^= pcg_hash_f(uv.x);
    rng_state ^= pcg_hash_f(uv.y);
    return rng_state;
}

@group(1) @binding(0) var<storage> effect_data: array<u32>;
@group(1) @binding(1) var<uniform> widget_size: vec2<f32>;

struct FragmentInput{
    @builtin(position) position: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
}

@fragment
fn fragment(
    in: FragmentInput
) -> @location(0) vec4<f32> {
    var rng_state = init_rand_state(in.uv);

    let r = rand(&rng_state);
    let g = rand(&rng_state);
    let b = rand(&rng_state);

    return vec4<f32>(r, g, b ,1.);

    // let count = arrayLength(&effect_data);
    // let max_x = u32(i32(count) - 1);

    // let pixel_pos = widget_size * in.uv;
    // let pixel_size = 1.0 / widget_size;

    // let rel_pos = vec2(widget_size.x/2., 0.) - pixel_pos;
    // let rel_dist = length(rel_pos);

    // var rel_angle_cos = 0.;
    // if rel_dist != 0. {
    //     rel_angle_cos = rel_pos.x / rel_dist;
    // }

    // let rel_angle = acos(rel_angle_cos);

    // let array_pos = clamp(u32((rel_angle / acos(-1.)) * f32(count)), u32(0), max_x);

    // let color = effect_data[array_pos];
    // let color_r = (color >> 0u) & 0xffu;
    // let color_g = (color >> 8u) & 0xffu;
    // let color_b = (color >> 16u) & 0xffu;

    // return vec4<f32>(f32(color_r) / 255.0, f32(color_g) / 255.0, f32(color_b) / 255.0, 1.);
}
