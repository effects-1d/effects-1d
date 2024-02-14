use effects_1d_common::{
    color::{self, Color},
    effects::{EffectState, FrameBufferRef, TimeBasedEffect},
    errors::RenderError,
};
use effects_1d_simulator::SimulateEffect;

use rand::Rng;

#[derive(Debug)]
struct DebugEffect {
    old_color: color::RGB,
    new_color: color::RGB,
    percent: f32,
    time_total: f64,
}

fn random_color() -> color::RGB {
    let mut rng = rand::thread_rng();
    color::RGB::new(rng.gen(), rng.gen(), rng.gen())
}

const SPEED: f32 = 0.05;

impl TimeBasedEffect for DebugEffect {
    type Color = color::RGB;

    fn init(_resolution_hint: Option<u32>) -> Self {
        Self {
            old_color: color::RGB::zero(),
            new_color: random_color(),
            percent: 0.,
            time_total: 0.0,
        }
    }

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::RGB>,
        d_t: f32,
    ) -> Result<EffectState, RenderError> {
        self.time_total += f64::from(d_t);

        self.percent += d_t * SPEED;
        if self.percent > 1. {
            self.percent -= 1.;
            self.old_color = self.new_color;
            if self.old_color == color::RGB::zero() {
                self.new_color = random_color();
            } else {
                self.new_color = color::RGB::zero();
            }
        }

        framebuffer.draw_sharp(0.0, self.percent, self.new_color);
        framebuffer.draw_sharp(self.percent, 1.0, self.old_color);

        framebuffer.set_pixel(
            framebuffer.len() / 2 + ((self.time_total as u64) % 10) as u32,
            color::RGB::new(u16::MAX, 0, 0),
        );

        Ok(EffectState { idle: false })
    }
}

fn main() {
    tracing_subscriber::fmt()
        // all spans/events with a level higher than TRACE (e.g, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(tracing::Level::DEBUG)
        // sets this to be the default, global collector for this application.
        .init();

    DebugEffect::simulate();
}
