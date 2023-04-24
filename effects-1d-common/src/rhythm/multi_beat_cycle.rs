use crate::effects::BeatInfo;

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
}
