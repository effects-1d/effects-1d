#[allow(unused_imports)]
use effects_1d_common::prelude::*;

use effects_1d_common::{
    color,
    effects::{
        BeatBasedEffect, BeatInfo, BlendMode, ConstructibleBeatBasedEffect, EffectState,
        FrameBufferRef,
    },
    errors::RenderError,
    random::{EffectRng, RngExt},
    rhythm::{BeatMultiplier, MultiBeatCycle},
};

#[derive(Clone, Copy, Debug)]
struct Slice {
    start: f32,
    width: f32,
    color: color::RGB,
}

#[derive(Debug)]
pub struct GlitchBurst {
    rng: EffectRng,
    cycle: MultiBeatCycle,
    beat_multiplier: BeatMultiplier,
    slices: [Slice; 16],
}

impl GlitchBurst {
    fn regenerate(&mut self) {
        for slice in &mut self.slices {
            slice.start = self.rng.random_range(0.0..0.98);
            slice.width = self.rng.random_range(0.008..0.12);
            slice.color = color::hsv8(
                self.rng.random::<f32>() * 360.0,
                1.0,
                self.rng.random_range(0.45..1.0),
            );
        }
    }
}

impl ConstructibleBeatBasedEffect for GlitchBurst {
    fn init(_resolution_hint: Option<u32>, start_beat: i32) -> Self {
        let mut this = Self {
            rng: EffectRng::new(),
            cycle: MultiBeatCycle::new(4, start_beat),
            beat_multiplier: BeatMultiplier::new(10, start_beat),
            slices: [Slice {
                start: 0.0,
                width: 0.0,
                color: color::RGB::new(0, 0, 0),
            }; 16],
        };
        this.regenerate();
        this
    }
}

impl BeatBasedEffect for GlitchBurst {
    type Color = color::RGB;

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<Self::Color>,
        _d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        self.cycle.update(beat);
        let fast = self.beat_multiplier.generate_beat(beat);
        if fast.is_new_beat {
            self.regenerate();
        }

        let mut background = color::RGB::new(0, 0, 0);
        if fast.current.rem_euclid(5) == 0 {
            background = color::hsv8(self.cycle.cycle_progress() * 360.0, 0.6, 0.08);
        }
        framebuffer.draw_smooth(0.0, 1.0, background, BlendMode::None);

        for slice in &self.slices {
            framebuffer.draw_smooth(
                slice.start,
                slice.start + slice.width,
                slice.color,
                BlendMode::Add,
            );
        }

        Ok(EffectState {
            idle: self.cycle.is_last_beat_of_cycle(),
        })
    }
}
