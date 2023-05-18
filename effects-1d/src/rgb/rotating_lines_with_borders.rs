#[allow(unused_imports)]
use effects_1d_common::prelude::*;

use effects_1d_common::{
    color,
    drawing::lines::Lines,
    effects::{BeatBasedEffect, BeatInfo, BlendMode, EffectState, FrameBufferRef},
    errors::RenderError,
    rhythm::MultiBeatCycle,
};

#[derive(Debug)]
pub struct RotatingLinesWithBorders {
    border_color: color::RGB,
    line_color: color::RGB,
    cycle: MultiBeatCycle,
    lines: Lines,
}

impl BeatBasedEffect for RotatingLinesWithBorders {
    type Color = color::RGB;

    fn init(resolution_hint: Option<u32>, start_beat: i32) -> Self {
        Self {
            border_color: color::rgb8(0, 0, 255),
            line_color: color::rgb8(0, 255, 255),
            cycle: MultiBeatCycle::new(3, start_beat),
            lines: Lines::new_with_resolution_hint(0.005, 0.035, 0.0, resolution_hint),
        }
    }

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::RGB>,
        _d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        self.cycle.update(beat);

        for line in self.lines.draw_iter(self.cycle.cycle_progress()) {
            framebuffer.draw_smooth(line.start, line.end, self.line_color, BlendMode::Add);
        }

        let border_size = framebuffer.len() / 3;
        let len = border_size as f32;

        for pos in 0..border_size {
            let alpha = 1.0 - (pos as f32) / len;
            framebuffer.update_pixel(pos, self.border_color, BlendMode::Alpha(alpha));
            framebuffer.update_pixel(
                framebuffer.len() - 1 - pos,
                self.border_color,
                BlendMode::Alpha(alpha),
            );
        }

        Ok(EffectState {
            idle: self.cycle.is_last_beat_of_cycle(),
        })
    }
}
