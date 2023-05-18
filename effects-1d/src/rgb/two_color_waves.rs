#[allow(unused_imports)]
use effects_1d_common::prelude::*;

use effects_1d_common::{
    color::{
        self,
        gradients::{ColorGradient, HslGradient, ReversedGradient},
        RGB,
    },
    effects::{BeatBasedEffect, BeatInfo, EffectState, FrameBufferRef},
    errors::RenderError,
    rhythm::MultiBeatCycle,
};

#[derive(Debug)]
pub struct TwoColorWaves {
    color_1: RGB,
    color_2: RGB,
    num_segments: u16,
    cycle: MultiBeatCycle,
}

impl BeatBasedEffect for TwoColorWaves {
    type Color = color::RGB;

    fn init(_resolution_hint: Option<u32>, start_beat: i32) -> Self {
        Self {
            color_1: RGB::new(0, 0, u16::MAX),
            color_2: RGB::new(0, u16::MAX, u16::MAX),
            num_segments: 10,
            cycle: MultiBeatCycle::new(4, start_beat),
        }
    }

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::RGB>,
        _d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        self.cycle.update(beat);

        let mut cycle_pos = self.cycle.cycle_progress();
        if cycle_pos > 0.5 {
            cycle_pos = 1.0 - cycle_pos
        }
        cycle_pos = 4. * cycle_pos - 1.;

        let mut cycle_pos_2 = 1.0 - cycle_pos.abs();

        // More of these make the transition harder, less make it softer.
        cycle_pos_2 = cycle_pos_2 * cycle_pos_2; // Power of 2
        cycle_pos_2 = cycle_pos_2 * cycle_pos_2; // Power of 4

        let cycle_pos_3 = if cycle_pos >= 0.0 {
            1.0 - cycle_pos_2
        } else {
            cycle_pos_2 - 1.0
        };
        let gradient_cycle_pos = cycle_pos_3 * 0.5 + 0.5;

        let gradient = HslGradient::new(self.color_1, self.color_2);
        let color1 = gradient.interpolate(gradient_cycle_pos);
        let color2 = gradient.interpolate(1. - gradient_cycle_pos);

        let gradient1 = HslGradient::new(color1, color2);
        let gradient2 = ReversedGradient::new(&gradient1);

        let segment_size = 1.0 / f32::from(self.num_segments);

        for segment_id in 0..=self.num_segments {
            let pos = f32::from(segment_id) * segment_size;
            if segment_id % 2 == 0 {
                framebuffer.draw_gradient(pos, pos + segment_size, &gradient1);
            } else {
                framebuffer.draw_gradient(pos, pos + segment_size, &gradient2);
            };
        }

        Ok(EffectState {
            idle: self.cycle.is_last_beat_of_cycle(),
        })
    }
}
