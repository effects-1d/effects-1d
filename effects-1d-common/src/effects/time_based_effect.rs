use crate::errors::RenderError;

use super::EffectState;

/// An effect whos animation is purely time-based.
pub trait TimeBasedEffect {
    /// Renders the current frame.
    ///
    /// It is expected that if this function returns `Ok()`, then the `framebuffer` is fulled with meaningful data.
    /// The data is already initialized to black, so
    fn render_frame(framebuffer: &mut [u8], d_t: f32) -> Result<EffectState, RenderError>;
}
