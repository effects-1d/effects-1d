#![doc = include_str!("../README.md")]
#![deny(unsafe_code)]
#![no_std]

// Adds effect submodules and exports the effect types
macro_rules! export_effects {
    ($($name: ident),*) => {
        $(
            ::paste::paste!{
                mod [<$name:snake>];
                pub use [<$name:snake>]::$name;
            }
        ),*
    }
}

export_effects!(BlinkingStripes);
