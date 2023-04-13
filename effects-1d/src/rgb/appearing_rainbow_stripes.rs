use effects_1d_common::{
    color,
    effects::{BeatBasedEffect, BeatInfo, BlendMode, EffectState, FrameBufferRef},
    errors::RenderError,
};

#[derive(Debug)]
pub struct AppearingRainbowStripes {
    num_stripes: u32,
    gap_size: f32,
    start_beat: i32,
}

impl BeatBasedEffect for AppearingRainbowStripes {
    type Color = color::Oklab;

    fn init(_resolution_hint: Option<u32>, start_beat: i32) -> Self {
        Self {
            num_stripes: 12,
            gap_size: 0.01,
            start_beat,
        }
    }

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<color::Oklab>,
        _d_t: f32,
        mut beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        let diff_beats = beat.current - self.start_beat;

        Ok(EffectState { idle: false })
    }
}
