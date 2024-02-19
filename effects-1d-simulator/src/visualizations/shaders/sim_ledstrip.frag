
const float GRIDLINE_SIZE = 0.15;

void main()
{
    float pixel_size = 1.0 / widget_size.x;

    uint effect_data_max_index = effect_data_len - 1u;

    float pos_start = (uvs.x - 0.5*pixel_size) * float(effect_data_len);
    float pos_end = (uvs.x + 0.5*pixel_size) * float(effect_data_len);

    float pos_start_floor = floor(pos_start);
    float pos_end_floor = floor(pos_end);

    uint index_start = clamp(uint(pos_start_floor), 0u, effect_data_max_index);
    uint index_end = clamp(uint(pos_end_floor), 0u, effect_data_max_index);

    vec3 output_color = vec3(0., 0., 0.);
    if(index_end <= index_start) {
        // If we start and end in the same cell, the average is equal to the cell value
        output_color = get_color(index_start);
    } else {
        for(uint i = index_start; i <= index_end; i += 1u){
            if (i == index_start) {
                // First cell, only take the starting part
                output_color += (1. - (pos_start - pos_start_floor)) * get_color(i);
            } else if (i == index_end) {
                // Last cell, only take the ending part
                output_color += (pos_end - pos_end_floor) * get_color(i);
            } else {
                // One of the middle cells, take full value of the cell
                output_color += get_color(i);
            }
        }
        // Divide to create the average of all cells
        output_color /= (pos_end - pos_start);
    }

    float alpha = 1.;

    // Draw grid lines if resolution is low enough
    float cell_size_pixels = widget_size.x / float(effect_data_len);
    if (cell_size_pixels > (1.0 / GRIDLINE_SIZE)) {

        float distance_to_gridline_start = abs(pos_start - round(pos_start));
        float distance_to_gridline_end = abs(pos_end - round(pos_end));

        float distance_to_gridline_min = min(
            distance_to_gridline_start,
            distance_to_gridline_end
        );
        float distance_to_gridline_max = max(
            distance_to_gridline_start,
            distance_to_gridline_end
        );

        alpha = clamp(
            1. - (GRIDLINE_SIZE - distance_to_gridline_min) / (distance_to_gridline_max - distance_to_gridline_min),
            0., 1.
        );
    }

    // Add alpha and do gamma correction
    color = vec4(linear_to_srgb(output_color), alpha);
}
