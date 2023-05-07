pub mod gradients;
pub use palette;
use palette::IntoColor;

use crate::lerp::Lerp;

/// 48-bit sRGB Color.
pub type RGB = palette::Srgb<u16>;

/// Creates an RGB color from 8bit color values.
pub fn rgb8(r: u8, g: u8, b: u8) -> RGB {
    palette::Srgb::<u8>::new(r, g, b).into_format()
}

/// Creates an RGB color from 8bit hsl color values.
pub fn hsv8(h: f32, s: f32, v: f32) -> RGB {
    let rgb: palette::Srgb<f32> = palette::Hsv::new(h, s, v).into_color();
    rgb.into_format()
}

/// 16-bit Monochrome Color
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Monochrome {
    /// Brightness value
    pub v: u16,
}

impl Monochrome {
    /// Creates a new monochrome color value.
    pub const fn new(v: u16) -> Self {
        Self { v }
    }

    /// Creates a monochrome color with maximum brightness.
    pub const fn full() -> Self {
        Self { v: u16::MAX }
    }
}

impl From<f32> for Monochrome {
    fn from(value: f32) -> Self {
        Self {
            v: (value * (f32::from(u16::MAX) + 1.0)) as u16,
        }
    }
}

impl PartialEq<u16> for Monochrome {
    fn eq(&self, other: &u16) -> bool {
        self.v.eq(other)
    }
}

/// 3-bit RGB Color
///
/// One bit each for R, G and B.
/// Can represent 8 different colors (including black and white).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct BinaryRGB {
    /// Red
    pub r: bool,
    /// Green
    pub g: bool,
    /// Blue
    pub b: bool,
}

impl BinaryRGB {
    /// Creates the color 'black'
    pub const fn black() -> Self {
        Self {
            r: false,
            g: false,
            b: false,
        }
    }
    /// Creates the color 'white'
    pub const fn white() -> Self {
        Self {
            r: true,
            g: true,
            b: true,
        }
    }
    /// Creates the color 'red'
    pub const fn red() -> Self {
        Self {
            r: true,
            g: false,
            b: false,
        }
    }
    /// Creates the color 'green'
    pub const fn green() -> Self {
        Self {
            r: false,
            g: true,
            b: false,
        }
    }
    /// Creates the color 'blue'
    pub const fn blue() -> Self {
        Self {
            r: false,
            g: false,
            b: true,
        }
    }
    /// Creates the color 'yellow'
    pub const fn yellow() -> Self {
        Self {
            r: true,
            g: true,
            b: false,
        }
    }
    /// Creates the color 'magenta'
    pub const fn magenta() -> Self {
        Self {
            r: true,
            g: false,
            b: true,
        }
    }
    /// Creates the color 'cyan'
    pub const fn cyan() -> Self {
        Self {
            r: false,
            g: true,
            b: true,
        }
    }

    /// A list of all possible colors (excluding black and white)
    pub const fn all_possible_colors() -> &'static [BinaryRGB; 6] {
        const ALL_COLORS: [BinaryRGB; 6] = [
            BinaryRGB::red(),
            BinaryRGB::yellow(),
            BinaryRGB::green(),
            BinaryRGB::cyan(),
            BinaryRGB::blue(),
            BinaryRGB::magenta(),
        ];
        &ALL_COLORS
    }
}

/// 1-bit Color
///
/// Can only represent on (true) and off (false).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
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
pub trait Color: Copy + core::fmt::Debug {
    /// Returns the zero value of the given color type.
    ///
    /// This is the value that can be added to any color without changing it.
    fn zero() -> Self;
}

impl Color for RGB {
    fn zero() -> Self {
        Self::new(0, 0, 0)
    }
}
impl Color for Monochrome {
    fn zero() -> Self {
        Self::new(0)
    }
}
impl Color for BinaryRGB {
    fn zero() -> Self {
        Self::black()
    }
}
impl Color for Binary {
    fn zero() -> Self {
        Self::off()
    }
}

/// Common functionality for blendable colors
pub trait BlendableColor: Color {
    /// Creates the maximum between two colors
    #[must_use]
    fn elementwise_max(self, other: Self) -> Self;

    /// Adds another color to this color.
    ///
    /// Performs a saturating add on each channel.
    #[must_use]
    fn elementwise_add(self, other: Self) -> Self;

    /// Linearly interpolates this color to another.
    ///
    /// A `percent` of `0.0` will return `self`,
    /// a `percent` of `1.0` will return `other`.
    ///
    /// Meant for transparent edges.
    /// Should not be used to generate gradients - use gradients instead.
    #[must_use]
    fn elementwise_lerp(self, other: Self, percent: f32) -> Self;
}

impl BlendableColor for RGB {
    #[inline]
    fn elementwise_max(self, other: Self) -> Self {
        RGB::new(
            self.red.max(other.red),
            self.green.max(other.green),
            self.blue.max(other.blue),
        )
    }

    #[inline]
    fn elementwise_add(self, other: Self) -> Self {
        RGB::new(
            self.red.saturating_add(other.red),
            self.green.saturating_add(other.green),
            self.blue.saturating_add(other.blue),
        )
    }

    #[inline]
    fn elementwise_lerp(self, other: Self, percent: f32) -> Self {
        RGB::new(
            self.red.clamping_lerp(other.red, percent),
            self.green.clamping_lerp(other.green, percent),
            self.blue.clamping_lerp(other.blue, percent),
        )
    }
}

impl BlendableColor for Monochrome {
    #[inline]
    fn elementwise_max(self, other: Self) -> Self {
        Self {
            v: self.v.max(other.v),
        }
    }

    #[inline]
    fn elementwise_add(self, other: Self) -> Self {
        Self {
            v: self.v.saturating_add(other.v),
        }
    }

    #[inline]
    fn elementwise_lerp(self, other: Self, percent: f32) -> Self {
        Self {
            v: self.v.clamping_lerp(other.v, percent),
        }
    }
}

/// A color with an attached transparency value
#[derive(Debug, Copy, Clone)]
pub struct TransparentColor<C> {
    /// The color component.
    pub value: C,
    /// The transparency component.
    pub alpha: f32,
}
