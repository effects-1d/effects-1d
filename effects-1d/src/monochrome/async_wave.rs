use core::f32::consts::PI;

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
pub struct AsyncWave {
    num_buckets: u16,
    blink_period: u8,
    cycle: MultiBeatCycle,
}

impl ConstructibleBeatBasedEffect for AsyncWave {
    fn init(resolution_hint: Option<u32>, start_beat: i32) -> Self {
        let blink_period = 4;
        let mut num_buckets = 50;

        if let Some(resolution_hint) = resolution_hint {
            if resolution_hint < 2 * u32::from(num_buckets) {
                num_buckets = resolution_hint as u16;
            }
        }

        let effect_duration_periods = num_buckets;

        Self {
            blink_period,
            num_buckets,
            cycle: MultiBeatCycle::new(
                effect_duration_periods * u16::from(blink_period),
                start_beat,
            ),
        }
    }
}

impl BeatBasedEffect for AsyncWave {
    type Color = color::Monochrome;

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::Monochrome>,
        _d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        self.cycle.update(beat);

        let t = (self.cycle.beat().rem_euclid(i32::from(self.blink_period)) as f32
            + beat.fractional)
            / f32::from(self.blink_period);

        let cycle_progress = self.cycle.cycle_progress();

        for i in 0..self.num_buckets {
            let start = f32::from(i) / f32::from(self.num_buckets);
            let end = f32::from(i + 1) / f32::from(self.num_buckets);

            let blink = t + cycle_progress * f32::from(i);
            let color = (2.0 * PI * blink).sin();

            // Seems to look prettier of only half-wave is shown
            //let color = color * 0.5 + 0.5;

            framebuffer.draw_smooth(start, end, color.into(), BlendMode::Add)
        }

        Ok(EffectState {
            idle: self.cycle.is_last_beat_of_cycle(),
        })
    }
}
