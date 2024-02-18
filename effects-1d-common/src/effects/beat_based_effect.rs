use core::fmt::Debug;

use crate::{color::Color, errors::RenderError};

use super::{
    framebuffer::FrameBufferRef, BeatInfo, ConstructibleMeasureBasedEffect, EffectState,
    MeasureBasedEffect, MeasureInfo,
};

/// An effect who can be constructed.
pub trait ConstructibleBeatBasedEffect: BeatBasedEffect {
    /// Creates a new instance of the effect.
    ///
    /// # Arguments
    ///
    /// * `resolution_hint` - A hint about what the resolution later might be.
    /// * `start_beat` - The beat number where the effect should start displaying.
    ///                  It can be assumed that the first time the effect will be rendered
    ///                  is a very short time after the `start_beat` started.
    ///
    /// **IMPORTANT**: The `resolution_hint` is an approximate value and is intended to configure
    /// the effect parameters so that it looks good at the given resolution.
    /// It is **not** meant to be used in anything that requires a precise value, like array initializations.
    /// The actual resolution later might change at any point, although it can be assumed that it will lie
    /// in the general area of the hint. The resolution might even change in every frame, for example
    /// at POV displays (where the frame size might depend on the varying rotation speed).
    fn init(resolution_hint: Option<u32>, start_beat: i32) -> Self;
}

/// An effect who's animation is based on time and beat.
pub trait BeatBasedEffect: Send + Sync + 'static + Debug {
    /// The color space the effect will render to
    type Color: Color;

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

impl<T> ConstructibleMeasureBasedEffect for T
where
    T: ConstructibleBeatBasedEffect,
{
    fn init(resolution_hint: Option<u32>, start_beat: i32) -> Self {
        ConstructibleBeatBasedEffect::init(resolution_hint, start_beat)
    }
}

impl<T> MeasureBasedEffect for T
where
    T: BeatBasedEffect,
{
    type Color = <Self as BeatBasedEffect>::Color;

    fn render_frame(
        &mut self,
        framebuffer: &mut dyn FrameBufferRef<Self::Color>,
        d_t: f32,
        measure: MeasureInfo,
    ) -> Result<EffectState, RenderError> {
        BeatBasedEffect::render_frame(self, framebuffer, d_t, measure.beat)
    }
}
