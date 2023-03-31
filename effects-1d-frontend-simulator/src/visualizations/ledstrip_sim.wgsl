@group(1) @binding(0) var<storage> effect_data: array<u32>;
@group(1) @binding(1) var<uniform> widget_size: vec2<f32>;

struct FragmentInput {
    #import bevy_pbr::mesh_vertex_output
}

fn get_color(index: u32) -> vec3<f32>{
    let color = effect_data[index];
    let color_r = (color >> 0u) & 0xffu;
    let color_g = (color >> 8u) & 0xffu;
    let color_b = (color >> 16u) & 0xffu;
    return vec3<f32>(
        f32(color_r) / 255.0,
        f32(color_g) / 255.0,
        f32(color_b) / 255.0,
    );
}

const GRIDLINE_SIZE: f32 = 0.15;

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    let pixel_size = 1.0 / widget_size.x;

    let effect_data_len = arrayLength(&effect_data);
    let effect_data_max_index = effect_data_len - 1u;

    let pos_start = (in.uv.x - 0.5*pixel_size) * f32(effect_data_len);
    let pos_end = (in.uv.x + 0.5*pixel_size) * f32(effect_data_len);

    let pos_start_floor = floor(pos_start);
    let pos_end_floor = floor(pos_end);

    let index_start = clamp(u32(pos_start_floor), 0u, effect_data_max_index);
    let index_end = clamp(u32(pos_end_floor), 0u, effect_data_max_index);

    var output_color = vec3<f32>(0., 0., 0.);
    if(index_end <= index_start) {
        // If we start and end in the same cell, the average is equal to the cell value
        output_color = get_color(index_start);
    } else {
        for(var i = index_start; i <= index_end; i += 1u){
            if i == index_start {
                // First cell, only take the starting part
                output_color += (1. - (pos_start - pos_start_floor)) * get_color(i);
            } else if i == index_end {
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

    var alpha = 1.;

    // Draw grid lines if resolution is low enough
    let cell_size_pixels = widget_size.x / f32(effect_data_len);
    if cell_size_pixels > 1.0 / GRIDLINE_SIZE {

        let distance_to_gridline_start = abs(pos_start - round(pos_start));
        let distance_to_gridline_end = abs(pos_end - round(pos_end));

        let distance_to_gridline_min = min(
            distance_to_gridline_start,
            distance_to_gridline_end
        );
        let distance_to_gridline_max = max(
            distance_to_gridline_start,
            distance_to_gridline_end
        );

        alpha = clamp(
            1. - (GRIDLINE_SIZE - distance_to_gridline_min) / (distance_to_gridline_max - distance_to_gridline_min),
            0., 1.
        );
    }

    return vec4(output_color, alpha);
}
