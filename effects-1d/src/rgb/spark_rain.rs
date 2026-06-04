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
    rhythm::BeatMultiplier,
};

#[derive(Clone, Copy, Debug)]
struct Spark {
    start_pos: f32,
    velocity: f32,
    start_beat: i32,
    start_fractional: f32,
    lifetime: f32,
    width: f32,
    color: color::RGB,
}

fn draw_wrapped(
    framebuffer: &mut dyn FrameBufferRef<color::RGB>,
    center: f32,
    width: f32,
    color: color::RGB,
) {
    let start = center - width / 2.0;
    let end = center + width / 2.0;
    if start < 0.0 {
        framebuffer.draw_smooth(start + 1.0, 1.0, color, BlendMode::Add);
        framebuffer.draw_smooth(0.0, end, color, BlendMode::Add);
    } else if end > 1.0 {
        framebuffer.draw_smooth(start, 1.0, color, BlendMode::Add);
        framebuffer.draw_smooth(0.0, end - 1.0, color, BlendMode::Add);
    } else {
        framebuffer.draw_smooth(start, end, color, BlendMode::Add);
    }
}

impl Spark {
    fn new(beat: BeatInfo, rng: &mut EffectRng) -> Self {
        Self {
            start_pos: rng.random_range(-0.08..0.12),
            velocity: rng.random_range(0.10..0.28),
            start_beat: beat.current,
            start_fractional: beat.fractional,
            lifetime: rng.random_range(3.0..8.0),
            width: rng.random_range(0.008..0.03),
            color: color::hsv8(rng.random::<f32>() * 360.0, 0.75, 1.0),
        }
    }

    fn render(
        self,
        beat: BeatInfo,
        framebuffer: &mut dyn FrameBufferRef<color::RGB>,
    ) -> Option<Self> {
        let age = (beat.current - self.start_beat) as f32 + beat.fractional - self.start_fractional;
        let progress = age / self.lifetime;
        if !(0.0..=1.0).contains(&progress) {
            return None;
        }

        let head = self.start_pos + self.velocity * age;
        if head > 1.12 {
            return None;
        }

        let brightness = (1.0 - progress).powf(2.0);
        for segment in 0..6u16 {
            let rel = f32::from(segment) / 6.0;
            let c = color::RGB::zero().elementwise_lerp(self.color, brightness * (1.0 - rel));
            draw_wrapped(framebuffer, head - rel * 0.08, self.width, c);
        }
        Some(self)
    }
}

#[derive(Debug)]
pub struct SparkRain {
    rng: EffectRng,
    sparks: [Option<Spark>; 16],
    multiplier: BeatMultiplier,
}

impl ConstructibleBeatBasedEffect for SparkRain {
    fn init(_resolution_hint: Option<u32>, start_beat: i32) -> Self {
        Self {
            rng: EffectRng::new(),
            sparks: core::array::from_fn(|_| None),
            multiplier: BeatMultiplier::new(4, start_beat),
        }
    }
}

impl BeatBasedEffect for SparkRain {
    type Color = color::RGB;

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<Self::Color>,
        _d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        let beat = self.multiplier.generate_beat(beat);
        for spark in &mut self.sparks {
            *spark = spark
                .take()
                .and_then(|spark| spark.render(beat, framebuffer));
        }
        if beat.is_new_beat && self.rng.random::<u8>() % 3 != 0 {
            for spark in &mut self.sparks {
                if spark.is_none() {
                    *spark = Some(Spark::new(beat, &mut self.rng));
                    break;
                }
            }
        }
        Ok(EffectState { idle: true })
    }
}
