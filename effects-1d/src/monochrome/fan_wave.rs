use core::f32::consts::PI;

use effects_1d_common::{
    color,
    effects::{BeatBasedEffect, BeatInfo, BlendMode, EffectState, FrameBufferRef},
    errors::RenderError,
    lerp::Lerp,
    rhythm::MultiBeatCycle,
};

#[derive(Debug)]
pub struct FanWave {
    line_count: u16,
    cycle: MultiBeatCycle,
}

impl FanWave {
    fn compute_pattern(&self, position: f32, progress: f32) -> f32 {
        let wave_size = 0.4;
        let wave_position = (-wave_size).lerp(1.0, progress);

        let position = if position < 0.5 {
            0.5 - position
        } else {
            position - 0.5
        } * 2.0;

        let position_in_wave = (position - wave_position) / wave_size;

        let triangle_wave = if position > wave_position && position < wave_position + wave_size {
            let mut value = 2.0 * position_in_wave;
            if value > 1.0 {
                value = 2.0 - value;
            }
            value.clamp(0.0, 1.0)
        } else {
            0.0
        };

        let smooth_wave = (triangle_wave * 0.5 * PI).sin();

        smooth_wave * smooth_wave
    }
}

impl BeatBasedEffect for FanWave {
    type Color = color::Monochrome;

    fn init(_resolution_hint: Option<u32>, start_beat: i32) -> Self {
        Self {
            line_count: 27,
            cycle: MultiBeatCycle::new(8, start_beat),
        }
    }

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::Monochrome>,
        _d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        self.cycle.update(beat);
        let progress = self.cycle.cycle_progress();

        let line_count_f = f32::from(self.line_count);
        let mut render_fan_strip = |line_id: u16| {
            let pos = (f32::from(line_id) + 0.5) / line_count_f;

            let saturation = self.compute_pattern(pos, progress) * 0.6;

            let width_half = 0.5 * saturation / line_count_f;
            framebuffer.draw_smooth(
                pos - width_half,
                pos + width_half,
                color::Monochrome::full(),
                BlendMode::Add,
            );
        };

        for line_id in 0..self.line_count {
            render_fan_strip(line_id);
        }

        Ok(EffectState {
            /* only unschedule at odd beats, fits better to the animation. */
            idle: self.cycle.is_last_beat_of_cycle(),
        })
    }
}
