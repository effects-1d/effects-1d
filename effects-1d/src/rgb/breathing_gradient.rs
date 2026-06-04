#[allow(unused_imports)]
use effects_1d_common::prelude::*;

use core::f32::consts::PI;
use effects_1d_common::{
    color::{self, gradients::HsvRainbowGradient},
    effects::{
        BeatBasedEffect, BeatInfo, ConstructibleBeatBasedEffect, EffectState, FrameBufferRef,
    },
    errors::RenderError,
    rhythm::MultiBeatCycle,
};

#[derive(Debug)]
pub struct BreathingGradient {
    cycle: MultiBeatCycle,
}

impl ConstructibleBeatBasedEffect for BreathingGradient {
    fn init(_resolution_hint: Option<u32>, start_beat: i32) -> Self {
        Self {
            cycle: MultiBeatCycle::new(16, start_beat),
        }
    }
}

impl BeatBasedEffect for BreathingGradient {
    type Color = color::RGB;

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<Self::Color>,
        _d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        self.cycle.update(beat);
        let p = self.cycle.cycle_progress();
        let breath = 0.5 - 0.5 * (2.0 * PI * p).cos();
        framebuffer.draw_gradient(
            0.0,
            1.0,
            &HsvRainbowGradient {
                saturation: 0.55 + 0.45 * breath,
                brightness: 0.20 + 0.80 * breath,
                offset: 360.0 * p,
                scale: 0.35 + 0.35 * breath,
                reversed: false,
            },
        );
        Ok(EffectState {
            idle: self.cycle.is_last_beat_of_cycle(),
        })
    }
}
