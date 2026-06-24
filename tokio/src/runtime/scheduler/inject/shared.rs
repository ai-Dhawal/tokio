use super::{Pop, Synced};
use crate::loom::sync::atomic::AtomicUsize;
use crate::runtime::task;
use std::marker::PhantomData;
use std::sync::atomic::Ordering::{Acquire, Release};
pub(crate) struct Shared<T: 'static> {
    /// Number of pending tasks in the queue. This helps prevent unnecessary
    /// locking in the hot path.
    pub(super) len: AtomicUsize,
    _p: PhantomData<T>,
}
unsafe impl<T> Send for Shared<T> {}
unsafe impl<T> Sync for Shared<T> {}
impl<T: 'static> Shared<T> {
    pub(crate) fn new() -> (Shared<T>, Synced) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn is_empty(&self) -> bool {
        panic!("STUB: not implemented");
    }
    #[cfg(any(feature = "taskdump", feature = "rt-multi-thread"))]
    pub(crate) fn is_closed(&self, synced: &Synced) -> bool {
        panic!("STUB: not implemented");
    }
    /// Closes the injection queue, returns `true` if the queue is open when the
    /// transition is made.
    pub(crate) fn close(&self, synced: &mut Synced) -> bool {
        panic!("STUB: not implemented");
    }
    pub(crate) fn len(&self) -> usize {
        panic!("STUB: not implemented");
    }
    /// Pushes a value into the queue.
    ///
    /// This does nothing if the queue is closed.
    ///
    /// # Safety
    ///
    /// Must be called with the same `Synced` instance returned by `Inject::new`
    pub(crate) unsafe fn push(&self, synced: &mut Synced, task: task::Notified<T>) {
        panic!("STUB: not implemented");
    }
    /// Pop a value from the queue.
    ///
    /// # Safety
    ///
    /// Must be called with the same `Synced` instance returned by `Inject::new`
    pub(crate) unsafe fn pop(&self, synced: &mut Synced) -> Option<task::Notified<T>> {
        panic!("STUB: not implemented");
    }
    /// Pop `n` values from the queue
    ///
    /// # Safety
    ///
    /// Must be called with the same `Synced` instance returned by `Inject::new`
    pub(crate) unsafe fn pop_n<'a>(
        &'a self,
        synced: &'a mut Synced,
        n: usize,
    ) -> Pop<'a, T> {
        panic!("STUB: not implemented");
    }
}
