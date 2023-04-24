use crate::effects::BeatInfo;

use super::BeatMultiplier;

/// A sequencer to simplify beat based cyclic actions.
#[derive(Debug)]
pub struct Sequencer {
    // configuration
    cycle_length: u16,
    skip_unschedule_until_beat: u16,
    possible_unschedule_beats: u32,
    sequences: &'static [u32],

    // state
    beat: BeatInfo,
    beat_multiplier: BeatMultiplier,
}

impl Sequencer {
    /// Creates new sequencer
    pub fn new(
        subbeats: u16,
        cycle_length: u16,
        sequences: &'static [u32],
        possible_unschedule_beats: u32,
        skip_unschedule_until_beat: u16,
        start_beat: i32,
    ) -> Self {
        Self {
            cycle_length,
            skip_unschedule_until_beat,
            possible_unschedule_beats,
            sequences,
            beat: Default::default(),
            beat_multiplier: BeatMultiplier::new(start_beat, subbeats),
        }
    }

    /// Move the sequencer to the next timestep.
    ///
    /// Must be called in every iteration of the effect, otherwise
    /// it might cause weird behavior.
    pub fn update(&mut self, beat: BeatInfo) {
        self.beat = self.beat_multiplier.generate_beat(beat);
    }

    /// The current beat
    pub fn beat(&self) -> BeatInfo {
        self.beat
    }

    /// The current beat number in a cycle
    pub fn current_position_in_cycle(&self) -> u16 {
        self.beat.current.rem_euclid(i32::from(self.cycle_length)) as u16
    }

    /// Extract a bit from a beat
    fn extract_state_from_sequence_data(&self, sequence: u32) -> bool {
        let position = self.current_position_in_cycle();
        assert!(position < 32);

        (sequence >> position) & 1 != 0
    }

    /// Whether or not the effect can be unscheduled in the next beat change
    pub fn ready_for_unscheduling(&self) -> bool {
        if self.beat.current < i32::from(self.skip_unschedule_until_beat) {
            false
        } else {
            self.extract_state_from_sequence_data(self.possible_unschedule_beats)
        }
    }

    /// The current value of a given sequence
    pub fn sequence_value(&self, sequence_id: u16) -> bool {
        self.extract_state_from_sequence_data(self.sequences[usize::from(sequence_id)])
    }
}
