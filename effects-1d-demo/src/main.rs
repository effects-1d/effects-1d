mod backends;

use backends::EffectsDemoBackend;
use effects_1d_common::random::{rand::rngs::OsRng, EffectRng, Rng};
use effects_1d_simulator::run_simulation;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    EffectRng::seed(OsRng.gen());

    let effect_renderer: EffectsDemoBackend = Default::default();
    run_simulation(effect_renderer);
}
