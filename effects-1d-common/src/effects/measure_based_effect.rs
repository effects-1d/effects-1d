use crate::errors::RenderError;

use super::{EffectState, MeasureInfo};

/// An effect whos animation is purely time-based.
pub trait MeasureBasedEffect {
    /// Renders the current frame.
    /// TODO: document args
    fn render_frame(
        &mut self,
        framebuffer: &mut [u8],
        d_t: f32,
        measure: MeasureInfo,
    ) -> Result<EffectState, RenderError>;
}
