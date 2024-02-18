#[allow(unused_imports)]
use effects_1d_common::prelude::*;

use effects_1d_common::{
    color,
    effects::{
        BeatBasedEffect, BeatInfo, ConstructibleBeatBasedEffect, EffectState, FrameBufferRef,
    },
    errors::RenderError,
    random::{EffectRng, Rng},
    rhythm::BeatMultiplier,
};

#[derive(Debug)]
pub struct JumpingLine {
    line_width: f32,
    beat_multiplier: BeatMultiplier,
    rng: EffectRng,
    line_position: f32,
}

impl JumpingLine {
    fn refresh(&mut self) {
        self.line_position = (1.0 - self.line_width) * self.rng.gen::<f32>();
    }
}

impl ConstructibleBeatBasedEffect for JumpingLine {
    fn init(_resolution_hint: Option<u32>, start_beat: i32) -> Self {
        let mut this = Self {
            line_width: 0.1,
            beat_multiplier: BeatMultiplier::new(3, start_beat),
            rng: EffectRng::new(),
            line_position: 0.0,
        };
        this.refresh();
        this
    }
}

impl BeatBasedEffect for JumpingLine {
    type Color = color::Binary;

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::Binary>,
        _d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        let beat = self.beat_multiplier.generate_beat(beat);

        if beat.is_new_beat {
            self.refresh();
        }

        framebuffer.draw_sharp(
            self.line_position,
            self.line_position + self.line_width,
            color::Binary::on(),
        );

        Ok(EffectState { idle: true })
    }
}
