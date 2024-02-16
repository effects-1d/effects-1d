void main()
{
    float width = widget_size.x;
    float x = uvs.x * width;
    float local_x = mod(x, 20.0);

    vec3 color_linrgb = texture(data, uvs).rgb;

    if (local_x > 15.0) {
        color_linrgb = vec3(0.0 + data_size / 10000.0, color_linrgb.g, color_linrgb.b);
    } else {
        color_linrgb = vec3(1.0, color_linrgb.g, color_linrgb.b);
    }

    //vec3 color_linrgb = vec3(1.0 - uvs.x,0.0,uvs.x);
    color = vec4(pow(color_linrgb, vec3(1.0 / 2.2)),1.0);
}
