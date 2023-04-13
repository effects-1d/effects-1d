use core::fmt::Debug;

use crate::{color::Color, errors::RenderError};

use super::{framebuffer::FrameBufferRef, BeatBasedEffect, BeatInfo, EffectState};

/// An effect whos animation is purely time-based.
pub trait TimeBasedEffect: Send + Sync + 'static + Debug {
    /// The color space the effect will render to
    type Color: Color;

    /// Creates a new instance of the effect.
    ///
    /// # Arguments
    ///
    /// * `resolution_hint` - A hint about what the resolution later might be.
    ///
    /// **IMPORTANT**: The `resolution_hint` is an approximate value and is intended to configure
    /// the effect parameters so that it looks good at the given resolution.
    /// It is **not** meant to be used in anything that requires a precise value, like array initializations.
    /// The actual resolution later might change at any point, although it can be assumed that it will lie
    /// in the general area of the hint. The resolution might even change in every frame, for example
    /// at POV displays (where the frame size might depend on the varying rotation speed).
    fn init(resolution_hint: Option<u32>) -> Self;

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
        framebuffer: &mut dyn FrameBufferRef<Self::Color>,
        d_t: f32,
    ) -> Result<EffectState, RenderError>;
}

impl<T> BeatBasedEffect for T
where
    T: TimeBasedEffect,
{
    type Color = <Self as TimeBasedEffect>::Color;

    fn init(resolution_hint: Option<u32>, _start_beat: i32) -> Self {
        TimeBasedEffect::init(resolution_hint)
    }

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<Self::Color>,
        d_t: f32,
        _beat: BeatInfo,
    ) -> Result<EffectState, RenderError> {
        TimeBasedEffect::render_frame(self, framebuffer, d_t)
    }
}
