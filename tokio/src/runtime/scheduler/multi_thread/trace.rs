use crate::loom::sync::atomic::{AtomicBool, Ordering};
use crate::loom::sync::{Barrier, Mutex};
use crate::runtime::dump::Dump;
use crate::runtime::scheduler::multi_thread::Handle;
use crate::sync::notify::Notify;
/// Tracing status of the worker.
pub(super) struct TraceStatus {
    pub(super) trace_requested: AtomicBool,
    pub(super) trace_start: Barrier,
    pub(super) trace_end: Barrier,
    pub(super) result_ready: Notify,
    pub(super) trace_result: Mutex<Option<Dump>>,
}
impl TraceStatus {
    pub(super) fn new(remotes_len: usize) -> Self {
        panic!("STUB: not implemented");
    }
    pub(super) fn trace_requested(&self) -> bool {
        panic!("STUB: not implemented");
    }
    pub(super) async fn start_trace_request(&self, handle: &Handle) {
        panic!("STUB: not implemented");
    }
    pub(super) fn stash_result(&self, dump: Dump) {
        panic!("STUB: not implemented");
    }
    pub(super) fn take_result(&self) -> Option<Dump> {
        panic!("STUB: not implemented");
    }
    pub(super) async fn end_trace_request(&self, handle: &Handle) {
        panic!("STUB: not implemented");
    }
}
