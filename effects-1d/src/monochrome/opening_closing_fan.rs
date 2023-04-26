use effects_1d_common::{
    color,
    drawing::lines::Lines,
    effects::{BeatBasedEffect, BeatInfo, BlendMode, EffectState, FrameBufferRef},
    errors::RenderError,
    rhythm::MultiBeatCycle,
};

#[derive(Debug)]
pub struct OpeningClosingFan {
    lines: Lines,
    cycle: MultiBeatCycle,
}

impl BeatBasedEffect for OpeningClosingFan {
    type Color = color::Monochrome;

    fn init(resolution_hint: Option<u32>, start_beat: i32) -> Self {
        let line_width = 0.03;
        Self {
            lines: Lines::new_with_resolution_hint(0.0, line_width, 0.5, resolution_hint),
            cycle: MultiBeatCycle::new(12, start_beat),
        }
    }

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::Monochrome>,
        _d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        self.cycle.update(beat);

        let progress = self.cycle.cycle_progress();

        let coverage = if progress < 0.5 {
            progress * 2.0
        } else {
            (1.0 - progress) * 2.0
        };
        let draw_offset = (progress * 2.0 - 1.0).clamp(0.0, 1.0);

        self.lines
            .set_line_width(coverage * self.lines.get_line_stride());

        for line in self.lines.draw_iter(draw_offset) {
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
