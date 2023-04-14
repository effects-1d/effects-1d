use effects_1d_common::{
    color::{self, BlendableColor},
    effects::{EffectState, FrameBufferRef, TimeBasedEffect},
    errors::RenderError,
};
use effects_1d_frontend_simulator::SimulateEffect;

#[derive(Debug)]
struct ColorTest;

impl TimeBasedEffect for ColorTest {
    type Color = color::Monochrome;

    fn init(_resolution_hint: Option<u32>) -> Self {
        Self
    }

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::Monochrome>,
        _d_t: f32,
    ) -> Result<EffectState, RenderError> {
        let len = framebuffer.len() as f32;
        for pos in 0..framebuffer.len() {
            let pos_f = pos as f32;
            let percent = pos_f / len;
            framebuffer.set_pixel(pos, color::Monochrome::full().apply_alpha(percent))
        }
        Ok(EffectState { idle: false })
    }
}

fn main() {
    ColorTest::simulate();
}
