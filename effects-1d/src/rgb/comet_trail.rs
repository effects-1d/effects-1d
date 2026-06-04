#[allow(unused_imports)]
use effects_1d_common::prelude::*;

use effects_1d_common::{
    color::{self, BlendableColor, Color},
    effects::{
        BeatBasedEffect, BeatInfo, BlendMode, ConstructibleBeatBasedEffect, EffectState,
        FrameBufferRef,
    },
    errors::RenderError,
    rhythm::MultiBeatCycle,
};

#[derive(Debug)]
pub struct CometTrail {
    cycle: MultiBeatCycle,
    width: f32,
    tail_length: f32,
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

impl ConstructibleBeatBasedEffect for CometTrail {
    fn init(resolution_hint: Option<u32>, start_beat: i32) -> Self {
        let mut width = 0.022;
        if let Some(resolution) = resolution_hint {
            width = width.max(1.5 / resolution as f32);
        }
        Self {
            cycle: MultiBeatCycle::new(8, start_beat),
            width,
            tail_length: 0.30,
        }
    }
}

impl BeatBasedEffect for CometTrail {
    type Color = color::RGB;

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<Self::Color>,
        _d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        self.cycle.update(beat);
        let head = self.cycle.cycle_progress();
        let base_hue = 360.0 * head;
        const TRAIL_SEGMENTS: u16 = 14;
        for segment in 0..TRAIL_SEGMENTS {
            let rel = f32::from(segment) / f32::from(TRAIL_SEGMENTS);
            let pos = (head - rel * self.tail_length).rem_euclid(1.0);
            let intensity = (1.0 - rel).powf(2.0);
            let c = color::RGB::zero().elementwise_lerp(
                color::hsv8(base_hue + rel * 65.0, 1.0, 1.0),
                intensity,
            );
            draw_wrapped(framebuffer, pos, self.width, c);
        }
        Ok(EffectState {
            idle: self.cycle.is_last_beat_of_cycle(),
        })
    }
}
