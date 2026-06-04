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
pub struct ScannerBounce {
    cycle: MultiBeatCycle,
    width: f32,
}

impl ConstructibleBeatBasedEffect for ScannerBounce {
    fn init(resolution_hint: Option<u32>, start_beat: i32) -> Self {
        let mut width: f32 = 0.035;
        if let Some(resolution) = resolution_hint {
            width = width.max(2.0 / resolution as f32);
        }
        Self {
            cycle: MultiBeatCycle::new(8, start_beat),
            width,
        }
    }
}

impl BeatBasedEffect for ScannerBounce {
    type Color = color::Binary;

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<Self::Color>,
        _d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        self.cycle.update(beat);
        let p = self.cycle.cycle_progress();
        let pos = if p < 0.5 { p * 2.0 } else { 2.0 - p * 2.0 };
        let start = pos * (1.0 - self.width);
        framebuffer.draw_sharp(start, start + self.width, color::Binary::on());

        Ok(EffectState {
            idle: self.cycle.is_last_beat_of_cycle(),
        })
    }
}
