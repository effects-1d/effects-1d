/// An object that can be interpolated between two values
pub trait Lerp {
    /// Interpolates between `self` and `other`.
    ///
    /// A `pct` value of `0.0` returns `self`, a value of `1.0` returns `other`.
    ///
    /// `pct` can be larger than `1.0` or smaller than `0.0`, in which case it will
    /// extrapolate beyound the limit values.
    fn lerp(self, other: Self, pct: f32) -> Self;

    /// Interpolates between `self` and `other`.
    ///
    /// A `pct` value of `0.0` returns `self`, a value of `1.0` returns `other`.
    ///
    /// Contrary to [`lerp()`], it will not extrapolate beyond 0.0 and 1.0, clamping `pct`
    /// inside of that range.
    #[inline]
    fn clamping_lerp(self, other: Self, pct: f32) -> Self
    where
        Self: Sized,
    {
        self.lerp(other, pct.clamp(0.0, 1.0))
    }
}

macro_rules! impl_lerp_for_int {
    ($($t: ty)+) => {
        $(
            impl Lerp for $t {
                #[inline]
                fn lerp(self, other: Self, pct: f32) -> Self {
                    let self_f = self as f32;
                    let other_f = other as f32;
                    let result_f = self_f.lerp(other_f, pct);
                    (result_f + 0.5).clamp(<$t>::MIN as f32, <$t>::MAX as f32) as $t
                }
            }
        )+
    };
}

macro_rules! impl_lerp_for_float {
    ($($t: ty)+) => {
        $(
            impl Lerp for $t {
                #[inline]
                fn lerp(self, other: Self, pct: f32) -> Self {
                    let pct: $t = pct.into();
                    self * (1.0 - pct) + other * pct
                }
            }
        )+
    };
}

impl_lerp_for_int!(u8 u16 u32 u64 i8 i16 i32 i64);
impl_lerp_for_float!(f32 f64);
