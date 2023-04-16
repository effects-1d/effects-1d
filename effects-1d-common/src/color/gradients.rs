//! Everything related to gradients.

use super::{Color, Monochrome, TransparentColor, RGB};
use palette::{FromColor, IntoColor, Mix};

/// A color gradient that can be used to interpolate between two colors
pub trait ColorGradient {
    /// The color type of the gradient
    type C;

    /// Produce the color at the given position
    ///
    /// # Arguments
    ///
    /// * `position` - How dominant the other color should be, from 0.0 to 1.0.
    ///
    fn interpolate(&self, position: f32) -> Self::C;

    /// Adds an alpha value to the gradient
    fn with_transparency(self, alpha_start: f32, alpha_end: f32) -> TransparentGradient<Self>
    where
        Self: Sized,
        Self::C: Color,
    {
        TransparentGradient {
            alpha_start,
            alpha_end,
            color_gradient: self,
        }
    }
}

/// Implements an RGB two-color gradient based on a specific color space.
macro_rules! two_color_gradient_impl {
    ($blendtype:ident) => {
        ::paste::paste! {
            #[doc = "An RGB gradient based on `" $blendtype "` blending."]
            pub struct [<$blendtype Gradient>] {
                start: palette::$blendtype,
                end: palette::$blendtype,
            }
            impl [<$blendtype Gradient>] {
                /// Creates a new gradient
                pub fn new(start: $crate::color::RGB, end: $crate::color::RGB) -> Self {
                    Self {
                        start: start.into_format().into_color(),
                        end: end.into_format().into_color(),
                    }
                }
            }
            impl ColorGradient for [<$blendtype Gradient>] {
                type C = $crate::color::RGB;

                fn interpolate(&self, position: f32) -> Self::C {
                    palette::Srgb::from_color(self.start.mix(self.end, position.clamp(0.0, 1.0))).into_format()
                }
            }
        }
    };
}

two_color_gradient_impl!(Oklab);
two_color_gradient_impl!(LinSrgb);
two_color_gradient_impl!(Srgb);
two_color_gradient_impl!(Hsl);

/// A rainbow gradient.
pub struct HsvRainbowGradient {
    /// The 's' component of the Hsv colors.
    /// Must be between `0.0` and `1.0`.
    pub saturation: f32,

    /// The 'v' component of the Hsv colors.
    /// Must be between `0.0` and `1.0`.
    pub brightness: f32,

    /// An offset of `360.0` produces the same color as an offset of `0`.
    pub offset: f32,

    /// A factor of `2.0` means that the colors are twice as close together.
    pub scale: f32,

    /// Reverses the color order
    pub reversed: bool,
}

impl ColorGradient for HsvRainbowGradient {
    type C = RGB;

    fn interpolate(&self, position: f32) -> RGB {
        let value: palette::Srgb = palette::Hsv::new(
            360.0
                * self.scale
                * (if self.reversed {
                    1.0 - position
                } else {
                    position
                })
                + self.offset,
            self.saturation,
            self.brightness,
        )
        .into_color();
        value.into_format()
    }
}

/// A linear gradient for monochrome colors.
pub struct MonochromeGradient {
    start: f32,
    end: f32,
}
impl MonochromeGradient {
    /// Creates a new gradient
    pub fn new(start: Monochrome, end: Monochrome) -> Self {
        Self {
            start: start.v.into(),
            end: end.v.into(),
        }
    }
}

impl ColorGradient for MonochromeGradient {
    type C = Monochrome;

    fn interpolate(&self, position: f32) -> Self::C {
        Monochrome {
            v: super::lerp16f(self.start, self.end, position),
        }
    }
}

/// A gradient that isn't actually a gradient but consist of just a single color.
pub struct SingleColorGradient<C: Color> {
    color: C,
}

impl<C: Color> SingleColorGradient<C> {
    /// Creates a new single-color gradient.
    pub fn new(color: C) -> Self {
        Self { color }
    }
}

impl<C: Color> ColorGradient for SingleColorGradient<C> {
    type C = C;

    fn interpolate(&self, _position: f32) -> Self::C {
        self.color
    }
}

/// A transparent gradient.
pub struct TransparentGradient<G: ColorGradient> {
    alpha_start: f32,
    alpha_end: f32,
    color_gradient: G,
}

impl<G: ColorGradient> ColorGradient for TransparentGradient<G> {
    type C = TransparentColor<G::C>;

    fn interpolate(&self, position: f32) -> Self::C {
        TransparentColor {
            alpha: self.alpha_start * (1. - position) + self.alpha_end * position,
            value: self.color_gradient.interpolate(position),
        }
    }
}

/// A generic gradient based on a provided closure.
pub struct CustomGradient<T> {
    f: T,
}

impl<T, C> CustomGradient<T>
where
    T: Fn(f32) -> C,
{
    /// Creates a new gradient based on the given color generator function
    pub fn new(f: T) -> Self {
        Self { f }
    }
}

impl<T, C> ColorGradient for CustomGradient<T>
where
    T: Fn(f32) -> C,
{
    type C = C;

    fn interpolate(&self, position: f32) -> Self::C {
        (self.f)(position)
    }
}
