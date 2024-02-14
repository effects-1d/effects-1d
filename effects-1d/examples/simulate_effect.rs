//! This example is meant to simulate a given effect, for debugging purposes.

#[allow(unused_imports)]
use effects_1d::{binary::*, binary_rgb::*, calibration::*, monochrome::*, rgb::*};
use effects_1d_simulator::SimulateEffect;

fn main() {
    tracing_subscriber::fmt()
        // all spans/events with a level higher than TRACE (e.g, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(tracing::Level::DEBUG)
        // sets this to be the default, global collector for this application.
        .init();

    RotatingLinesFilled::simulate();
}
