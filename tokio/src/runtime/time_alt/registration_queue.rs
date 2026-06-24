use super::{EntryHandle, RegistrationQueueEntry};
use crate::util::linked_list::LinkedList;
/// A queue of entries that need to be registered in the timer wheel.
#[derive(Debug)]
pub(crate) struct RegistrationQueue {
    list: LinkedList<RegistrationQueueEntry>,
}
impl Drop for RegistrationQueue {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
impl RegistrationQueue {
    pub(crate) fn new() -> Self {
        panic!("STUB: not implemented");
    }
    /// # Safety
    ///
    /// Behavior is undefined if any of the following conditions are violated:
    ///
    /// - `Entry::extra_pointers` of `hdl` must not being used.
    pub(crate) unsafe fn push_front(&mut self, hdl: EntryHandle) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn pop_front(&mut self) -> Option<EntryHandle> {
        panic!("STUB: not implemented");
    }
}
#[cfg(test)]
mod tests;
