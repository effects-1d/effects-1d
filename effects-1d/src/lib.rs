#![doc = include_str!("../README.md")]
#![deny(unsafe_code)]
#![no_std]

// Adds effect submodules and exports the effect types
macro_rules! export_effects {
    ($($name: ident),* $(,)?) => {
        $(
            ::paste::paste!{
                mod [<$name:snake>];
                pub use [<$name:snake>]::$name;
            }
        )*
    }
}

pub mod binary {
    export_effects! {
        BlinkingStripes,
        Lighthouse,
        StrobingRotatingLines,
        //Wiping,
    }
}

pub mod binary_rgb {
    export_effects! {
        JumpingStripes,
    }
}

pub mod monochrome {
    export_effects! {
        SplittingLine,
    }
}

pub mod rgb {
    export_effects! {
        AppearingRainbowStripes,
        RotatingRainbow,
        TwoColorWaves,
        RotatingFronts
    }
}
