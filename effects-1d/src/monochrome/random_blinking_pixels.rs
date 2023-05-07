use effects_1d_common::{
    color,
    effects::{EffectState, FrameBufferRef, TimeBasedEffect},
    errors::RenderError,
    random::{EffectRng, Rng},
};

// Parameters
const NUM_LINES: usize = 14;
const DECAY_DURATION_MIN: f32 = 2.0;
const DECAY_DURATION_MAX: f32 = 4.0;
const DECAY_SPEED: f32 = 1.0;
const DECAY_EXPONENTIAL: bool = true;
const DECAY_EXPONENTIAL_FACTOR: f32 = 0.5;

#[derive(Debug)]
enum LineState {
    Decaying { t: f32, val: f32 },
}

#[derive(Debug)]
struct Line {
    start: f32,
    end: f32,
    state: LineState,
}

fn progress_t(t: &mut f32, d_t: f32) -> Option<f32> {
    *t -= d_t;
    if *t < 0.0 {
        Some(-*t)
    } else {
        None
    }
}

fn generate_decay_duration(rng: &mut EffectRng) -> f32 {
    rng.gen_range(DECAY_DURATION_MIN..=DECAY_DURATION_MAX)
}

impl Line {
    fn new(start: f32, end: f32, rng: &mut EffectRng) -> Self {
        Self {
            start,
            end,
            state: LineState::Decaying {
                t: rng.gen_range(0.0..(DECAY_DURATION_MAX - DECAY_DURATION_MIN)),
                val: 0.0,
            },
        }
    }

    fn update(&mut self, d_t: f32, rng: &mut EffectRng) {
        let mut leftover_d_t = Some(d_t);
        while let Some(d_t) = leftover_d_t {
            match &mut self.state {
                LineState::Decaying { t, val } => {
                    leftover_d_t = progress_t(t, d_t);
                    if leftover_d_t.is_some() {
                        self.state = LineState::Decaying {
                            t: generate_decay_duration(rng),
                            val: 1.0,
                        }
                    } else {
                        if DECAY_EXPONENTIAL {
                            *val = *val * DECAY_EXPONENTIAL_FACTOR.powf(d_t * DECAY_SPEED);
                        } else {
                            *val = (*val - d_t * DECAY_SPEED).clamp(0.0, 1.0);
                        }
                    }
                }
            }
        }
    }

    fn render(&self, framebuffer: &mut dyn FrameBufferRef<color::Monochrome>) {
        match self.state {
            LineState::Decaying { t: _, val } => {
                framebuffer.draw_sharp(self.start, self.end, val.into())
            }
        }
    }
}

#[derive(Debug)]
pub struct RandomBlinkingPixels {
    rng: EffectRng,
    lines: [Line; NUM_LINES],
    startup: f32,
}

impl TimeBasedEffect for RandomBlinkingPixels {
    type Color = color::Monochrome;

    fn init(_resolution_hint: Option<u32>) -> Self {
        let mut rng = EffectRng::new();
        let lines = core::array::from_fn(|pos| {
            let start = (pos as f32) / (NUM_LINES as f32);
            let end = ((pos + 1) as f32) / (NUM_LINES as f32);

            Line::new(start, end, &mut rng)
        });
        Self {
            rng,
            lines,
            startup: 2.0 * DECAY_DURATION_MAX,
        }
    }

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::Monochrome>,
        d_t: f32,
    ) -> Result<EffectState, RenderError> {
        self.startup -= d_t;

        for line in &mut self.lines {
            line.update(d_t, &mut self.rng);
            line.render(framebuffer);
        }

        Ok(EffectState {
            idle: self.startup <= 0.0,
        })
    }
}
