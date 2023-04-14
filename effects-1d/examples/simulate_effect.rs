//! This example is meant to simulate a given effect, for debugging purposes.

#[allow(unused_imports)]
use effects_1d::{binary::*, binary_rgb::*, monochrome::*, rgb::*};
use effects_1d_frontend_simulator::SimulateEffect;

fn main() {
    SplittingLine::simulate();
}
