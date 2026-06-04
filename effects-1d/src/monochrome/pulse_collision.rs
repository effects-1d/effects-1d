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
pub struct PulseCollision {
    cycle: MultiBeatCycle,
    width: f32,
}

fn brightness(v: f32) -> color::Monochrome {
    color::Monochrome::new((f32::from(u16::MAX) * v.clamp(0.0, 1.0)) as u16)
}

fn smoothstep(x: f32) -> f32 {
    let x = x.clamp(0.0, 1.0);
    x * x * (3.0 - 2.0 * x)
}

impl ConstructibleBeatBasedEffect for PulseCollision {
    fn init(resolution_hint: Option<u32>, start_beat: i32) -> Self {
        let mut width: f32 = 0.025;
        if let Some(resolution) = resolution_hint {
            width = width.max(2.0 / resolution as f32);
        }
        Self {
            cycle: MultiBeatCycle::new(8, start_beat),
            width,
        }
    }
}

impl BeatBasedEffect for PulseCollision {
    type Color = color::Monochrome;

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<Self::Color>,
        _d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        self.cycle.update(beat);
        let p = self.cycle.cycle_progress();

        if p < 0.45 {
            let travel = smoothstep(p / 0.45) * 0.5;
            framebuffer.draw_smooth(
                travel - self.width / 2.0,
                travel + self.width / 2.0,
                color::Monochrome::full(),
                BlendMode::Add,
            );
            framebuffer.draw_smooth(
                1.0 - travel - self.width / 2.0,
                1.0 - travel + self.width / 2.0,
                color::Monochrome::full(),
                BlendMode::Add,
            );
        } else if p < 0.58 {
            let flash = 1.0 - ((p - 0.45) / 0.13).clamp(0.0, 1.0);
            framebuffer.draw_smooth(0.0, 1.0, brightness(0.15 * flash), BlendMode::Add);
            framebuffer.draw_smooth(0.45, 0.55, color::Monochrome::full(), BlendMode::Add);
        } else {
            let burst = smoothstep((p - 0.58) / 0.42);
            for fragment in 0..4u16 {
                let strength = 1.0 - f32::from(fragment) * 0.18;
                let offset = burst * (0.08 + f32::from(fragment) * 0.095);
                let w = self.width * (1.0 - f32::from(fragment) * 0.08);
                let c = brightness(strength * (1.0 - burst * 0.6));
                framebuffer.draw_smooth(
                    0.5 - offset - w / 2.0,
                    0.5 - offset + w / 2.0,
                    c,
                    BlendMode::Add,
                );
                framebuffer.draw_smooth(
                    0.5 + offset - w / 2.0,
                    0.5 + offset + w / 2.0,
                    c,
                    BlendMode::Add,
                );
            }
        }

        Ok(EffectState {
            idle: self.cycle.is_last_beat_of_cycle(),
        })
    }
}
