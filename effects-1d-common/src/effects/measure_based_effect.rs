use crate::{color::Color, errors::RenderError};

use super::{framebuffer::FrameBufferRef, EffectState, MeasureInfo};

/// An effect whos animation is purely time-based.
pub trait MeasureBasedEffect {
    /// The color space the effect will render to.
    type Color: Color;

    /// Creates a new instance of the effect.
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
    /// * `measure` - Information about the position in the current beat/measure.
    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<Self::Color>,
        d_t: f32,
        measure: MeasureInfo,
    ) -> Result<EffectState, RenderError>;
}
