use crate::{effects::BeatInfo, lerp::Lerp};

/// Combines many beats into a multi-beat cycle.
#[derive(Debug)]
pub struct MultiBeatCycle {
    cycle_number: i32,
    cycle_progress: f32,
    is_new_cycle: bool,
    is_last_beat_of_cycle: bool,
    cycle_length: u16,
    start_beat: i32,
}

impl MultiBeatCycle {
    /// Creates a new multi-beat cycle.
    pub fn new(cycle_length: u16, start_beat: i32) -> Self {
        Self {
            cycle_number: 0,
            cycle_progress: 0.0,
            is_new_cycle: true,
            is_last_beat_of_cycle: false,
            cycle_length,
            start_beat,
        }
    }

    /// Move the cycle to the next timestep.
    ///
    /// Must be called in every iteration of the effect, otherwise
    /// it might cause weird behavior.
    pub fn update(&mut self, mut beat: BeatInfo) {
        beat.current -= self.start_beat;

        let previous_cycle_number = self.cycle_number;
        self.cycle_number = beat.current.div_euclid(i32::from(self.cycle_length));

        self.is_new_cycle = (previous_cycle_number != self.cycle_number) && beat.is_new_beat;

        let beat_in_cycle = beat.current.rem_euclid(i32::from(self.cycle_length)) as u16;
        self.cycle_progress =
            (f32::from(beat_in_cycle) + beat.fractional) / f32::from(self.cycle_length);

        self.is_last_beat_of_cycle = (beat_in_cycle + 1) == self.cycle_length;
    }

    /// Get the number of the current cycle
    pub fn cycle_number(&self) -> i32 {
        self.cycle_number
    }

    /// Get the progress of the current cycle.
    ///
    /// Will return a number between 0.0 and 1.0.
    pub fn cycle_progress(&self) -> f32 {
        self.cycle_progress
    }

    /// True if this is the first rendering of a new cycle.
    pub fn is_new_cycle(&self) -> bool {
        self.is_new_cycle
    }

    /// True if we are in the last beat of the cycle.
    ///
    /// Often used for the idle state.
    pub fn is_last_beat_of_cycle(&self) -> bool {
        self.is_last_beat_of_cycle
    }

    /// Interpolates between two values.
    ///
    /// Returns the first value at the beginning of the cycle
    /// and the second value at the end of the cycle.
    pub fn lerp<T>(&self, a: T, b: T) -> T
    where
        T: Lerp,
    {
        a.lerp(b, self.cycle_progress())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    struct BeatTester {
        previous_cycle: i32,
        start_beat: i32,
        cycle_length: u16,
    }

    impl BeatTester {
        pub fn test(&mut self, base: BeatInfo, cycle: &MultiBeatCycle) {
            let f_base = base - BeatInfo::new(self.start_beat, 0.0);
            let f_cycle =
                BeatInfo::new(cycle.cycle_number(), cycle.cycle_progress()) - BeatInfo::zero();

            assert_relative_eq!(
                f_cycle * f32::from(self.cycle_length),
                f_base,
                epsilon = f32::EPSILON * 10.0
            );

            assert!(cycle.cycle_progress() >= 0.0);
            assert!(cycle.cycle_progress() <= 1.0);

            if cycle.cycle_number() != self.previous_cycle {
                assert!(cycle.is_new_cycle());
                assert!(base.is_new_beat);
            } else {
                assert!(!cycle.is_new_cycle());
            }
            if !base.is_new_beat {
                assert!(!cycle.is_new_cycle());
            }

            if (base.current - self.start_beat).rem_euclid(i32::from(self.cycle_length)) + 1
                == i32::from(self.cycle_length)
            {
                assert!(cycle.is_last_beat_of_cycle());
            } else {
                assert!(!cycle.is_last_beat_of_cycle());
            }

            self.previous_cycle = cycle.cycle_number();
        }
    }

    #[test]
    fn multi_beat_cycle() {
        let mut t = BeatTester {
            previous_cycle: -2,
            start_beat: 2,
            cycle_length: 3,
        };

        let mut cycle = MultiBeatCycle::new(3, 2);

        let mut base_beat = BeatInfo::new(-2, 0.5);

        for i in 1..100u16 {
            let diff = f32::from(i) / 4.7;

            base_beat.progress(diff);

            cycle.update(base_beat);
            t.test(base_beat, &cycle);
        }
    }
}
