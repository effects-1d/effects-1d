#[allow(unused_imports)]
use effects_1d_common::prelude::*;

use effects_1d_common::{
    color::{self, Monochrome},
    effects::{BeatBasedEffect, BeatInfo, BlendMode, EffectState, FrameBufferRef},
    errors::RenderError,
    lerp::Lerp,
    random::{EffectRng, Rng},
    rhythm::BeatMultiplier,
};

#[derive(Debug)]
pub struct BinaryExplosions {
    rng: EffectRng,
    explosions: [Option<Explosion>; 5],
    multiplier: BeatMultiplier,
}

#[derive(Debug)]
struct Explosion {
    position: f32,
    start_beat: i32,
    start_subbeat: f32,
    duration: f32,
    movespeed: f32,
    fill: f32,
}

impl Explosion {
    pub fn new(beat: BeatInfo, rng: &mut EffectRng) -> Self {
        let diameter = rng.gen_range(0.01..0.3);
        let duration = rng.gen_range(1.0..5.0) * 2.0;
        let movespeed = diameter / duration;

        Self {
            position: rng.gen_range(0.1..0.9),
            start_beat: beat.current,
            start_subbeat: beat.fractional,
            duration,
            movespeed,
            fill: rng.gen_range(0.05..0.5),
        }
    }

    pub fn render(
        &self,
        beat: BeatInfo,
        framebuffer: &mut dyn FrameBufferRef<color::Monochrome>,
    ) -> bool {
        let delta_beat = beat.current - self.start_beat;
        let delta_subbeat = beat.fractional - self.start_subbeat;

        let progress = (delta_beat as f32 + delta_subbeat) / self.duration;
        if progress > 1.0 {
            return false;
        }

        let endpoint = self.movespeed * self.duration;

        let outer = endpoint.lerp(0.0, 1.0 - progress);
        let inner = endpoint.lerp(0.0, (1.0 - progress) / (1.0 - self.fill));

        if inner > 0.0 {
            framebuffer.draw_smooth(
                self.position - outer,
                self.position - inner,
                Monochrome::full(),
                BlendMode::Add,
            );
            framebuffer.draw_smooth(
                self.position + inner,
                self.position + outer,
                Monochrome::full(),
                BlendMode::Add,
            );
        } else {
            framebuffer.draw_smooth(
                self.position - outer,
                self.position + outer,
                Monochrome::full(),
                BlendMode::Add,
            );
        }

        true
    }
}

impl BeatBasedEffect for BinaryExplosions {
    type Color = color::Monochrome;

    fn init(_resolution_hint: Option<u32>, start_beat: i32) -> Self {
        Self {
            explosions: Default::default(),
            rng: EffectRng::new(),
            multiplier: BeatMultiplier::new(2, start_beat),
        }
    }

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::Monochrome>,
        _d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        let beat = self.multiplier.generate_beat(beat);

        for explosion in &mut self.explosions {
            *explosion = explosion.take().filter(|e| e.render(beat, framebuffer));
        }

        if beat.is_new_beat {
            for explosion in &mut self.explosions {
                if explosion.is_none() {
                    *explosion = Some(Explosion::new(beat, &mut self.rng));
                    break;
                }
            }
        }

        Ok(EffectState { idle: true })
    }
}
