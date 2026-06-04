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
pub struct PulseCollision {
    cycle: MultiBeatCycle,
    width: f32,
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
    type Color = color::RGB;

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<Self::Color>,
        _d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        self.cycle.update(beat);
        let p = self.cycle.cycle_progress();
        let left = color::hsv8(210.0 + p * 120.0, 1.0, 1.0);
        let right = color::hsv8(20.0 + p * 120.0, 1.0, 1.0);

        if p < 0.45 {
            let travel = smoothstep(p / 0.45) * 0.5;
            framebuffer.draw_smooth(
                travel - self.width / 2.0,
                travel + self.width / 2.0,
                left,
                BlendMode::Add,
            );
            framebuffer.draw_smooth(
                1.0 - travel - self.width / 2.0,
                1.0 - travel + self.width / 2.0,
                right,
                BlendMode::Add,
            );
        } else if p < 0.58 {
            let flash = 1.0 - ((p - 0.45) / 0.13).clamp(0.0, 1.0);
            let flash_color =
                color::RGB::zero().elementwise_lerp(color::rgb8(255, 255, 255), flash);
            framebuffer.draw_smooth(0.0, 1.0, flash_color, BlendMode::Add);
            framebuffer.draw_smooth(0.43, 0.57, color::rgb8(255, 255, 255), BlendMode::Add);
        } else {
            let burst = smoothstep((p - 0.58) / 0.42);
            for fragment in 0..5u16 {
                let rel = f32::from(fragment) / 5.0;
                let offset = burst * (0.07 + rel * 0.42);
                let w = self.width * (1.0 - rel * 0.3);
                let c = color::RGB::zero().elementwise_lerp(
                    color::hsv8(360.0 * rel + 180.0 * p, 1.0, 1.0),
                    (1.0 - burst * 0.65) * (1.0 - rel * 0.25),
                );
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
