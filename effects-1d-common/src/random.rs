/// Re-export of the `rand` crate
pub use rand::Rng;

/// The random number generator that should be
/// used for everything related to effects
pub struct EffectRng(rand::rngs::SmallRng);
impl EffectRng {
    /// Creates a random number generator
    pub fn new() -> Self {
        use rand::SeedableRng;
        Self(rand::rngs::SmallRng::from_entropy())
    }
}

impl rand::RngCore for EffectRng {
    fn next_u32(&mut self) -> u32 {
        self.0.next_u32()
    }
    fn next_u64(&mut self) -> u64 {
        self.0.next_u64()
    }
    fn fill_bytes(&mut self, d: &mut [u8]) {
        self.0.fill_bytes(d)
    }
    fn try_fill_bytes(&mut self, d: &mut [u8]) -> Result<(), rand::Error> {
        self.0.try_fill_bytes(d)
    }
}
impl core::fmt::Debug for EffectRng {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("EffectRng").finish()
    }
}
