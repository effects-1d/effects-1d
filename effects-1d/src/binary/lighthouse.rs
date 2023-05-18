#[allow(unused_imports)]
use effects_1d_common::prelude::*;

use effects_1d_common::{
    color,
    effects::{BeatBasedEffect, BeatInfo, EffectState, FrameBufferRef},
    errors::RenderError,
};

use core::f32::consts::PI;

#[derive(Debug)]
pub struct Lighthouse {
    stripe_width: f32,
    cycle_length: u16,
    start_beat: i32,
}

impl BeatBasedEffect for Lighthouse {
    type Color = color::Binary;

    fn init(resolution_hint: Option<u32>, start_beat: i32) -> Self {
        let mut this = Self {
            stripe_width: 0.03,
            cycle_length: 8,
            start_beat,
        };

        if let Some(resolution) = resolution_hint {
            let pixel_size = 1.0 / resolution as f32;
            if this.stripe_width < pixel_size {
                this.stripe_width = pixel_size;
            }
        }

        this
    }

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::Binary>,
        _d_t: f32,
        mut beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        beat.current -= self.start_beat;
        beat.current = beat.current.rem_euclid(i32::from(self.cycle_length));
        let cycle_pos = (beat - BeatInfo::zero()) / f32::from(self.cycle_length);

        let stripe_pos = (0.5 - 0.5 * (cycle_pos * 2. * PI).cos()) * (1. - self.stripe_width);

        framebuffer.draw_sharp(
            stripe_pos,
            stripe_pos + self.stripe_width,
            color::Binary::on(),
        );

        Ok(EffectState {
            idle: beat.current + 1 == i32::from(self.cycle_length),
        })
    }
}
