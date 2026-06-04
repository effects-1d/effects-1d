#[allow(unused_imports)]
use effects_1d_common::prelude::*;

use effects_1d_common::{
    color::{self, BlendableColor, Color},
    effects::{
        BeatBasedEffect, BeatInfo, BlendMode, ConstructibleBeatBasedEffect, EffectState,
        FrameBufferRef,
    },
    errors::RenderError,
    random::{EffectRng, RngExt},
    rhythm::BeatMultiplier,
};

#[derive(Clone, Copy, Debug)]
struct Ripple {
    position: f32,
    start_beat: i32,
    start_fractional: f32,
    duration: f32,
    max_radius: f32,
    color: color::RGB,
}

impl Ripple {
    fn new(beat: BeatInfo, rng: &mut EffectRng) -> Self {
        Self {
            position: rng.random_range(0.08..0.92),
            start_beat: beat.current,
            start_fractional: beat.fractional,
            duration: rng.random_range(3.0..7.0),
            max_radius: rng.random_range(0.18..0.48),
            color: color::hsv8(rng.random::<f32>() * 360.0, 0.85, 1.0),
        }
    }

    fn render(
        self,
        beat: BeatInfo,
        framebuffer: &mut dyn FrameBufferRef<color::RGB>,
    ) -> Option<Self> {
        let age = (beat.current - self.start_beat) as f32 + beat.fractional - self.start_fractional;
        let progress = age / self.duration;
        if !(0.0..=1.0).contains(&progress) {
            return None;
        }
        let radius = self.max_radius * progress;
        let width = 0.012 + 0.025 * (1.0 - progress);
        let intensity = (1.0 - progress).powf(2.0);
        let color = color::RGB::zero().elementwise_lerp(self.color, intensity);
        framebuffer.draw_smooth(
            self.position - radius - width,
            self.position - radius + width,
            color,
            BlendMode::Add,
        );
        framebuffer.draw_smooth(
            self.position + radius - width,
            self.position + radius + width,
            color,
            BlendMode::Add,
        );
        Some(self)
    }
}

#[derive(Debug)]
pub struct RippleField {
    rng: EffectRng,
    ripples: [Option<Ripple>; 8],
    multiplier: BeatMultiplier,
}

impl ConstructibleBeatBasedEffect for RippleField {
    fn init(_resolution_hint: Option<u32>, start_beat: i32) -> Self {
        Self {
            rng: EffectRng::new(),
            ripples: core::array::from_fn(|_| None),
            multiplier: BeatMultiplier::new(2, start_beat),
        }
    }
}

impl BeatBasedEffect for RippleField {
    type Color = color::RGB;

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<Self::Color>,
        _d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        let beat = self.multiplier.generate_beat(beat);
        for ripple in &mut self.ripples {
            *ripple = ripple
                .take()
                .and_then(|ripple| ripple.render(beat, framebuffer));
        }
        if beat.is_new_beat {
            for ripple in &mut self.ripples {
                if ripple.is_none() {
                    *ripple = Some(Ripple::new(beat, &mut self.rng));
                    break;
                }
            }
        }
        Ok(EffectState { idle: true })
    }
}
