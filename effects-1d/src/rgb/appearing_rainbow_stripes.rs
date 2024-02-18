#[allow(unused_imports)]
use effects_1d_common::prelude::*;

use effects_1d_common::{
    color::{
        self,
        gradients::{ColorGradient, HsvRainbowGradient},
    },
    effects::{
        BeatBasedEffect, BeatInfo, BlendMode, ConstructibleBeatBasedEffect, EffectState,
        FrameBufferRef,
    },
    errors::RenderError,
};

#[derive(Debug)]
pub struct AppearingRainbowStripes {
    num_stripes_per_side: u16,
    gap_size: f32,
    repetitions: u16,
    start_beat: i32,
}

impl ConstructibleBeatBasedEffect for AppearingRainbowStripes {
    fn init(_resolution_hint: Option<u32>, start_beat: i32) -> Self {
        Self {
            num_stripes_per_side: 12,
            gap_size: 0.01,
            repetitions: 2,
            start_beat,
        }
    }
}

impl BeatBasedEffect for AppearingRainbowStripes {
    type Color = color::RGB;

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::RGB>,
        _d_t: f32,
        mut beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        beat.current -= self.start_beat;

        let repetition_duration = i32::from(self.num_stripes_per_side * 2 + 2);
        let repetition = beat.current / repetition_duration;
        if repetition >= i32::from(self.repetitions) {
            return Err(RenderError::EffectOver);
        }

        beat.current = beat.current.rem_euclid(repetition_duration);

        let num_stripes = self.num_stripes_per_side * 2 - 1;
        let stripe_size =
            (1. - self.gap_size * f32::from(num_stripes - 1)) / f32::from(num_stripes);
        let stripe_stride = stripe_size + self.gap_size;

        let gradient = HsvRainbowGradient {
            saturation: 1.0,
            brightness: 1.0,
            offset: 240.0,
            scale: 2. / 3.,
            reversed: false,
        };

        for i in 0..num_stripes {
            let stripe_id = i32::from(if i < self.num_stripes_per_side {
                i
            } else {
                num_stripes - i - 1
            });

            let i_f = f32::from(i);

            let stripe_color =
                gradient.interpolate((stripe_id as f32) / (self.num_stripes_per_side - 1) as f32);

            let start_of_destructure = i32::from(self.num_stripes_per_side) + 1;

            if stripe_id < beat.current && beat.current < (stripe_id + start_of_destructure) {
                framebuffer.draw_smooth(
                    i_f * stripe_stride,
                    i_f * stripe_stride + stripe_size,
                    stripe_color,
                    BlendMode::None,
                )
            } else if stripe_id == beat.current {
                framebuffer.draw_smooth(
                    i_f * stripe_stride,
                    i_f * stripe_stride + stripe_size,
                    stripe_color,
                    BlendMode::Alpha(beat.fractional),
                )
            } else if beat.current == (stripe_id + start_of_destructure) {
                framebuffer.draw_smooth(
                    i_f * stripe_stride,
                    i_f * stripe_stride + stripe_size,
                    stripe_color,
                    BlendMode::Alpha(1.0 - beat.fractional),
                )
            }
        }

        Ok(EffectState {
            idle: beat.current + 1 == repetition_duration,
        })
    }
}
