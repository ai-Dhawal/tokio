use super::{CancellationQueueEntry, EntryHandle};
use crate::loom::sync::{Arc, Mutex};
use crate::util::linked_list::LinkedList;
#[derive(Debug, Default)]
struct Inner {
    list: LinkedList<CancellationQueueEntry>,
}
impl Drop for Inner {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
impl Inner {
    fn new() -> Self {
        panic!("STUB: not implemented");
    }
    /// # Safety
    ///
    /// Behavior is undefined if any of the following conditions are violated:
    ///
    /// - `hdl` must not in any [`super::cancellation_queue`], and also mus not in any [`super::WakeQueue`].
    unsafe fn push_front(&mut self, hdl: EntryHandle) {
        panic!("STUB: not implemented");
    }
    fn into_iter(self) -> impl Iterator<Item = EntryHandle> {
        panic!("STUB: not implemented");
        #[allow(unreachable_code)] std::iter::empty::<EntryHandle>()
    }
}
#[derive(Debug, Clone)]
pub(crate) struct Sender {
    inner: Arc<Mutex<Inner>>,
}
impl Sender {
    /// # Safety
    ///
    /// Behavior is undefined if any of the following conditions are violated:
    ///
    /// - `hdl` must not in any cancellation queue.
    pub(crate) unsafe fn send(&self, hdl: EntryHandle) {
        panic!("STUB: not implemented");
    }
}
#[derive(Debug)]
pub(crate) struct Receiver {
    inner: Arc<Mutex<Inner>>,
}
impl Receiver {
    pub(crate) fn recv_all(&mut self) -> impl Iterator<Item = EntryHandle> {
        panic!("STUB: not implemented");
        #[allow(unreachable_code)] std::iter::empty::<EntryHandle>()
    }
}
pub(crate) fn new() -> (Sender, Receiver) {
    panic!("STUB: not implemented");
}
#[cfg(test)]
mod tests;
