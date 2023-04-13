/// 96-bit Okhsv Color.
///
/// Okhsv was chosen over RGB for better color blending.
pub type Okhsv = palette::Okhsv<f32>;
pub use palette;
use palette::MixAssign;

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

impl PartialEq<u16> for Monochrome {
    fn eq(&self, other: &u16) -> bool {
        self.v.eq(other)
    }
}

impl core::ops::AddAssign for Monochrome {
    fn add_assign(&mut self, rhs: Self) {
        self.v = self.v.saturating_add(rhs.v)
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

impl Color for Okhsv {
    fn zero() -> Self {
        Self::new(0., 0., 0.)
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

/// Common functionality for interpolatable colors
pub trait InterpolatableColor: Color + core::ops::AddAssign {
    /// Interpolates between the current color and another color.
    ///
    /// # Arguments
    ///
    /// * `other` - The other color to interpolate to.
    /// * `percent` - How dominant the other color should be, from 0.0 to 1.0.
    fn interpolate(self, other: Self, percent: f32) -> Self;

    /// Creates the maximum between two colors
    fn assign_elementwise_max(&mut self, other: Self);
}

fn lerp16(value_a: u16, value_b: u16, percent: f32) -> u16 {
    let percent = percent.clamp(0.0, 1.0);

    let a: f32 = value_a.into();
    let b: f32 = value_b.into();

    // the + 0.5 is for proper rounding; float->int conversion is always a floor() operation
    (a * (1.0 - percent) + b * percent + 0.5).clamp(0.0, 65535.0) as u16
}

impl InterpolatableColor for Okhsv {
    fn interpolate(self, other: Self, percent: f32) -> Self {
        use palette::Mix;
        self.mix(other, percent)
    }

    fn assign_elementwise_max(&mut self, other: Self) {
        let max_value = self.value.max(other.value);
        let total_value = self.value + other.value;
        if total_value == 0.0 {
            return;
        }
        let mix_factor = other.value / total_value;

        self.mix_assign(other, mix_factor);
        self.value = max_value;
    }
}

impl InterpolatableColor for Monochrome {
    fn interpolate(self, other: Self, percent: f32) -> Self {
        Self {
            v: lerp16(self.v, other.v, percent),
        }
    }

    fn assign_elementwise_max(&mut self, other: Self) {
        self.v = self.v.max(other.v);
    }
}
