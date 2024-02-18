#[allow(unused_imports)]
use effects_1d_common::prelude::*;

use effects_1d_common::{
    color::{self, BlendableColor, Color},
    effects::{ConstructibleTimeBasedEffect, EffectState, FrameBufferRef, TimeBasedEffect},
    errors::RenderError,
};

#[derive(Debug)]
pub struct ColorTestMonochrome;

impl ConstructibleTimeBasedEffect for ColorTestMonochrome {
    fn init(_resolution_hint: Option<u32>) -> Self {
        Self
    }
}

impl TimeBasedEffect for ColorTestMonochrome {
    type Color = color::Monochrome;

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::Monochrome>,
        _d_t: f32,
    ) -> Result<EffectState, RenderError> {
        let len = framebuffer.len() as f32;
        for pos in 0..framebuffer.len() {
            let pos_f = pos as f32;
            let percent = pos_f / len;
            framebuffer.set_pixel(
                pos,
                color::Monochrome::zero().elementwise_lerp(color::Monochrome::full(), percent),
            )
        }
        Ok(EffectState { idle: false })
    }
}
