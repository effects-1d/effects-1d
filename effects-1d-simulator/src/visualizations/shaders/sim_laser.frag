
const float BRIGHTNESS = 0.02;
const uint MSAA_SAMPLES = 8u;


vec3 get_laser_color(vec2 uv) {
    uint effect_data_max_index = effect_data_len - 1u;

    vec2 pixel_pos = widget_size * uv;
    vec2 pixel_size = 1.0 / widget_size;

    vec2 rel_pos = vec2(widget_size.x/2., 0.) - pixel_pos;
    float rel_dist = length(rel_pos);

    float rel_angle_cos = 0.;
    if (rel_dist != 0.) {
        rel_angle_cos = rel_pos.x / rel_dist;
    }

    float rel_angle = acos(rel_angle_cos);

    uint array_pos = clamp(uint((rel_angle / acos(-1.)) * float(effect_data_len)), uint(0), effect_data_max_index);

    vec3 laser_color = get_color(array_pos);

    float brightness_multiplier = 10000000.0;
    if (rel_dist > 0.) {
        brightness_multiplier = BRIGHTNESS * widget_size.y / rel_dist;
    }
    vec3 brightness_adjusted_laser_color = laser_color * brightness_multiplier;

    return brightness_adjusted_laser_color;
}

void main()
{
    uint rng_state = init_rand_state();

    vec2 pixel_size = 1.0 / widget_size;

    vec3 color_sum = vec3(0., 0., 0.);
    for(uint i = 0u; i < MSAA_SAMPLES; i++){
        vec2 offset = vec2(
            rand(rng_state) - 0.5,
            rand(rng_state) - 0.5
        ) * pixel_size;

        color_sum += get_laser_color(vec2(uvs.x, 1.0 - uvs.y) + offset);
    };
    vec3 color_result = color_sum / float(MSAA_SAMPLES);

    // Tone mapping
    float total_overshoot =
        max(
            max(color_result.r - 1., 0.),
            max(color_result.g - 1., color_result.b - 1.)
        );

    vec3 tonemapped_color = color_result + total_overshoot * 0.1;

    // Add alpha and do gamma correction
    color = vec4(linear_to_srgb(tonemapped_color), 1.0);
}
