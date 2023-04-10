use crate::errors::RenderError;

use super::{EffectState, MeasureInfo};

/// An effect whos animation is purely time-based.
pub trait MeasureBasedEffect {
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
        measure: MeasureInfo,
    ) -> Result<EffectState, RenderError>;
}
