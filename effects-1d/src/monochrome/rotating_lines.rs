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
pub struct RotatingLines {
    cycle: MultiBeatCycle,
    lines: Lines,
}

impl BeatBasedEffect for RotatingLines {
    type Color = color::Monochrome;

    fn init(resolution_hint: Option<u32>, start_beat: i32) -> Self {
        Self {
            cycle: MultiBeatCycle::new(3, start_beat),
            lines: Lines::new_with_resolution_hint(0.005, 0.035, 0.0, resolution_hint),
        }
    }

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::Monochrome>,
        _d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        self.cycle.update(beat);

        for line in self.lines.draw_iter(self.cycle.cycle_progress()) {
            framebuffer.draw_smooth(
                line.start,
                line.end,
                color::Monochrome::full(),
                BlendMode::Add,
            );
        }

        Ok(EffectState {
            idle: self.cycle.is_last_beat_of_cycle(),
        })
    }
}
