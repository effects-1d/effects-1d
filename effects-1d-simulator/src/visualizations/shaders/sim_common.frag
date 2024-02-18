
in vec2 uvs;

uniform sampler2D effect_data;
uniform uint effect_data_len;
uniform vec2 widget_size;
uniform float time;

layout (location = 0) out vec4 color;

vec3 get_color(uint index) {
    float tex_coord = (float(index) + 0.5) / float(effect_data_len);
    return clamp(
        texture(effect_data, vec2(tex_coord, 0.5)).rgb,
        vec3(0.,0.,0.), vec3(1.,1.,1.)
    );
}

float srgb_inverse_transfer(float col) {
    if (col <= 0.0031308) {
        return 12.92 * col;
    } else {
        return 1.055 * pow(col, 1.0/2.4) - 0.055;
    }
}

vec3 linear_to_srgb(vec3 col) {
    return vec3(
        srgb_inverse_transfer(col.r),
        srgb_inverse_transfer(col.g),
        srgb_inverse_transfer(col.b)
    );
}

// RNG /////
uint pcg_hash(uint data)
{
    uint state = data * 747796405u + 2891336453u;
    uint word = ((state >> ((state >> 28u) + 4u)) ^ state) * 277803737u;
    return (word >> 22u) ^ word;
}

uint pcg_hash_f(float data)
{
    return pcg_hash(floatBitsToUint(data));
}

const uint RAND_PCG_MAX = 0xFFFFFFFFu;
uint rand_pcg(inout uint rng_state)
{
    uint state = rng_state;
    rng_state = rng_state * 747796405u + 2891336453u;
    uint word = ((state >> ((state >> 28u) + 4u)) ^ state) * 277803737u;
    return (word >> 22u) ^ word;
}

float rand(inout uint rng_state)
{
    uint raw_value = rand_pcg(rng_state);
    return float(raw_value) / float(RAND_PCG_MAX);
}

uint init_rand_state() {
    uint rng_state = 0u;
    rng_state ^= pcg_hash_f(time);
    rng_state ^= pcg_hash_f(uvs.x);
    rng_state ^= pcg_hash_f(uvs.y);
    return rng_state;
}
