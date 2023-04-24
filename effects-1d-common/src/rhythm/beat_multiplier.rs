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
    pub fn new(multiplication_factor: u16, start_beat: i32) -> Self {
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
        extern crate std;
        std::println!(
            "{:?} {:?} {:?}",
            base_beat.current,
            self.start_beat,
            self.multiplication_factor
        );
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

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    struct BeatTester {
        previous_beat: i32,
        start_beat: i32,
        multiplication_factor: u16,
    }

    impl BeatTester {
        pub fn test(&mut self, slow: BeatInfo, fast: BeatInfo) {
            let f_slow = slow - BeatInfo::zero();
            let f_fast = fast - BeatInfo::zero();

            assert_relative_eq!(
                (f_slow - self.start_beat as f32) * f32::from(self.multiplication_factor),
                f_fast,
                epsilon = f32::EPSILON * 10.0
            );

            assert!(fast.fractional >= 0.0);
            assert!(fast.fractional <= 1.0);

            if fast.current != self.previous_beat {
                assert!(fast.is_new_beat);
            } else {
                assert!(!fast.is_new_beat);
                assert!(!slow.is_new_beat);
            }
            if slow.is_new_beat {
                assert!(fast.is_new_beat);
            }

            self.previous_beat = fast.current;
        }
    }

    #[test]
    fn beat_multiplier() {
        let mut t = BeatTester {
            previous_beat: -2,
            start_beat: 3,
            multiplication_factor: 4,
        };

        let mut multiplier = BeatMultiplier::new(4, 3);

        let mut slow_beat = BeatInfo::new(-2, 0.5);

        for i in 0..100u16 {
            let diff = f32::from(i) / 14.7;

            slow_beat.progress(diff);

            let fast_beat = multiplier.generate_beat(slow_beat);
            t.test(slow_beat, fast_beat);
        }
    }
}
