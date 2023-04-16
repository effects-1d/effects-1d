use effects_1d_common::{
    color::{self, gradients::HsvRainbowGradient},
    effects::{EffectState, FrameBufferRef, TimeBasedEffect},
    errors::RenderError,
};
use effects_1d_simulator::SimulateEffect;

#[derive(Debug)]
struct ColorTest {
    time: f32,
}

impl TimeBasedEffect for ColorTest {
    type Color = color::RGB;

    fn init(_resolution_hint: Option<u32>) -> Self {
        Self { time: 0.0 }
    }

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::RGB>,
        d_t: f32,
    ) -> Result<EffectState, RenderError> {
        self.time += d_t;
        if self.time > 2.0 {
            self.time -= 2.0;
        }

        framebuffer.draw_gradient(
            0.0,
            0.5,
            &HsvRainbowGradient {
                saturation: 1.0,
                brightness: 1.0,
                offset: self.time * 180.0,
                scale: 1.0,
                reversed: false,
            },
        );
        framebuffer.draw_gradient(
            0.5,
            1.0,
            &HsvRainbowGradient {
                saturation: 1.0,
                brightness: 1.0,
                offset: self.time * 180.0,
                scale: 2.0,
                reversed: false,
            },
        );

        Ok(EffectState { idle: false })
    }
}

fn main() {
    ColorTest::simulate();
}
