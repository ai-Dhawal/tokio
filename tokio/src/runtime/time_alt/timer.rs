use super::{EntryHandle, TempLocalContext};
use crate::runtime::scheduler;
use crate::time::Instant;
use std::pin::Pin;
use std::task::{Context, Poll};
#[cfg(any(feature = "rt", feature = "rt-multi-thread"))]
use crate::util::error::RUNTIME_SHUTTING_DOWN_ERROR;
pub(crate) struct Timer {
    /// The entry in the timing wheel.
    entry: EntryHandle,
}
impl std::fmt::Debug for Timer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl Drop for Timer {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
impl Timer {
    #[track_caller]
    pub(crate) fn new(handle: scheduler::Handle, deadline: Instant) -> Self {
        panic!("STUB: not implemented");
    }
    pub(crate) fn is_elapsed(&self) -> bool {
        panic!("STUB: not implemented");
    }
    pub(crate) fn poll_elapsed(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        panic!("STUB: not implemented");
    }
}
fn with_current_temp_local_context<F, R>(sched_hdl: &scheduler::Handle, f: F) -> R
where
    F: FnOnce(Option<TempLocalContext<'_>>) -> R,
{
    panic!("STUB: not implemented");
}
fn push_from_remote(sched_hdl: &scheduler::Handle, entry_hdl: EntryHandle) {
    panic!("STUB: not implemented");
}
fn deadline_to_tick(sched_hdl: &scheduler::Handle, deadline: Instant) -> u64 {
    panic!("STUB: not implemented");
}
