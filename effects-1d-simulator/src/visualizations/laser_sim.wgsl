#import bevy_sprite::{
    mesh2d_types,
    // The time since startup data is in the globals binding which is part of the mesh_view_bindings import
    mesh2d_view_bindings::globals,
    mesh2d_vertex_output::VertexOutput,
}

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

@group(1) @binding(0) var<storage> effect_data: array<vec4<f32>>;
@group(1) @binding(1) var<uniform> widget_size: vec2<f32>;

fn get_color(index: u32) -> vec3<f32>{
    return clamp(effect_data[index].rgb, vec3(0.,0.,0.), vec3(1.,1.,1.));
}

const BRIGHTNESS: f32 = 0.02;
const MSAA_SAMPLES: u32 = 8u;

fn get_laser_color(uv: vec2<f32>) -> vec3<f32> {
    let effect_data_len = arrayLength(&effect_data);
    let effect_data_max_index = effect_data_len - 1u;

    let pixel_pos = widget_size * uv;
    let pixel_size = 1.0 / widget_size;

    let rel_pos = vec2(widget_size.x/2., 0.) - pixel_pos;
    let rel_dist = length(rel_pos);

    var rel_angle_cos = 0.;
    if rel_dist != 0. {
        rel_angle_cos = rel_pos.x / rel_dist;
    }

    let rel_angle = acos(rel_angle_cos);

    let array_pos = clamp(u32((rel_angle / acos(-1.)) * f32(effect_data_len)), u32(0), effect_data_max_index);

    let laser_color = get_color(array_pos);

    var brightness_multiplier = 10000000.0;
    if rel_dist > 0. {
        brightness_multiplier = BRIGHTNESS * widget_size.y / rel_dist;
    }
    let brightness_adjusted_laser_color = laser_color * brightness_multiplier;

    return brightness_adjusted_laser_color;
}

@fragment
fn fragment(
    in: VertexOutput
) -> @location(0) vec4<f32> {
    var rng_state = init_rand_state(in.uv);

    let pixel_size = 1.0 / widget_size;

    var color_sum = vec3(0., 0., 0.);
    for(var i = 0u; i < MSAA_SAMPLES; i++){
        let offset = vec2(
            rand(&rng_state) - 0.5,
            rand(&rng_state) - 0.5
        ) * pixel_size;

        color_sum += get_laser_color(in.uv + offset);
    };
    let color = color_sum / f32(MSAA_SAMPLES);

    // Tone mapping
    let total_overshoot =
        max(
            max(color.r - 1., 0.),
            max(color.g - 1., color.b - 1.)
        );

    let tonemapped_color = color + total_overshoot * 0.1;

    return vec4(tonemapped_color, 1.);
}
