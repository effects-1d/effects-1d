mod time_based_effect;
pub use time_based_effect::TimeBasedEffect;
mod beat_based_effect;
pub use beat_based_effect::BeatBasedEffect;
mod measure_based_effect;
pub use measure_based_effect::MeasureBasedEffect;

mod framebuffer;
pub use framebuffer::FrameBufferRef;

/// The current state of an effect.
#[derive(Debug)]
pub struct EffectState {
    /// The effect can be scheduled out if desired.
    ///
    /// Meant for effects that don't have a fix end;
    /// whether or not they get scheduled out is a decision
    /// of the runtime system that the effect can't influence.
    ///
    /// If the effect gets scheduled out, this will happen at a beat change.
    ///
    /// Effects that can only be scheduled out at specific times
    /// (for example because they loop and it would look bad to stop them in the middle
    /// of the loop) should set this flag to `true` during the entire
    /// last beat in the loop.
    pub idle: bool,
    // TODO: Add transition hints as soon as we implemented transitions
}

/// The progress of the current beat.
///
/// Note that this is not coupled to the current time in any way.
/// The beat tempo could slow down or speed up spontaneously.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct BeatInfo {
    /// The number of the current beat.
    pub current: i32,

    /// The position within the current beat.
    ///
    /// This is always a value between 0.0 and 1.0.
    /// Whenever it crosses 1.0, it will restart at 0.0
    /// and the `current` variable gets increased by one.
    pub fractional: f32,

    /// Indicates that the `current` beat just changed.
    pub is_new_beat: bool,
}

impl BeatInfo {
    /// Creates a new BeatInfo object.
    ///
    /// # Arguments
    ///
    /// * `current` - The integer part of the current beat.
    /// * `fractional` - The fractional part of the current beat.
    pub const fn new(current: i32, fractional: f32) -> Self {
        Self {
            current,
            fractional,
            is_new_beat: false,
        }
    }

    /// Creates a beat info whose current beat is at 0.0
    pub const fn zero() -> Self {
        Self {
            current: 0,
            fractional: 0.0,
            is_new_beat: true,
        }
    }

    /// Progresses the beat by a specific amount.
    pub fn progress(&mut self, delta: f32) {
        assert!(delta >= 0.0);
        self.fractional += delta;
        let passed_full_beats = self.fractional as i32;
        self.fractional -= passed_full_beats as f32;
        self.current += passed_full_beats;
        self.is_new_beat = passed_full_beats != 0;
    }

    /// Returns the next full beat relative to this one.
    pub fn next_full_beat(&self) -> BeatInfo {
        BeatInfo {
            current: if self.is_new_beat {
                self.current
            } else {
                self.current + 1
            },
            fractional: 0.0,
            is_new_beat: true,
        }
    }
}

impl PartialOrd for BeatInfo {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        match self.current.partial_cmp(&other.current) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.fractional.partial_cmp(&other.fractional)
    }
}

impl core::ops::Sub<BeatInfo> for BeatInfo {
    type Output = f32;

    fn sub(self, rhs: BeatInfo) -> Self::Output {
        let diff_current = self.current - rhs.current;
        diff_current as f32 + self.fractional - rhs.fractional
    }
}

/// The progress of the current measure.
///
/// Note that this is not coupled to the current time in any way.
/// The beat tempo could slow down or speed up spontaneously.
#[derive(Clone, Debug)]
pub struct MeasureInfo {
    /// Information about the current beat.
    pub beat: BeatInfo,
    /// The beat where the current measure started.
    pub current_measure_start: i32,
    /// The beat where the next measure will start.
    pub next_measure_start: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate std;

    #[test]
    fn beatinfo_sub() {
        let diff = BeatInfo::new(5, 0.3) - BeatInfo::new(3, 0.9);
        assert!((diff - 1.4).abs() < 10.0 * f32::EPSILON);
        let diff = BeatInfo::new(3, 0.9) - BeatInfo::new(5, 0.3);
        assert!((diff + 1.4).abs() < 10.0 * f32::EPSILON);
    }
}

/// Different blend modes used for smooth rendering
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum BlendMode {
    /// Additive blend mode
    ///
    /// New color gets added to the existing pixel color.
    Add,

    /// Maximum blend mode
    ///
    /// The pixel becomes the maximum of its existing color and the new color.
    Max,
}
