#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
//#![deny(unsafe_code)]
#![no_std]
#![feature(core_intrinsics)]

/// Pixel colors
pub mod color;
/// Utilities that assist in drawing shapes
pub mod drawing;
/// Effect-releated traits and structs
pub mod effects;
/// Common error types
pub mod errors;
/// Linear interpolation
pub mod lerp;
/// Math functionality that is missing in `core`.
pub mod math;
/// Randomness
pub mod random;
/// Sequencer for beat based cyclic actions
pub mod rhythm;

/// Target-specific imports like F32Ext
///
/// Meant to be used as `use effects-1d-common::prelude::*;`.
pub mod prelude {
    pub use super::math::F32Ext;
}
