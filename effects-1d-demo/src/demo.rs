mod backends;
use backends::EffectsDemoBackend;
mod effect_collection;

use effects_1d_common::random::{rand::rngs::OsRng, EffectRng, Rng};
use effects_1d_simulator::run_simulation;

pub fn run() {
    EffectRng::seed(OsRng.gen());

    let effect_renderer: EffectsDemoBackend = Default::default();
    run_simulation(effect_renderer);
}
