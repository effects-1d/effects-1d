use effects_1d_common::{
    color,
    effects::{BeatBasedEffect, BeatInfo, EffectState, FrameBufferRef},
    errors::RenderError,
};

pub struct SplittingLine {
    line_width_half: f32,
    line_speed: f32,
    split_distance: i32,
    start_beat: Option<i32>,
}

impl BeatBasedEffect for SplittingLine {
    type Color = color::Monochrome;

    fn init() -> Self {
        Self {
            line_width_half: 0.0015,
            line_speed: 0.01,
            split_distance: 2,
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
            framebuffer.draw_sharp(
                0.5 + offset * self.line_speed * split_distance_f - self.line_width_half,
                0.5 + offset * self.line_speed * split_distance_f + self.line_width_half,
                color::Monochrome::new(255),
            );
            framebuffer.draw_sharp(
                0.5 - offset * self.line_speed * split_distance_f - self.line_width_half,
                0.5 - offset * self.line_speed * split_distance_f + self.line_width_half,
                color::Monochrome::new(255),
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
