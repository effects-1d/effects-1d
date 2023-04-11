mod time_based_effect;
pub use time_based_effect::TimeBasedEffect;
mod beat_based_effect;
pub use beat_based_effect::BeatBasedEffect;
mod measure_based_effect;
pub use measure_based_effect::MeasureBasedEffect;

mod framebuffer;
pub use framebuffer::{FrameBuffer, FrameBufferRef};

/// The current state of an effect.
pub struct EffectState {
    /// The effect is finished and a new effect
    /// needs to be played immediately
    pub over: bool,

    /// The effect can be scheduled out if desired.
    ///
    /// Meant for effects that don't have a fix end;
    /// whether or not they get scheduled out is a decision
    /// of the runtime system that the effect can't influence.
    pub idle: bool,
    // TODO: Add transition hints as soon as we implemented transitions
}

/// The progress of the current beat.
///
/// Note that this is not coupled to the current time in any way.
/// The beat tempo could slow down or speed up spontaneously.
pub struct BeatInfo {
    /// The number of the current beat.
    pub current: i32,

    /// The position within the current beat.
    ///
    /// This is always a value between 0.0 and 1.0.
    /// Whenever it crosses 1.0, it will restart at 0.0
    /// and the `current` variable gets increased by one.
    pub fractional: f32,
}

/// The progress of the current measure.
///
/// Note that this is not coupled to the current time in any way.
/// The beat tempo could slow down or speed up spontaneously.
pub struct MeasureInfo {
    /// Information about the current beat.
    pub beat: BeatInfo,
    /// The beat where the current measure started.
    pub current_measure_start: i32,
    /// The beat where the next measure will start.
    pub next_measure_start: i32,
}
