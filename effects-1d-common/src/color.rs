/// 24-bit RGB Color
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RGB {
    /// Red
    pub r: u8,
    /// Green
    pub g: u8,
    /// Blue
    pub b: u8,
}

/// 8-bit Monochrome Color
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Monochrome {
    /// Brightness value
    pub v: u8,
}

/// 3-bit RGB Color
///
/// One bit each for R, G and B.
/// Can represent 8 different colors (including black and white).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BinaryRGB {
    /// Red
    pub r: bool,
    /// Green
    pub g: bool,
    /// Blue
    pub b: bool,
}

/// 1-bit Color
///
/// Can only represent on (true) and off (false).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Binary {
    /// On or Off
    pub v: bool,
}

impl Binary {
    /// Creates an 'on' value.
    pub const fn on() -> Self {
        Self { v: true }
    }
    /// Creates an 'off' value.
    pub const fn off() -> Self {
        Self { v: false }
    }
}

/// Common functionality of all colors
pub trait Color: Copy + core::fmt::Debug + Eq + PartialEq {}

impl Color for RGB {}
impl Color for Monochrome {}
impl Color for BinaryRGB {}
impl Color for Binary {}
