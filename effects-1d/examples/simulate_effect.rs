//! This example is meant to simulate a given effect, for debugging purposes.

#[allow(unused_imports)]
use effects_1d::{binary::*, binary_rgb::*, calibration::*, monochrome::*, rgb::*};
use effects_1d_simulator::SimulateEffect;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    RotatingLinesFilled::simulate();
}
