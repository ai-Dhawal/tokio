use super::{EnterRuntime, CONTEXT};
use crate::loom::thread::AccessError;
use crate::util::markers::NotSendOrSync;
use std::marker::PhantomData;
use std::time::Duration;
/// Guard tracking that a caller has entered a blocking region.
#[must_use]
pub(crate) struct BlockingRegionGuard {
    _p: PhantomData<NotSendOrSync>,
}
pub(crate) struct DisallowBlockInPlaceGuard(bool);
pub(crate) fn try_enter_blocking_region() -> Option<BlockingRegionGuard> {
    panic!("STUB: not implemented");
}
/// Disallows blocking in the current runtime context until the guard is dropped.
pub(crate) fn disallow_block_in_place() -> DisallowBlockInPlaceGuard {
    panic!("STUB: not implemented");
}
impl BlockingRegionGuard {
    pub(super) fn new() -> BlockingRegionGuard {
        panic!("STUB: not implemented");
    }
    /// Blocks the thread on the specified future, returning the value with
    /// which that future completes.
    pub(crate) fn block_on<F>(&mut self, f: F) -> Result<F::Output, AccessError>
    where
        F: std::future::Future,
    {
        panic!("STUB: not implemented");
    }
    /// Blocks the thread on the specified future for **at most** `timeout`
    ///
    /// If the future completes before `timeout`, the result is returned. If
    /// `timeout` elapses, then `Err` is returned.
    pub(crate) fn block_on_timeout<F>(
        &mut self,
        f: F,
        timeout: Duration,
    ) -> Result<F::Output, ()>
    where
        F: std::future::Future,
    {
        panic!("STUB: not implemented");
    }
}
impl Drop for DisallowBlockInPlaceGuard {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
