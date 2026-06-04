#[allow(unused_imports)]
use effects_1d_common::prelude::*;

use core::f32::consts::PI;
use effects_1d_common::{
    color,
    effects::{
        BeatBasedEffect, BeatInfo, ConstructibleBeatBasedEffect, EffectState, FrameBufferRef,
    },
    errors::RenderError,
    rhythm::MultiBeatCycle,
};

#[derive(Debug)]
pub struct AuroraRibbon {
    cycle: MultiBeatCycle,
}

impl ConstructibleBeatBasedEffect for AuroraRibbon {
    fn init(_resolution_hint: Option<u32>, start_beat: i32) -> Self {
        Self {
            cycle: MultiBeatCycle::new(24, start_beat),
        }
    }
}

impl BeatBasedEffect for AuroraRibbon {
    type Color = color::RGB;

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<Self::Color>,
        _d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        self.cycle.update(beat);
        let mut t = self.cycle.cycle_number() as f32 + self.cycle.cycle_progress();
        let len = framebuffer.len().max(1) as f32;
        t *= 2.0;
        framebuffer.set_pixels(&|pos| {
            let x = (pos as f32 + 0.5) / len;
            let wave_a = (x * 2.0 * PI + t * 1.7).sin();
            let wave_b = (x * 5.0 * PI - t * 0.9).sin();
            let wave_c = (x * 9.0 * PI + t * 0.45).cos();
            let brightness =
                (0.28 + 0.20 * wave_a + 0.16 * wave_b + 0.10 * wave_c).clamp(0.02, 0.85);
            let hue = 170.0 + 65.0 * wave_a + 30.0 * wave_c + t * 18.0;
            let saturation = (0.55 + 0.25 * wave_b).clamp(0.35, 0.9);
            color::hsv8(hue, saturation, brightness)
        });
        Ok(EffectState { idle: true })
    }
}
