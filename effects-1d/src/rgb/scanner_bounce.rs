#[allow(unused_imports)]
use effects_1d_common::prelude::*;

use effects_1d_common::{
    color::{self, BlendableColor, Color},
    effects::{
        BeatBasedEffect, BeatInfo, BlendMode, ConstructibleBeatBasedEffect, EffectState,
        FrameBufferRef,
    },
    errors::RenderError,
    rhythm::MultiBeatCycle,
};

#[derive(Debug)]
pub struct ScannerBounce {
    cycle: MultiBeatCycle,
    width: f32,
}

impl ConstructibleBeatBasedEffect for ScannerBounce {
    fn init(resolution_hint: Option<u32>, start_beat: i32) -> Self {
        let mut width: f32 = 0.025;
        if let Some(resolution) = resolution_hint {
            width = width.max(1.5 / resolution as f32);
        }
        Self {
            cycle: MultiBeatCycle::new(8, start_beat),
            width,
        }
    }
}

impl BeatBasedEffect for ScannerBounce {
    type Color = color::RGB;

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<Self::Color>,
        _d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        self.cycle.update(beat);
        let p = self.cycle.cycle_progress();
        let pos = if p < 0.5 { p * 2.0 } else { 2.0 - p * 2.0 };
        let hue = 360.0 * self.cycle.cycle_progress();
        let head_color = color::hsv8(hue, 1.0, 1.0);

        for segment in 0..8u16 {
            let rel = f32::from(segment) / 8.0;
            let offset = rel * 0.14 * if p < 0.5 { -1.0 } else { 1.0 };
            let center = pos + offset;
            let c = color::RGB::zero().elementwise_lerp(head_color, (1.0 - rel).powf(2.0));
            framebuffer.draw_smooth(
                center - self.width / 2.0,
                center + self.width / 2.0,
                c,
                BlendMode::Add,
            );
        }

        Ok(EffectState {
            idle: self.cycle.is_last_beat_of_cycle(),
        })
    }
}
