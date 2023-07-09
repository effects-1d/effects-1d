#[allow(unused_imports)]
use effects_1d_common::prelude::*;

use effects_1d_common::{
    color::{
        self,
        gradients::{ColorGradient, OklabGradient},
    },
    effects::{BeatBasedEffect, BeatInfo, BlendMode, EffectState, FrameBufferRef},
    errors::RenderError,
    rhythm::MultiBeatCycle,
};

use core::f32::consts::PI;

#[derive(Debug)]
pub struct SegmentedSnake {
    cycle: MultiBeatCycle,
    line_count: u16,
    line_size: f32,
    color: OklabGradient,
}

impl BeatBasedEffect for SegmentedSnake {
    type Color = color::RGB;

    fn init(resolution_hint: Option<u32>, start_beat: i32) -> Self {
        let mut line_size = 0.01;
        if let Some(resolution) = resolution_hint {
            let pixel_size = 1.0 / (resolution as f32);
            if line_size < pixel_size {
                line_size = pixel_size;
            }
        }

        Self {
            cycle: MultiBeatCycle::new(24, start_beat),
            line_count: 18,
            line_size,
            color: OklabGradient::new(color::rgb8(0, 0, 255), color::rgb8(255, 0, 0)),
        }
    }

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::RGB>,
        _d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        self.cycle.update(beat);

        let progress = self.cycle.cycle_progress();

        let two_pi = 2.0 * PI;

        if self.cycle.cycle_number() >= 0 {
            for line_id in (0..self.line_count).rev() {
                let offset = f32::from(line_id) / (2.0 * f32::from(self.line_count));
                if self.cycle.cycle_number() == 0 && progress < offset {
                    continue;
                }

                let pos = (progress - offset).rem_euclid(1.0);
                let pos = (pos * two_pi).cos() * 0.5 + 0.5;

                let color = self
                    .color
                    .interpolate((f32::from(line_id) + 1.0) / f32::from(self.line_count));
                let start = pos * (1.0 - self.line_size);
                let end = start + self.line_size;
                framebuffer.draw_smooth(start, end, color, BlendMode::None);
            }
        }

        Ok(EffectState {
            idle: self.cycle.cycle_number() >= 2,
        })
    }
}
