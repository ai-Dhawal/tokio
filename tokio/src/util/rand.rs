cfg_rt! {
    mod rt; pub (crate) use rt::RngSeedGenerator; cfg_unstable! { mod rt_unstable; }
}
/// A seed for random number generation.
///
/// In order to make certain functions within a runtime deterministic, a seed
/// can be specified at the time of creation.
#[allow(unreachable_pub)]
#[derive(Clone, Debug)]
pub struct RngSeed {
    s: u32,
    r: u32,
}
/// Fast random number generate.
///
/// Implement `xorshift64+`: 2 32-bit `xorshift` sequences added together.
/// Shift triplet `[17,7,16]` was calculated as indicated in Marsaglia's
/// `Xorshift` paper: <https://www.jstatsoft.org/article/view/v008i14/xorshift.pdf>
/// This generator passes the SmallCrush suite, part of TestU01 framework:
/// <http://simul.iro.umontreal.ca/testu01/tu01.html>
#[derive(Clone, Copy, Debug)]
pub(crate) struct FastRand {
    one: u32,
    two: u32,
}
impl RngSeed {
    /// Creates a random seed using loom internally.
    pub(crate) fn new() -> Self {
        panic!("STUB: not implemented");
    }
    fn from_u64(seed: u64) -> Self {
        panic!("STUB: not implemented");
    }
    fn from_pair(s: u32, r: u32) -> Self {
        panic!("STUB: not implemented");
    }
}
impl FastRand {
    /// Initialize a new fast random number generator using the default source of entropy.
    pub(crate) fn new() -> FastRand {
        panic!("STUB: not implemented");
    }
    /// Initializes a new, thread-local, fast random number generator.
    pub(crate) fn from_seed(seed: RngSeed) -> FastRand {
        panic!("STUB: not implemented");
    }
    #[cfg(
        any(
            feature = "macros",
            feature = "rt-multi-thread",
            all(feature = "sync", feature = "rt")
        )
    )]
    pub(crate) fn fastrand_n(&mut self, n: u32) -> u32 {
        panic!("STUB: not implemented");
    }
    fn fastrand(&mut self) -> u32 {
        panic!("STUB: not implemented");
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn non_zero_seed_from_u64() {
        let seed = RngSeed::from_u64(0);
        assert_eq!(seed.s, 0);
        assert_eq!(seed.r, 1);
    }
    #[test]
    fn non_zero_seed_from_pair() {
        let seed = RngSeed::from_pair(0, 0);
        assert_eq!(seed.s, 0);
        assert_eq!(seed.r, 1);
    }
}
