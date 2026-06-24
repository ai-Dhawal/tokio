use super::{BlockingRegionGuard, SetCurrentGuard, CONTEXT};
use crate::runtime::scheduler;
use crate::util::rand::{FastRand, RngSeed};
use std::fmt;
#[derive(Debug, Clone, Copy)]
#[must_use]
pub(crate) enum EnterRuntime {
    /// Currently in a runtime context.
    #[cfg_attr(not(feature = "rt"), allow(dead_code))]
    Entered { allow_block_in_place: bool },
    /// Not in a runtime context **or** a blocking region.
    NotEntered,
}
/// Guard tracking that a caller has entered a runtime context.
#[must_use]
pub(crate) struct EnterRuntimeGuard {
    /// Tracks that the current thread has entered a blocking function call.
    pub(crate) blocking: BlockingRegionGuard,
    #[allow(dead_code)]
    pub(crate) handle: SetCurrentGuard,
    old_seed: RngSeed,
}
/// Marks the current thread as being within the dynamic extent of an
/// executor.
#[track_caller]
pub(crate) fn enter_runtime<F, R>(
    handle: &scheduler::Handle,
    allow_block_in_place: bool,
    f: F,
) -> R
where
    F: FnOnce(&mut BlockingRegionGuard) -> R,
{
    panic!("STUB: not implemented");
}
impl fmt::Debug for EnterRuntimeGuard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl Drop for EnterRuntimeGuard {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
impl EnterRuntime {
    pub(crate) fn is_entered(self) -> bool {
        panic!("STUB: not implemented");
    }
}
