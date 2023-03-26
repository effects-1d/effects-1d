mod time_based_effect;
pub use time_based_effect::TimeBasedEffect;

/// The current state of an effect
pub struct EffectState {
    /// The effect is finished and a new effect
    /// needs to be played immediately
    pub over: bool,

    /// The effect can be scheduled out if desired.
    ///
    /// Meant for effects that don't have a fix end;
    /// whether or not they get scheduled out is a decision
    /// of the runtime
    pub idle: bool,
    // TODO: Add transition hints
}
