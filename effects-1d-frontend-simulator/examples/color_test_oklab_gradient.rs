use effects_1d_common::{
    color::{
        self,
        palette::{self, FromColor},
    },
    effects::{EffectState, FrameBufferRef, TimeBasedEffect},
    errors::RenderError,
};
use effects_1d_frontend_simulator::SimulateEffect;

#[derive(Debug)]
struct ColorTest;

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
        for pos in 0..framebuffer.len() {
            let pos_f = pos as f32;
            let percent = 2. * pos_f / len;
            if percent < 1. {
                framebuffer.set_pixel(
                    pos,
                    palette::Srgb::from_color(palette::Hsv::new(percent * 360., 1.0, 1.0))
                        .into_format(),
                );
            } else {
                let percent = percent - 1.0;
                framebuffer.set_pixel(
                    pos,
                    palette::Srgb::from_color(palette::Hsv::new(percent * 360., 0.5, 1.0))
                        .into_format(),
                );
            }
        }
        Ok(EffectState { idle: false })
    }
}

fn main() {
    ColorTest::simulate();
}
