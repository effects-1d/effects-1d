
in vec2 uvs;

uniform sampler2D data;

layout (location = 0) out vec4 color;

void main()
{
    vec3 color_linrgb = texture(data, uvs).rgb;

    //vec3 color_linrgb = vec3(1.0 - uvs.x,0.0,uvs.x);
    color = vec4(pow(color_linrgb, vec3(1.0 / 2.2)),1.0);
}
