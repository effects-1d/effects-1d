use crate::effects::BeatInfo;

/// Generates a BeatInfo with twice the frequency as a given BeatInfo
#[derive(Debug)]
pub struct BeatMultiplier {
    multiplication_factor: u16,
    start_beat: i32,
    last_fractional_full_part: i32,
}

impl BeatMultiplier {
    /// Creates a new beatdoubler.
    pub fn new(start_beat: i32, multiplication_factor: u16) -> Self {
        Self {
            start_beat,
            last_fractional_full_part: 0,
            multiplication_factor,
        }
    }

    /// Generates the new, faster beat from the current, slow beat.
    ///
    /// Needs to be called in every update.
    pub fn generate_beat(&mut self, base_beat: BeatInfo) -> BeatInfo {
        let mut full_beat_part =
            (base_beat.current - self.start_beat) * i32::from(self.multiplication_factor);

        let fractional_part = base_beat.fractional * f32::from(self.multiplication_factor);
        let fractional_full_part = fractional_part as i32;
        let fractional_frac_part = fractional_part - fractional_full_part as f32;

        full_beat_part += fractional_full_part;

        let is_new_beat =
            base_beat.is_new_beat || (self.last_fractional_full_part != fractional_full_part);

        self.last_fractional_full_part = fractional_full_part;

        BeatInfo {
            current: full_beat_part,
            fractional: fractional_frac_part,
            is_new_beat,
        }
    }
}
