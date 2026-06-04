#[allow(unused_imports)]
use effects_1d_common::prelude::*;

use effects_1d_common::{
    color,
    effects::{
        BeatBasedEffect, BeatInfo, ConstructibleBeatBasedEffect, EffectState, FrameBufferRef,
    },
    errors::RenderError,
    random::{EffectRng, RngExt},
};

#[derive(Default, Debug)]
struct Stripe {
    pos: f32,
    color: color::BinaryRGB,
}

#[derive(Debug)]
pub struct JumpingStripes {
    stripe_size: f32,
    stripes: [Stripe; 2],
    rng: EffectRng,
}

impl JumpingStripes {
    fn regenerate_stripes(&mut self) {
        const POSSIBLE_COLORS: &[color::BinaryRGB; 6] = color::BinaryRGB::all_possible_colors();
        const NUM_COLORS: usize = POSSIBLE_COLORS.len();

        let colorid_0 = self.rng.random_range(0..NUM_COLORS);
        let mut colorid_1 = self.rng.random_range(0..(NUM_COLORS - 1));
        if colorid_1 >= colorid_0 {
            colorid_1 += 1;
        }

        let position_0 = self.rng.random_range(0.0..(1.0 - self.stripe_size));
        let mut position_1 = self.rng.random_range(0.0..(1.0 - 3.0 * self.stripe_size));
        if position_1 > position_0 - self.stripe_size {
            position_1 += 2.0 * self.stripe_size;
        }

        self.stripes[0].color = POSSIBLE_COLORS[colorid_0];
        self.stripes[1].color = POSSIBLE_COLORS[colorid_1];
        self.stripes[0].pos = position_0;
        self.stripes[1].pos = position_1;
    }
}

impl ConstructibleBeatBasedEffect for JumpingStripes {
    fn init(_resolution_hint: Option<u32>, _start_beat: i32) -> Self {
        let mut this = Self {
            stripe_size: 0.05,
            stripes: Default::default(),
            rng: EffectRng::new(),
        };
        this.regenerate_stripes();
        this
    }
}

impl BeatBasedEffect for JumpingStripes {
    type Color = color::BinaryRGB;

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::BinaryRGB>,
        _d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        if beat.is_new_beat {
            self.regenerate_stripes();
            framebuffer.draw_sharp(0.0, 1.0, color::BinaryRGB::white());
        }

        for stripe in &self.stripes {
            framebuffer.draw_sharp(
                stripe.pos - self.stripe_size / 2.0,
                stripe.pos + self.stripe_size / 2.0,
                stripe.color,
            )
        }

        Ok(EffectState { idle: true })
    }
}
