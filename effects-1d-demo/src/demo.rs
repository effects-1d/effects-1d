mod backends;
use backends::EffectsDemoBackend;
mod effect_collection;

use effects_1d_common::random::{
    rand::{self, rngs::StdRng},
    EffectRng, RngExt,
};
use effects_1d_simulator::run_simulation;

pub fn run() {
    EffectRng::seed(rand::make_rng::<StdRng>().random());

    let effect_renderer: EffectsDemoBackend = Default::default();
    run_simulation(effect_renderer);
}
