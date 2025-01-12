use rand::{
    Error, Rng, RngCore, SeedableRng,
    distributions::uniform::SampleRange,
    rngs::{StdRng, ThreadRng},
    thread_rng,
};

/// A fake data generator, with an internal RNG which could be seeded to generate deterministic
/// results.
///
/// The [`Unreal`] struct implements the [`RngCore`] trait and therefore the [`Rng`] trait, so it
/// can be used as any other RNG.
pub struct Unreal<R: RngCore> {
    pub(crate) rng: R,
}

impl<R: RngCore> Unreal<R> {
    /// Create a new [`Unreal`] with the given RNG.
    pub const fn from_rng(rng: R) -> Self {
        Self { rng }
    }

    pub(crate) fn choose<T: Copy, const N: usize>(&mut self, array: [T; N]) -> T {
        array[self.gen_range(0..N)]
    }

    pub(crate) fn numbers(&mut self, range: impl SampleRange<usize>, min_width: usize) -> String {
        format!("{:0>min_width$}", self.gen_range(range))
    }
}

impl Unreal<StdRng> {
    #[must_use]
    /// Create a new [`Unreal`] with a seeded [`StdRng`].
    pub fn from_stdrng_seed(seed: u64) -> Self {
        Self::from_rng(StdRng::seed_from_u64(seed))
    }
}

impl<R: RngCore + SeedableRng> Unreal<R> {
    #[must_use]
    /// Create a new [`Unreal`] with a seeded RNG.
    pub fn from_seed(seed: u64) -> Self {
        Self::from_rng(R::seed_from_u64(seed))
    }
}

impl Unreal<ThreadRng> {
    #[must_use]
    /// Create a new [`Unreal`] with the thread-local [`ThreadRng`].
    pub fn from_thread_rng() -> Self {
        Self::from_rng(thread_rng())
    }
}

impl<R: RngCore> RngCore for Unreal<R> {
    fn next_u32(&mut self) -> u32 {
        self.rng.next_u32()
    }

    fn next_u64(&mut self) -> u64 {
        self.rng.next_u64()
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.rng.fill_bytes(dest);
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        self.rng.try_fill_bytes(dest)
    }
}
