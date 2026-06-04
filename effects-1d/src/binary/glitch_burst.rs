#[allow(unused_imports)]
use effects_1d_common::prelude::*;

use effects_1d_common::{
    color,
    effects::{
        BeatBasedEffect, BeatInfo, ConstructibleBeatBasedEffect, EffectState, FrameBufferRef,
    },
    errors::RenderError,
    random::{EffectRng, RngExt},
    rhythm::{BeatMultiplier, MultiBeatCycle},
};

#[derive(Clone, Copy, Debug, Default)]
struct Slice {
    start: f32,
    width: f32,
    enabled: bool,
}

#[derive(Debug)]
pub struct GlitchBurst {
    rng: EffectRng,
    cycle: MultiBeatCycle,
    beat_multiplier: BeatMultiplier,
    slices: [Slice; 14],
}

impl GlitchBurst {
    fn regenerate(&mut self) {
        for slice in &mut self.slices {
            slice.start = self.rng.random_range(0.0..0.98);
            slice.width = self.rng.random_range(0.01..0.12);
            slice.enabled = self.rng.random::<u8>() % 3 != 0;
        }
    }
}

impl ConstructibleBeatBasedEffect for GlitchBurst {
    fn init(_resolution_hint: Option<u32>, start_beat: i32) -> Self {
        let mut this = Self {
            rng: EffectRng::new(),
            cycle: MultiBeatCycle::new(4, start_beat),
            beat_multiplier: BeatMultiplier::new(8, start_beat),
            slices: [Slice::default(); 14],
        };
        this.regenerate();
        this
    }
}

impl BeatBasedEffect for GlitchBurst {
    type Color = color::Binary;

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<Self::Color>,
        _d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        self.cycle.update(beat);
        let fast_beat = self.beat_multiplier.generate_beat(beat);
        if fast_beat.is_new_beat {
            self.regenerate();
        }

        for slice in &self.slices {
            if slice.enabled {
                framebuffer.draw_sharp(slice.start, slice.start + slice.width, color::Binary::on());
            }
        }

        Ok(EffectState {
            idle: self.cycle.is_last_beat_of_cycle(),
        })
    }
}
