use effects_1d_common::{
    color,
    effects::{EffectState, FrameBufferRef, TimeBasedEffect},
    errors::RenderError,
    random::{EffectRng, Rng},
};

// Parameters
const NUM_LINES: usize = 16;
const STARTUP_DURATION: f32 = 2.0;
const DECAY_DURATION_MIN: f32 = 1.0;
const DECAY_DURATION_MAX: f32 = 2.0;
const DECAY_SPEED: f32 = 1.0;

#[derive(Debug)]
enum LineState {
    Startup { t: f32 },
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
    fn new(start: f32, end: f32, delay: f32) -> Self {
        Self {
            start,
            end,
            state: LineState::Startup { t: delay },
        }
    }

    fn update(&mut self, d_t: f32, rng: &mut EffectRng) -> bool {
        let mut starting = false;

        let mut leftover_d_t = Some(d_t);
        while let Some(d_t) = leftover_d_t {
            match &mut self.state {
                LineState::Startup { t } => {
                    starting = true;
                    leftover_d_t = progress_t(t, d_t);
                    if leftover_d_t.is_some() {
                        self.state = LineState::Decaying {
                            t: generate_decay_duration(rng),
                            val: 1.0,
                        }
                    }
                }
                LineState::Decaying { t, val } => {
                    leftover_d_t = progress_t(t, d_t);
                    if leftover_d_t.is_some() {
                        self.state = LineState::Decaying {
                            t: generate_decay_duration(rng),
                            val: 1.0,
                        }
                    } else {
                        // TODO update brightness
                        *val = (*val - d_t * DECAY_SPEED).clamp(0.0, 1.0);
                    }
                }
            }
        }

        starting
    }

    fn render(&self, framebuffer: &mut dyn FrameBufferRef<color::Monochrome>) {
        match self.state {
            LineState::Startup { t: _ } => (),
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
}

impl TimeBasedEffect for RandomBlinkingPixels {
    type Color = color::Monochrome;

    fn init(_resolution_hint: Option<u32>) -> Self {
        Self {
            rng: EffectRng::new(),
            lines: core::array::from_fn(|pos| {
                let start = (pos as f32) / (NUM_LINES as f32);
                let end = ((pos + 1) as f32) / (NUM_LINES as f32);
                let delay = start * STARTUP_DURATION;

                Line::new(start, end, delay)
            }),
        }
    }

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::Monochrome>,
        d_t: f32,
    ) -> Result<EffectState, RenderError> {
        let mut starting = false;

        for line in &mut self.lines {
            starting &= line.update(d_t, &mut self.rng);
            line.render(framebuffer);
        }
        // self.cycle.update(beat);

        // let progress = self.cycle.cycle_progress();

        // let coverage = if progress < 0.5 {
        //     progress * 2.0
        // } else {
        //     (1.0 - progress) * 2.0
        // };
        // let draw_offset = (progress * 2.0 - 1.0).clamp(0.0, 1.0);

        // self.lines
        //     .set_line_width(coverage * self.lines.get_line_stride());

        // for line in self.lines.draw_iter(draw_offset) {
        //     framebuffer.draw_smooth(
        //         line.start,
        //         line.end,
        //         color::Monochrome::full(),
        //         BlendMode::Add,
        //     );
        // }

        Ok(EffectState {
            idle: false && !starting,
        })
    }
}
