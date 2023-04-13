use effects_1d_common::{
    color::{self, InterpolatableColor},
    effects::{EffectState, FrameBufferRef, TimeBasedEffect},
    errors::RenderError,
};
use effects_1d_frontend_simulator::SimulateEffect;

#[derive(Debug)]
struct ColorTest;

impl TimeBasedEffect for ColorTest {
    type Color = color::Okhsv;

    fn init(_resolution_hint: Option<u32>) -> Self {
        Self
    }

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::Okhsv>,
        _d_t: f32,
    ) -> Result<EffectState, RenderError> {
        let len = framebuffer.len() as f32;
        for pos in 0..framebuffer.len() {
            let pos_f = pos as f32;
            let percent = pos_f / len;
            if percent < 0.5 {
                framebuffer.set_pixel(
                    pos,
                    color::Okhsv::new(29., 1., 1.)
                        .interpolate(color::Okhsv::new(142., 1., 1.), 2. * percent),
                )
            } else {
                framebuffer.set_pixel(
                    pos,
                    color::Okhsv::new(0., 0., 0.)
                        .interpolate(color::Okhsv::new(0., 0., 1.), 2. * (percent - 0.5)),
                )
            }
        }
        Ok(EffectState { idle: false })
    }
}

fn main() {
    ColorTest::simulate();
}
