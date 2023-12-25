#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![deny(unsafe_code)]
#![no_std]

mod switch_timer;
pub use switch_timer::SwitchTimer;
