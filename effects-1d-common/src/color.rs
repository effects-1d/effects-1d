/// 24-bit RGB Color
pub struct RGB {
    /// Red
    pub r: u8,
    /// Green
    pub g: u8,
    /// Blue
    pub b: u8,
}

/// 8-bit Monochrome Color
pub struct Monochrome {
    /// Brightness value
    pub v: u8,
}

/// 3-bit RGB Color
///
/// One bit each for R, G and B.
/// Can represent 8 different colors (including black and white).
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
pub struct Binary {
    /// On or Off
    pub v: bool,
}

/// Common functionality of all colors
pub trait Color {}

impl Color for RGB {}
impl Color for Monochrome {}
impl Color for BinaryRGB {}
impl Color for Binary {}
