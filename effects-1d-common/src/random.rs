use core::sync::atomic::{AtomicU32, Ordering};

/// Re-exported from the [`rand`] crate.
pub use rand::Rng;

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
