#[allow(unused_imports)]
use effects_1d_common::prelude::*;

use effects_1d_common::{
    color,
    effects::{
        BeatBasedEffect, BeatInfo, ConstructibleBeatBasedEffect, EffectState, FrameBufferRef,
    },
    errors::RenderError,
    rhythm::MultiBeatCycle,
};

#[derive(Debug)]
pub struct Wiping {
    cycle: MultiBeatCycle,
}

impl ConstructibleBeatBasedEffect for Wiping {
    fn init(_resolution_hint: Option<u32>, start_beat: i32) -> Self {
        Self {
            cycle: MultiBeatCycle::new(3, start_beat),
        }
    }
}

impl BeatBasedEffect for Wiping {
    type Color = color::Binary;

    // GOAL: fill left to right, empty left to right, fill right to left, empty right to left.

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::Binary>,
        _d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        self.cycle.update(beat);

        let sequence_id = self.cycle.cycle_number().rem_euclid(4);

        match sequence_id {
            0 => framebuffer.draw_sharp(0.0, self.cycle.cycle_progress(), color::Binary::on()),
            1 => framebuffer.draw_sharp(self.cycle.cycle_progress(), 1.0, color::Binary::on()),
            2 => {
                framebuffer.draw_sharp(1.0 - self.cycle.cycle_progress(), 1.0, color::Binary::on())
            }
            _ => {
                framebuffer.draw_sharp(0.0, 1.0 - self.cycle.cycle_progress(), color::Binary::on())
            }
        }

        Ok(EffectState {
            idle: self.cycle.is_last_beat_of_cycle() && (sequence_id.rem_euclid(2) == 1),
        })
    }
}
