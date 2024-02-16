
in vec2 uvs;

uniform sampler2D effect_data;
uniform uint effect_data_len;
uniform vec2 widget_size;

layout (location = 0) out vec4 color;

vec3 get_color(uint index) {
    float tex_coord = (float(index) + 0.5) / float(effect_data_len);
    return clamp(
        texture(effect_data, vec2(tex_coord, 0.5)).rgb,
        vec3(0.,0.,0.), vec3(1.,1.,1.)
    );
}
