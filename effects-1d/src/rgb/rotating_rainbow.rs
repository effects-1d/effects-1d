#[allow(unused_imports)]
use effects_1d_common::prelude::*;

use effects_1d_common::{
    color::{self, gradients::HsvRainbowGradient},
    effects::{
        BeatBasedEffect, BeatInfo, ConstructibleBeatBasedEffect, EffectState, FrameBufferRef,
    },
    errors::RenderError,
};

#[derive(Debug)]
pub struct RotatingRainbow {
    cycle_len: u8,
    rainbow_scale: f32,
}

impl ConstructibleBeatBasedEffect for RotatingRainbow {
    fn init(_resolution_hint: Option<u32>, _start_beat: i32) -> Self {
        Self {
            cycle_len: 12,
            rainbow_scale: 0.5,
        }
    }
}

impl BeatBasedEffect for RotatingRainbow {
    type Color = color::RGB;

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::RGB>,
        _d_t: f32,
        mut beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        beat.current = beat.current.rem_euclid(i32::from(self.cycle_len));

        let cycle_pos = (beat - BeatInfo::zero()) / f32::from(self.cycle_len);

        framebuffer.draw_gradient(
            0.0,
            1.0,
            &HsvRainbowGradient {
                saturation: 1.0,
                brightness: 1.0,
                offset: cycle_pos * 360.0,
                scale: self.rainbow_scale,
                reversed: false,
            },
        );

        Ok(EffectState { idle: true })
    }
}
