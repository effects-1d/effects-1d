#[allow(unused_imports)]
use effects_1d_common::prelude::*;
use effects_1d_common::random::{EffectRng, Rng};

/// A timer that determines whether or not an effect
/// should be changed.
pub struct SwitchTimer {
    mean_effect_duration_secs: f32,
    current_duration: f32,
    current_probability: f32,
    rng: EffectRng,
}

impl SwitchTimer {
    /// Creates a new timer.
    ///
    /// # Arguments
    ///
    /// - `mean_effect_duration_secs` - For how many seconds an effect should be shown, on average.
    pub fn new(mean_effect_duration_secs: f32) -> Self {
        Self {
            mean_effect_duration_secs,
            current_duration: 0.0,
            current_probability: 0.0,
            rng: EffectRng::new(),
        }
    }

    /// Change for how many seconds an effect should be shown, on average.
    pub fn set_effect_duration(&mut self, duration_secs: f32) {
        self.mean_effect_duration_secs = duration_secs;
    }

    /// Updates the timer.
    ///
    /// Returns true if the effect must be changed.
    pub fn update(&mut self, d_t: f32, switch_possible: bool) -> bool {
        self.current_duration += d_t;

        if !switch_possible {
            return false;
        }

        let target_probability =
            Self::switching_cdf(self.mean_effect_duration_secs, self.current_duration);

        if target_probability <= self.current_probability {
            return false;
        }
        if self.current_probability >= 1.0 {
            return true;
        }

        let draw_probability =
            (target_probability - self.current_probability) / (1.0 - self.current_probability);

        let should_finish = if draw_probability >= 1.0 {
            true
        } else {
            self.rng.gen_bool(f64::from(draw_probability))
        };

        if should_finish {
            self.reset();
        } else {
            self.update_probability(draw_probability);
        }

        should_finish
    }

    fn update_probability(&mut self, new_prob: f32) {
        self.current_probability = 1.0 - (1.0 - self.current_probability) * (1.0 - new_prob);
    }

    /// Computes the desired switching CDF over time
    fn switching_cdf(mean: f32, t: f32) -> f32 {
        1.0 - (1.0 - 1.0 / mean).powf(t)
    }

    /// Resets the timer.
    ///
    /// Should be done at the start of a new effect.
    pub fn reset(&mut self) {
        self.current_duration = 0.0;
        self.current_probability = 0.0;
    }
}
