use effects_1d_common::{
    color,
    effects::{BeatBasedEffect, BeatInfo, BlendMode, EffectState, FrameBufferRef},
    errors::RenderError,
};

#[derive(Debug)]
pub struct SplittingLine {
    line_width_half: f32,
    line_speed: f32,
    split_distance: i32,
    start_beat: Option<i32>,
}

impl BeatBasedEffect for SplittingLine {
    type Color = color::Monochrome;

    fn init(resolution_hint: Option<u32>) -> Self {
        let mut line_width: f32 = 0.003;
        let mut line_speed: f32 = 0.01;
        let split_distance: i32 = 2;

        if let Some(resolution_hint) = resolution_hint {
            let pixel_size = 1.0 / resolution_hint as f32;
            line_width = line_width.max(1.7 * pixel_size);

            // let distance_between_lines = 2.0 * split_distance as f32 * line_speed;
            // Distance should be at least 5 * the line width
            let desired_min_distance_between_lines = 8.0 * line_width;
            let desired_min_line_speed =
                desired_min_distance_between_lines / (2.0 * split_distance as f32);
            if line_speed < desired_min_line_speed {
                line_speed = desired_min_line_speed;
            }
        }

        Self {
            line_width_half: line_width / 2.0,
            line_speed,
            split_distance,
            start_beat: None,
        }
    }

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::Monochrome>,
        _d_t: f32,
        mut beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        let start_beat = *self
            .start_beat
            .get_or_insert_with(|| beat.next_full_beat().current);

        let split_distance = self.split_distance;
        let split_distance_f = self.split_distance as f32;

        beat.current -= start_beat;
        // Display nothing until the start of the next beat
        if beat.current < 0 {
            return Ok(EffectState {
                over: false,
                idle: false,
            });
        }

        let num_splits = beat.current / split_distance;
        let cycle_position =
            (beat.fractional + (beat.current % (split_distance * 2)) as f32) / split_distance_f;

        let mut draw_line = |offset: f32| {
            framebuffer.draw_smooth(
                0.5 + offset * self.line_speed * split_distance_f - self.line_width_half,
                0.5 + offset * self.line_speed * split_distance_f + self.line_width_half,
                color::Monochrome::new(255),
                BlendMode::Max,
            );
            framebuffer.draw_smooth(
                0.5 - offset * self.line_speed * split_distance_f - self.line_width_half,
                0.5 - offset * self.line_speed * split_distance_f + self.line_width_half,
                color::Monochrome::new(255),
                BlendMode::Max,
            );
        };

        let num_segments = (0.5 / (self.line_speed * split_distance_f)) as i32 + 2; // +2 to make sure no jumping happens at the corners
        for seg_id in 0..num_segments {
            if seg_id > num_splits {
                break;
            }

            let offset = (seg_id + 1) / 2;
            let offset = if seg_id % 2 == 0 {
                offset as f32
            } else {
                -offset as f32
            };
            draw_line(cycle_position + 2.0 * offset);
        }

        Ok(EffectState {
            over: false,
            idle: num_splits > num_segments,
        })
    }
}
