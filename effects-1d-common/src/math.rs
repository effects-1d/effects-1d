/// Math extensions for `f32`
pub trait F32Ext {
    /// Sinus
    fn sin(self) -> Self;
    /// Cosinus
    fn cos(self) -> Self;
    /// Absolute value
    fn abs(self) -> Self;
    /// Euclidean remainder
    fn rem_euclid(self, div: Self) -> Self;
}

impl F32Ext for f32 {
    fn sin(self) -> f32 {
        unsafe { core::intrinsics::sinf32(self) }
    }

    fn cos(self) -> f32 {
        unsafe { core::intrinsics::cosf32(self) }
    }

    fn abs(self) -> f32 {
        unsafe { core::intrinsics::fabsf32(self) }
    }

    fn rem_euclid(self, rhs: f32) -> f32 {
        let r = self % rhs;
        if r < 0.0 {
            r + rhs.abs()
        } else {
            r
        }
    }
}
