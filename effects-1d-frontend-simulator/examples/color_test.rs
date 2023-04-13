use effects_1d_common::{
    color::{
        self,
        palette::{FromColor, Mix},
        rgb, InterpolatableColor,
    },
    effects::{EffectState, FrameBufferRef, TimeBasedEffect},
    errors::RenderError,
};
use effects_1d_frontend_simulator::SimulateEffect;

#[derive(Debug)]
struct ColorTest;

impl TimeBasedEffect for ColorTest {
    type Color = color::Oklab;

    fn init(_resolution_hint: Option<u32>) -> Self {
        Self
    }

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::Oklab>,
        _d_t: f32,
    ) -> Result<EffectState, RenderError> {
        let len = framebuffer.len() as f32;
        for pos in 0..framebuffer.len() {
            let pos_f = pos as f32;
            let percent = pos_f / len;
            if percent < 1.0 / 3.0 {
                framebuffer.set_pixel(
                    pos,
                    rgb(1., 0., 0.).interpolate(rgb(0., 0., 1.), 3. * percent),
                )
            } else if percent < 2.0 / 3.0 {
                framebuffer.set_pixel(
                    pos,
                    rgb(1., 0., 0.).interpolate(rgb(0., 1., 0.), 3. * percent - 1.),
                )
            } else {
                framebuffer.set_pixel(
                    pos,
                    rgb(0., 0., 0.).interpolate(rgb(1., 1., 1.), 3. * percent - 2.),
                )
            }
            // framebuffer.set_pixel(
            //     pos,
            //     color::Oklab::from_color(color::palette::Srgb::new(1., 0., 0.)).mix(
            //         color::Oklab::from_color(color::palette::Srgb::new(0., 0., 1.)),
            //         percent,
            //     ),
            // )
        }
        Ok(EffectState { idle: false })
    }
}

fn main() {
    ColorTest::simulate();
}
