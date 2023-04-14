use effects_1d_common::{
    color::{self, ColorGradient, HslGradient, RGB},
    effects::{EffectState, FrameBufferRef, TimeBasedEffect},
    errors::RenderError,
};
use effects_1d_frontend_simulator::SimulateEffect;

#[derive(Debug)]
struct ColorTest;

type Gradient = HslGradient;

impl TimeBasedEffect for ColorTest {
    type Color = color::RGB;

    fn init(_resolution_hint: Option<u32>) -> Self {
        Self
    }

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::RGB>,
        _d_t: f32,
    ) -> Result<EffectState, RenderError> {
        let len = framebuffer.len() as f32;

        let gradient0 = Gradient::new(RGB::new(u16::MAX, 0, 0), RGB::new(0, 0, u16::MAX));
        let gradient1 = Gradient::new(RGB::new(u16::MAX, 0, 0), RGB::new(0, u16::MAX, 0));
        let gradient2 = Gradient::new(RGB::new(0, 0, 0), RGB::new(u16::MAX, u16::MAX, u16::MAX));

        for pos in 0..framebuffer.len() {
            let pos_f = pos as f32;
            let percent = 3. * pos_f / len;
            if percent < 1. {
                framebuffer.set_pixel(pos, gradient0.interpolate(percent))
            } else if percent < 2. {
                framebuffer.set_pixel(pos, gradient1.interpolate(percent - 1.))
            } else {
                framebuffer.set_pixel(pos, gradient2.interpolate(percent - 2.))
            }
        }
        Ok(EffectState { idle: false })
    }
}

fn main() {
    ColorTest::simulate();
}
