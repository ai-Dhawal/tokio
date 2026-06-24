use super::{Core, Handle, Shared};
use crate::loom::sync::Arc;
use crate::runtime::scheduler::multi_thread::Stats;
use crate::runtime::task::trace::trace_multi_thread;
use crate::runtime::{dump, WorkerMetrics};
use std::time::Duration;
impl Handle {
    pub(super) fn trace_core(&self, mut core: Box<Core>) -> Box<Core> {
        panic!("STUB: not implemented");
    }
}
impl Shared {
    /// Steal all tasks from remotes into a single local queue.
    pub(super) fn steal_all(&self) -> super::queue::Local<Arc<Handle>> {
        panic!("STUB: not implemented");
    }
}
