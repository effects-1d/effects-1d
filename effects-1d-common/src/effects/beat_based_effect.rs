use crate::errors::RenderError;

use super::{BeatInfo, EffectState, MeasureBasedEffect, MeasureInfo};

/// An effect who's animation is purely time-based.
pub trait BeatBasedEffect {
    /// Renders the current frame.
    /// TODO: document args
    fn render_frame(
        &mut self,
        framebuffer: &mut [u8],
        d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError>;
}

impl<T> MeasureBasedEffect for T
where
    T: BeatBasedEffect,
{
    fn render_frame(
        &mut self,
        framebuffer: &mut [u8],
        d_t: f32,
        measure: MeasureInfo,
    ) -> Result<EffectState, RenderError> {
        BeatBasedEffect::render_frame(self, framebuffer, d_t, measure.beat)
    }
}
