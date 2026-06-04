#[allow(unused_imports)]
use effects_1d_common::prelude::*;

use core::f32::consts::PI;
use effects_1d_common::{
    color::{
        self,
        gradients::{ColorGradient, OklabGradient},
    },
    effects::{
        BeatBasedEffect, BeatInfo, BlendMode, ConstructibleBeatBasedEffect, EffectState,
        FrameBufferRef,
    },
    errors::RenderError,
    rhythm::MultiBeatCycle,
};

#[derive(Debug)]
pub struct ColorOrbit {
    cycle: MultiBeatCycle,
    gradient: OklabGradient,
    width: f32,
}

impl ConstructibleBeatBasedEffect for ColorOrbit {
    fn init(resolution_hint: Option<u32>, start_beat: i32) -> Self {
        let mut width: f32 = 0.018;
        if let Some(resolution) = resolution_hint {
            width = width.max(1.5 / resolution as f32);
        }
        Self {
            cycle: MultiBeatCycle::new(12, start_beat),
            gradient: OklabGradient::new(color::rgb8(255, 40, 180), color::rgb8(0, 220, 255)),
            width,
        }
    }
}

impl BeatBasedEffect for ColorOrbit {
    type Color = color::RGB;

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<Self::Color>,
        _d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        self.cycle.update(beat);
        let base = self.cycle.cycle_progress() * 2.0 * PI;
        const ORBS: u16 = 9;
        for orb in 0..ORBS {
            let orb_f = f32::from(orb);
            let phase = base + orb_f * 2.0 * PI / f32::from(ORBS);
            let pos = 0.5 + 0.5 * phase.cos();
            let width = self.width * (1.0 + 0.4 * (phase * 0.5).sin().abs());
            let color = self.gradient.interpolate(orb_f / f32::from(ORBS - 1));
            framebuffer.draw_smooth(pos - width / 2.0, pos + width / 2.0, color, BlendMode::Add);
        }
        Ok(EffectState {
            idle: self.cycle.is_last_beat_of_cycle(),
        })
    }
}
