#[allow(unused_imports)]
use effects_1d_common::prelude::*;

use effects_1d_common::{
    color,
    effects::{
        BeatBasedEffect, BeatInfo, BlendMode, ConstructibleBeatBasedEffect, EffectState,
        FrameBufferRef,
    },
    errors::RenderError,
    rhythm::MultiBeatCycle,
};

#[derive(Debug)]
pub struct MeasureChaser {
    cycle: MultiBeatCycle,
}

impl ConstructibleBeatBasedEffect for MeasureChaser {
    fn init(_resolution_hint: Option<u32>, start_beat: i32) -> Self {
        Self {
            cycle: MultiBeatCycle::new(4, start_beat),
        }
    }
}

impl BeatBasedEffect for MeasureChaser {
    type Color = color::RGB;

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<Self::Color>,
        _d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        self.cycle.update(beat);
        let beat_in_measure = self.cycle.beat().rem_euclid(4);
        let local = beat.fractional;
        let section_width = 0.25;

        for section in 0..4i32 {
            let start = section as f32 * section_width;
            let end = start + section_width;
            let color = color::hsv8(section as f32 * 90.0 + self.cycle.cycle_number() as f32 * 30.0, 0.85, 1.0);
            if section < beat_in_measure {
                framebuffer.draw_smooth(start, end, color, BlendMode::None);
            } else if section == beat_in_measure {
                let wipe_end = start + section_width * local;
                framebuffer.draw_smooth(start, wipe_end, color, BlendMode::None);
            }
        }

        if beat_in_measure == 3 {
            let flash = (local * core::f32::consts::PI).sin();
            let c = color::hsv8(self.cycle.cycle_progress() * 360.0, 0.25, 0.35 * flash);
            framebuffer.draw_smooth(0.0, 1.0, c, BlendMode::Add);
        }

        Ok(EffectState {
            idle: self.cycle.is_last_beat_of_cycle(),
        })
    }
}
