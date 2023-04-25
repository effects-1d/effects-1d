#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![no_std]

/// Pixel colors
pub mod color;
/// Utilities that assist in drawing shapes
pub mod drawing;
/// Effect-releated traits and structs
pub mod effects;
/// Common error types
pub mod errors;
/// Randomness
pub mod random;
/// Sequencer for beat based cyclic actions
pub mod rhythm;
