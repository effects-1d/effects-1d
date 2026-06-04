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
pub struct BinaryMarchingAnts {
    cycle: MultiBeatCycle,
    stripe_count: u16,
    duty: f32,
}

impl ConstructibleBeatBasedEffect for BinaryMarchingAnts {
    fn init(resolution_hint: Option<u32>, start_beat: i32) -> Self {
        let stripe_count = resolution_hint
            .map(|resolution| (resolution / 5).clamp(6, 24) as u16)
            .unwrap_or(12);
        Self {
            cycle: MultiBeatCycle::new(4, start_beat),
            stripe_count,
            duty: 0.5,
        }
    }
}

impl BeatBasedEffect for BinaryMarchingAnts {
    type Color = color::Binary;

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<Self::Color>,
        _d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        self.cycle.update(beat);
        let offset = self.cycle.cycle_progress();
        let stripe_count = f32::from(self.stripe_count);
        let duty = self.duty;
        let len = framebuffer.len().max(1) as f32;

        framebuffer.set_pixels(&|pos| {
            let x = (pos as f32 + 0.5) / len;
            let phase = (x * stripe_count - offset).rem_euclid(1.0);
            if phase < duty {
                color::Binary::on()
            } else {
                color::Binary::off()
            }
        });

        Ok(EffectState {
            idle: self.cycle.is_last_beat_of_cycle(),
        })
    }
}
