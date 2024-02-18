use core::f32::consts::PI;

#[allow(unused_imports)]
use effects_1d_common::prelude::*;

use effects_1d_common::{
    color::{self, Monochrome},
    drawing::lines::Lines,
    effects::{
        BeatBasedEffect, BeatInfo, BlendMode, ConstructibleBeatBasedEffect, EffectState,
        FrameBufferRef,
    },
    errors::RenderError,
    rhythm::MultiBeatCycle,
};

#[derive(Debug)]
pub struct RotatingLinesFilled {
    cycle: MultiBeatCycle,
    lines: Lines,
    inner: f32,
    inner_velocity: f32,
    inner_frequency: f32,
}

impl ConstructibleBeatBasedEffect for RotatingLinesFilled {
    fn init(resolution_hint: Option<u32>, start_beat: i32) -> Self {
        Self {
            cycle: MultiBeatCycle::new(12, start_beat),
            lines: Lines::new_with_resolution_hint(0.065, 0.070, 0.0, resolution_hint),
            inner: 0.0,
            inner_velocity: 1.5,
            inner_frequency: 20.0,
        }
    }
}

impl BeatBasedEffect for RotatingLinesFilled {
    type Color = color::Monochrome;

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::Monochrome>,
        d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        self.cycle.update(beat);

        self.inner += (self.inner_velocity * d_t).rem_euclid(2.0 * PI);

        framebuffer.draw_sharp(0.0, 1.0, Monochrome::full());

        for line in self.lines.draw_iter(self.cycle.cycle_progress()) {
            let pos = line.start * self.inner_frequency + self.inner;
            let value = 0.5 * pos.sin() + 0.5;
            let value = 0.4 * value + 0.1;
            framebuffer.draw_smooth(
                line.start,
                line.end,
                color::Monochrome::new((value * f32::from(u16::MAX)) as u16),
                BlendMode::None,
            );
        }

        Ok(EffectState {
            idle: self.cycle.is_last_beat_of_cycle(),
        })
    }
}
