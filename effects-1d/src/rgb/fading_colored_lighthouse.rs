#[allow(unused_imports)]
use effects_1d_common::prelude::*;

use effects_1d_common::{
    color::{self, BlendableColor, Color},
    effects::{BeatBasedEffect, BeatInfo, BlendMode, EffectState, FrameBufferRef},
    errors::RenderError,
    random::{EffectRng, Rng},
    rhythm::MultiBeatCycle,
};

#[derive(Debug)]
struct Stripe {
    line_width: f32,
    pos_start: f32,
    pos_end: f32,
    color: color::RGB,
    cycle: MultiBeatCycle,
}
impl Stripe {
    pub fn new(offset: u16, line_width: f32, cycle_length: u16, start_beat: i32) -> Self {
        Self {
            line_width,
            cycle: MultiBeatCycle::new(cycle_length, start_beat + i32::from(offset)),
            pos_start: Default::default(),
            pos_end: Default::default(),
            color: Default::default(),
        }
    }

    fn regenerate(&mut self, rng: &mut EffectRng) {
        let range = 1.0 - self.line_width;

        self.pos_start = range * rng.gen::<f32>();
        self.pos_end = range * rng.gen::<f32>();

        self.color = color::hsv8(rng.gen::<f32>() * 360.0, 1.0, 1.0);
    }

    pub fn update(&mut self, beat: BeatInfo, rng: &mut EffectRng) {
        self.cycle.update(beat);
        if self.cycle.is_new_cycle() {
            self.regenerate(rng);
        }
    }

    pub fn render(&self, framebuffer: &mut dyn FrameBufferRef<color::RGB>) {
        if self.cycle.cycle_number() < 0 {
            return;
        }

        let progress = self.cycle.cycle_progress();

        let pos = self.cycle.lerp(self.pos_start, self.pos_end);

        let brightness = if progress < 0.5 {
            2.0 * progress
        } else {
            2.0 * (1.0 - progress)
        };

        let color = color::RGB::zero().elementwise_lerp(self.color, brightness);

        framebuffer.draw_smooth(pos, pos + self.line_width, color, BlendMode::Add);
    }
}

#[derive(Debug)]
pub struct FadingColoredLighthouse {
    rng: EffectRng,
    stripes: [Stripe; 3],
}

impl BeatBasedEffect for FadingColoredLighthouse {
    type Color = color::RGB;

    fn init(_resolution_hint: Option<u32>, start_beat: i32) -> Self {
        let line_width = 0.1;
        let cycle_length = 6;

        let mut this = Self {
            rng: EffectRng::new(),
            stripes: [
                Stripe::new(0, line_width, cycle_length, start_beat),
                Stripe::new(2, line_width, cycle_length, start_beat),
                Stripe::new(4, line_width, cycle_length, start_beat),
            ],
        };

        for stripe in &mut this.stripes {
            stripe.regenerate(&mut this.rng)
        }

        this
    }

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::RGB>,
        _d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        for stripe in &mut self.stripes {
            stripe.update(beat, &mut self.rng);
            stripe.render(framebuffer);
        }

        Ok(EffectState {
            idle: self.stripes[0].cycle.is_last_beat_of_cycle(),
        })
    }
}
