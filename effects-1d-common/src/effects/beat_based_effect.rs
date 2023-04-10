use crate::errors::RenderError;

use super::{BeatInfo, EffectState, MeasureBasedEffect, MeasureInfo};

/// An effect who's animation is purely time-based.
pub trait BeatBasedEffect {
    /// The color space the effect will render to
    type PixelType;

    /// Creates a new instance of the effect
    fn init() -> Self;

    /// Renders the current frame.
    /// TODO: document args
    fn render_frame(
        &mut self,
        framebuffer: &mut [Self::PixelType],
        d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError>;
}

impl<T> MeasureBasedEffect for T
where
    T: BeatBasedEffect,
{
    type PixelType = <Self as BeatBasedEffect>::PixelType;

    fn init() -> Self {
        BeatBasedEffect::init()
    }

    fn render_frame(
        &mut self,
        framebuffer: &mut [Self::PixelType],
        d_t: f32,
        measure: MeasureInfo,
    ) -> Result<EffectState, RenderError> {
        BeatBasedEffect::render_frame(self, framebuffer, d_t, measure.beat)
    }
}
