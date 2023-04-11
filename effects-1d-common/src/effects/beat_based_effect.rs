use crate::{color::Color, errors::RenderError};

use super::{framebuffer::FrameBufferRef, BeatInfo, EffectState, MeasureBasedEffect, MeasureInfo};

/// An effect who's animation is purely time-based.
pub trait BeatBasedEffect {
    /// The color space the effect will render to
    type Color: Color;

    /// Creates a new instance of the effect
    fn init() -> Self;

    /// Renders the current frame.
    ///
    /// It is expected that if this function returns `Ok()`, then the `framebuffer` is fulled with meaningful data.
    /// The data is already initialized to black, so black pixels don't have to be set.
    ///
    /// # Arguments
    ///
    /// * `framebuffer` - The framebuffer that should get filled by this function.
    /// * `d_t` - The time difference to the previous frame, in seconds.
    /// * `beat` - Information about the position in the current beat.
    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<Self::Color>,
        d_t: f32,
        beat: BeatInfo,
    ) -> Result<EffectState, RenderError>;
}

impl<T> MeasureBasedEffect for T
where
    T: BeatBasedEffect,
{
    type Color = <Self as BeatBasedEffect>::Color;

    fn init() -> Self {
        BeatBasedEffect::init()
    }

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<Self::Color>,
        d_t: f32,
        measure: MeasureInfo,
    ) -> Result<EffectState, RenderError> {
        BeatBasedEffect::render_frame(self, framebuffer, d_t, measure.beat)
    }
}
