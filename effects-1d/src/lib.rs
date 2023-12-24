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
        Wiping,
        JumpingLine,
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
        RotatingLines,
        RotatingLinesFilled,
        OpeningClosingFan,
        FanWave,
        RandomBlinkingPixels,
        MonoPsychedelic1,
        AsyncWave,
    }
}

pub mod rgb {
    export_effects! {
        AppearingRainbowStripes,
        RotatingRainbow,
        TwoColorWaves,
        RotatingFronts,
        RotatingLinesWithBorders,
        FadingColoredLighthouse,
        RapidFireStripes,
        SegmentedSnake,
        Psychedelic1,
    }
}

pub mod calibration {
    export_effects! {
        ColorTestMonochrome,
        ColorTestRgbGradients,
        ColorTestRgbHslGradient,
    }
}
