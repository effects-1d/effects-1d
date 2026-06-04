#[allow(unused_imports)]
use effects_1d_common::prelude::*;

use effects_1d_common::{
    color::{self, BlendableColor, Color},
    effects::{
        BeatBasedEffect, BeatInfo, BlendMode, ConstructibleBeatBasedEffect, EffectState,
        FrameBufferRef,
    },
    errors::RenderError,
    random::{EffectRng, RngExt},
    rhythm::{BeatMultiplier, MultiBeatCycle},
};

#[derive(Debug)]
struct Stripe {
    line_width: f32,
    pos: f32,
    color: color::RGB,
    cycle: MultiBeatCycle,
}
impl Stripe {
    pub fn new(offset: u16, line_width: f32, cycle_length: u16) -> Self {
        Self {
            line_width,
            cycle: MultiBeatCycle::new(cycle_length, i32::from(offset)),
            pos: Default::default(),
            color: Default::default(),
        }
    }

    fn regenerate(&mut self, rng: &mut EffectRng) {
        let range = 1.0 - self.line_width;
        self.pos = range * rng.random::<f32>();
        self.color = color::hsv8(rng.random::<f32>() * 360.0, 1.0, 1.0);
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

        let brightness = {
            let b2 = 1.0 - progress;
            b2 * b2
        };

        let color = color::RGB::zero().elementwise_lerp(self.color, brightness);

        framebuffer.draw_smooth(self.pos, self.pos + self.line_width, color, BlendMode::Add);
    }
}

const NUM_STRIPES: usize = 8;
const CYCLE_LENGTH: u16 = 6;
const BEAT_MULTIPLIER: u16 = 2;

#[derive(Debug)]
pub struct RapidFireStripes {
    rng: EffectRng,
    stripes: [Stripe; NUM_STRIPES],
    beat_multiplier: BeatMultiplier,
}

impl ConstructibleBeatBasedEffect for RapidFireStripes {
    fn init(_resolution_hint: Option<u32>, start_beat: i32) -> Self {
        let line_width = 0.1;

        let stripes = core::array::from_fn(|stripe_counter| {
            Stripe::new(stripe_counter as u16, line_width, CYCLE_LENGTH)
        });

        let mut this = Self {
            rng: EffectRng::new(),
            stripes,
            beat_multiplier: BeatMultiplier::new(BEAT_MULTIPLIER, start_beat),
        };

        for stripe in &mut this.stripes {
            stripe.regenerate(&mut this.rng)
        }

        this
    }
}

impl BeatBasedEffect for RapidFireStripes {
    type Color = color::RGB;

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::RGB>,
        _d_t: f32,
        mut beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        beat = self.beat_multiplier.generate_beat(beat);

        for stripe in &mut self.stripes {
            stripe.update(beat, &mut self.rng);
            stripe.render(framebuffer);
        }

        Ok(EffectState {
            idle: self.stripes[0].cycle.is_last_beat_of_cycle(),
        })
    }
}
