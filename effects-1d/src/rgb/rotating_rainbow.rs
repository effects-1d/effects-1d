use effects_1d_common::{
    color::{self, HsvRainbowGradient},
    effects::{BeatBasedEffect, BeatInfo, BlendMode, EffectState, FrameBufferRef},
    errors::RenderError,
};

#[derive(Debug)]
pub struct RotatingRainbow {
    cycle_len: u8,
    rainbow_scale: f32,
}

impl BeatBasedEffect for RotatingRainbow {
    type Color = color::RGB;

    fn init(_resolution_hint: Option<u32>, _start_beat: i32) -> Self {
        Self {
            cycle_len: 12,
            rainbow_scale: 0.5,
        }
    }

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::RGB>,
        _d_t: f32,
        mut beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        beat.current %= i32::from(self.cycle_len);

        let cycle_pos = (beat - BeatInfo::zero()) / f32::from(self.cycle_len);

        framebuffer.draw_gradient(
            0.0,
            1.0,
            &HsvRainbowGradient {
                saturation: 1.0,
                brightness: 1.0,
                offset: cycle_pos * 360.0,
                scale: self.rainbow_scale,
                reversed: false,
            },
        );

        Ok(EffectState { idle: false })
    }
}
