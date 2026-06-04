use core::{
    convert::Infallible,
    sync::atomic::{AtomicU32, Ordering},
};

/// Re-exported from the [`rand`] crate.
pub use rand::RngExt;

pub use rand;

static NEXT_SEED: AtomicU32 = AtomicU32::new(1);

/// The random number generator that should be
/// used for everything related to effects
pub struct EffectRng(rand::rngs::SmallRng);
impl EffectRng {
    /// Creates a random number generator
    pub fn new() -> Self {
        use rand::SeedableRng;
        let seed = NEXT_SEED.fetch_add(1, Ordering::Relaxed);
        Self(rand::rngs::SmallRng::seed_from_u64(seed.into()))
    }

    /// Initializes the random number generator with a given seed
    pub fn seed(val: u32) {
        NEXT_SEED.store(val, Ordering::Relaxed);
    }
}

impl rand::TryRng for EffectRng {
    type Error = Infallible;

    fn try_next_u32(&mut self) -> Result<u32, Infallible> {
        self.0.try_next_u32()
    }
    fn try_next_u64(&mut self) -> Result<u64, Infallible> {
        self.0.try_next_u64()
    }
    fn try_fill_bytes(&mut self, dst: &mut [u8]) -> Result<(), Infallible> {
        self.0.try_fill_bytes(dst)
    }
}
impl core::fmt::Debug for EffectRng {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("EffectRng").finish()
    }
}
