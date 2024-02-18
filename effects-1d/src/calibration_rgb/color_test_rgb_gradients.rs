#[allow(unused_imports)]
use effects_1d_common::prelude::*;

use effects_1d_common::{
    color::{self, gradients::HslGradient, RGB},
    effects::{ConstructibleTimeBasedEffect, EffectState, FrameBufferRef, TimeBasedEffect},
    errors::RenderError,
};

#[derive(Debug)]
pub struct ColorTestRgbGradients;

type Gradient = HslGradient;

impl ConstructibleTimeBasedEffect for ColorTestRgbGradients {
    fn init(_resolution_hint: Option<u32>) -> Self {
        Self
    }
}

impl TimeBasedEffect for ColorTestRgbGradients {
    type Color = color::RGB;

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::RGB>,
        _d_t: f32,
    ) -> Result<EffectState, RenderError> {
        let gradient0 = Gradient::new(RGB::new(u16::MAX, 0, 0), RGB::new(0, 0, u16::MAX));
        let gradient1 = Gradient::new(RGB::new(u16::MAX, 0, 0), RGB::new(0, u16::MAX, 0));
        let gradient2 = Gradient::new(RGB::new(0, 0, 0), RGB::new(u16::MAX, u16::MAX, u16::MAX));

        framebuffer.draw_gradient(0. / 3., 1. / 3., &gradient0);
        framebuffer.draw_gradient(1. / 3., 2. / 3., &gradient1);
        framebuffer.draw_gradient(2. / 3., 3. / 3., &gradient2);

        Ok(EffectState { idle: false })
    }
}
