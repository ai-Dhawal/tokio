use super::{FastRand, RngSeed};
use std::sync::Mutex;
/// A deterministic generator for seeds (and other generators).
///
/// Given the same initial seed, the generator will output the same sequence of seeds.
///
/// Since the seed generator will be kept in a runtime handle, we need to wrap `FastRand`
/// in a Mutex to make it thread safe. Different to the `FastRand` that we keep in a
/// thread local store, the expectation is that seed generation will not need to happen
/// very frequently, so the cost of the mutex should be minimal.
#[derive(Debug)]
pub(crate) struct RngSeedGenerator {
    /// Internal state for the seed generator. We keep it in a Mutex so that we can safely
    /// use it across multiple threads.
    state: Mutex<FastRand>,
}
impl RngSeedGenerator {
    /// Returns a new generator from the provided seed.
    pub(crate) fn new(seed: RngSeed) -> Self {
        panic!("STUB: not implemented");
    }
    /// Returns the next seed in the sequence.
    pub(crate) fn next_seed(&self) -> RngSeed {
        panic!("STUB: not implemented");
    }
    /// Directly creates a generator using the next seed.
    pub(crate) fn next_generator(&self) -> Self {
        panic!("STUB: not implemented");
    }
}
impl FastRand {
    /// Replaces the state of the random number generator with the provided seed, returning
    /// the seed that represents the previous state of the random number generator.
    ///
    /// The random number generator will become equivalent to one created with
    /// the same seed.
    pub(crate) fn replace_seed(&mut self, seed: RngSeed) -> RngSeed {
        panic!("STUB: not implemented");
    }
}
