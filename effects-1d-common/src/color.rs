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

impl RGB {
    /// A color with all values set to zero
    pub const fn black() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }
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

/// Common functionality for interpolatable colors
pub trait InterpolatableColor: Color {
    /// Interpolates between the current color and another color.
    ///
    /// # Arguments
    ///
    /// * `other` - The other color to interpolate to.
    /// * `percent` - How dominant the other color should be, from 0.0 to 1.0.
    fn interpolate(&self, other: &Self, percent: f32) -> Self;
}

fn lerp8(value_a: u8, value_b: u8, percent: f32) -> u8 {
    let percent = percent.clamp(0.0, 1.0);

    let a: f32 = value_a.into();
    let b: f32 = value_b.into();

    // the + 0.5 is for proper rounding; float->int conversion is always a floor() operation
    (a * (1.0 - percent) + b * percent + 0.5).clamp(0.0, 255.0) as u8
}

impl InterpolatableColor for RGB {
    fn interpolate(&self, other: &Self, percent: f32) -> Self {
        Self {
            r: lerp8(self.r, other.r, percent),
            g: lerp8(self.g, other.g, percent),
            b: lerp8(self.b, other.b, percent),
        }
    }
}
impl InterpolatableColor for Monochrome {
    fn interpolate(&self, other: &Self, percent: f32) -> Self {
        Self {
            v: lerp8(self.v, other.v, percent),
        }
    }
}
