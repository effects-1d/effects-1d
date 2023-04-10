use crate::errors::RenderError;

use super::{BeatBasedEffect, BeatInfo, EffectState};

/// An effect whos animation is purely time-based.
pub trait TimeBasedEffect {
    /// The color space the effect will render to
    type PixelType;

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
    fn render_frame(
        &mut self,
        framebuffer: &mut [Self::PixelType],
        d_t: f32,
    ) -> Result<EffectState, RenderError>;
}

impl<T> BeatBasedEffect for T
where
    T: TimeBasedEffect,
{
    type PixelType = <Self as TimeBasedEffect>::PixelType;

    fn init() -> Self {
        TimeBasedEffect::init()
    }

    fn render_frame(
        &mut self,
        framebuffer: &mut [Self::PixelType],
        d_t: f32,
        _beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        TimeBasedEffect::render_frame(self, framebuffer, d_t)
    }
}
