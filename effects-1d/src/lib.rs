#![doc = include_str!("../README.md")]
#![deny(unsafe_code)]
#![no_std]
#![allow(clippy::collapsible_if)]

// Adds effect submodules and exports the effect types
macro_rules! export_effects {
    ($modname: ident, $color: ty, {$($name: ident),* $(,)?}) => {
        ::pastey::paste!{
            pub mod $modname {
                $(
                    mod [<$name:snake>];
                    pub use [<$name:snake>]::$name;
                )*
            }

            #[derive(Debug, Clone, Copy, Eq, PartialEq)]
            pub enum [< $modname:camel Effect >] {
                $(
                    [<$name>],
                )*
            }

            #[derive(Debug)]
            pub enum [< $modname:camel EffectInstance >] {
                $(
                    [<$name>]($modname::[<$name>]),
                )*
            }

            impl [< $modname:camel Effect >] {
                pub fn create(self, resolution_hint: Option<u32>, start_beat: i32) -> [< $modname:camel EffectInstance >] {
                    match self {
                        $(
                            Self::[<$name>] => {
                                [< $modname:camel EffectInstance >]::[<$name>](::effects_1d_common::effects::ConstructibleBeatBasedEffect::init(resolution_hint, start_beat))
                            }
                        )*
                    }
                }

                pub fn available() -> &'static [Self] {
                    &[
                        $(
                            Self::[<$name>],
                        )*
                    ]
                }
            }

            impl ::core::fmt::Display for [< $modname:camel Effect >] {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error> {
                    ::core::fmt::Debug::fmt(self, f)
                }
            }

            impl [< $modname:camel EffectInstance >] {
                pub fn as_effect(&mut self) -> &mut dyn ::effects_1d_common::effects::BeatBasedEffect<Color = $color> {
                    match self {
                        $(
                            Self::[<$name>](effect) => effect,
                        )*
                    }
                }
            }
        }
    }
}

export_effects!(binary, ::effects_1d_common::color::Binary, {
    BlinkingStripes,
    Lighthouse,
    StrobingRotatingLines,
    Wiping,
    JumpingLine,
    ScannerBounce,
    BinaryMarchingAnts,
});

export_effects!(binary_rgb, ::effects_1d_common::color::BinaryRGB, {
    JumpingStripes,
});

export_effects!(monochrome, ::effects_1d_common::color::Monochrome, {
    SplittingLine,
    RotatingLines,
    RotatingLinesFilled,
    OpeningClosingFan,
    FanWave,
    RandomBlinkingPixels,
    MonoPsychedelic1,
    AsyncWave,
    BinaryExplosions,
    CometTrail,
    PulseCollision,
    BeatEqualizer,
    RippleField,
});

export_effects!(rgb, ::effects_1d_common::color::RGB, {
    AppearingRainbowStripes,
    RotatingRainbow,
    TwoColorWaves,
    RotatingFronts,
    RotatingLinesWithBorders,
    FadingColoredLighthouse,
    RapidFireStripes,
    SegmentedSnake,
    Psychedelic1,
    CometTrail,
    PulseCollision,
    BreathingGradient,
    ScannerBounce,
    BeatEqualizer,
    ColorOrbit,
    RippleField,
    MeasureChaser,
    AuroraRibbon,
});

export_effects!(calibration_monochrome, ::effects_1d_common::color::Monochrome, {
    ColorTestMonochrome,
});

export_effects!(calibration_rgb, ::effects_1d_common::color::RGB, {
    ColorTestRgbGradients,
    ColorTestRgbHslGradient,
});
