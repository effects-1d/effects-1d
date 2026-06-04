#[allow(unused_imports)]
use effects_1d_common::prelude::*;

use effects_1d_common::{
    color,
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

fn brightness(v: f32) -> color::Monochrome {
    color::Monochrome::new((f32::from(u16::MAX) * v.clamp(0.0, 1.0)) as u16)
}

fn draw_wrapped(
    framebuffer: &mut dyn FrameBufferRef<color::Monochrome>,
    center: f32,
    width: f32,
    color: color::Monochrome,
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
        let mut width: f32 = 0.025;
        if let Some(resolution) = resolution_hint {
            width = width.max(1.5 / resolution as f32);
        }
        Self {
            cycle: MultiBeatCycle::new(8, start_beat),
            width,
            tail_length: 0.28,
        }
    }
}

impl BeatBasedEffect for CometTrail {
    type Color = color::Monochrome;

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<Self::Color>,
        _d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        self.cycle.update(beat);
        let head = self.cycle.cycle_progress();
        const TRAIL_SEGMENTS: u16 = 12;
        for segment in 0..TRAIL_SEGMENTS {
            let rel = f32::from(segment) / f32::from(TRAIL_SEGMENTS);
            let pos = (head - rel * self.tail_length).rem_euclid(1.0);
            let intensity = (1.0 - rel).powf(2.0);
            draw_wrapped(framebuffer, pos, self.width, brightness(intensity));
        }

        Ok(EffectState {
            idle: self.cycle.is_last_beat_of_cycle(),
        })
    }
}
