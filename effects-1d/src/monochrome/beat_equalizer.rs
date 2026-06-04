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
pub struct BeatEqualizer {
    cycle: MultiBeatCycle,
    buckets: u16,
}

fn brightness(v: f32) -> color::Monochrome {
    color::Monochrome::new((f32::from(u16::MAX) * v.clamp(0.0, 1.0)) as u16)
}

fn hash(bucket: u16, beat: i32) -> f32 {
    let mut x = bucket as u32;
    x = x.wrapping_mul(1_664_525).wrapping_add(beat as u32);
    x ^= x >> 16;
    x = x.wrapping_mul(2_246_822_519);
    ((x >> 8) & 0xffff) as f32 / 65_535.0
}

impl ConstructibleBeatBasedEffect for BeatEqualizer {
    fn init(resolution_hint: Option<u32>, start_beat: i32) -> Self {
        let buckets = resolution_hint
            .map(|resolution| (resolution / 6).clamp(6, 18) as u16)
            .unwrap_or(12);
        Self {
            cycle: MultiBeatCycle::new(4, start_beat),
            buckets,
        }
    }
}

impl BeatBasedEffect for BeatEqualizer {
    type Color = color::Monochrome;

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<Self::Color>,
        _d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        self.cycle.update(beat);
        let beat_phase = beat.fractional;
        let attack = if beat_phase < 0.18 {
            beat_phase / 0.18
        } else {
            (1.0 - (beat_phase - 0.18) / 0.82).clamp(0.0, 1.0)
        };

        for bucket in 0..self.buckets {
            let target = 0.25 + hash(bucket, beat.current) * 0.75;
            let phase_bias = if (i32::from(bucket) + beat.current).rem_euclid(4) == 0 {
                1.0
            } else {
                0.45
            };
            let level = target * attack * phase_bias;
            let start = f32::from(bucket) / f32::from(self.buckets);
            let end = f32::from(bucket + 1) / f32::from(self.buckets);
            framebuffer.draw_smooth(start, end, brightness(level), BlendMode::Add);
        }

        Ok(EffectState {
            idle: self.cycle.is_last_beat_of_cycle(),
        })
    }
}
