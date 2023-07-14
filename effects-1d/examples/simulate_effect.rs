//! This example is meant to simulate a given effect, for debugging purposes.

#[allow(unused_imports)]
use effects_1d::{binary::*, binary_rgb::*, calibration::*, monochrome::*, rgb::*};
use effects_1d_simulator::SimulateEffect;

fn main() {
    MonoPsychedelic1::simulate();
}
