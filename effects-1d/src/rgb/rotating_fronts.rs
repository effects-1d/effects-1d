use effects_1d_common::{
    color::{self, gradients::HslGradient, RGB},
    effects::{BeatBasedEffect, BeatInfo, EffectState, FrameBufferRef},
    errors::RenderError,
};

#[derive(Debug)]
pub struct RotatingFronts {
    color_1: RGB,
    color_2: RGB,
    num_segments: u16,
    cycle_length: u16,
}

impl BeatBasedEffect for RotatingFronts {
    type Color = color::RGB;

    fn init(_resolution_hint: Option<u32>, _start_beat: i32) -> Self {
        Self {
            color_1: RGB::new(0, 0, u16::MAX),
            color_2: RGB::new(0, u16::MAX, u16::MAX),
            num_segments: 10,
            cycle_length: 2,
        }
    }

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::RGB>,
        _d_t: f32,
        mut beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        beat.current %= i32::from(self.cycle_length);

        let cycle_pos = (beat - BeatInfo::zero()) / f32::from(self.cycle_length);

        let gradient = HslGradient::new(self.color_1, self.color_2);

        let segment_size = 1.0 / f32::from(self.num_segments);

        for segment_id in 0..=self.num_segments {
            let pos = (f32::from(segment_id) - 1.0 + cycle_pos) * segment_size;
            framebuffer.draw_gradient(pos, pos + segment_size, &gradient);
        }

        Ok(EffectState { idle: true })
    }
}
