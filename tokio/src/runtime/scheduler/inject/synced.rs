#![cfg_attr(
    any(not(all(tokio_unstable, feature = "full")), target_family = "wasm"),
    allow(dead_code)
)]
use crate::runtime::task;
pub(crate) struct Synced {
    /// True if the queue is closed.
    pub(super) is_closed: bool,
    /// Linked-list head.
    pub(super) head: Option<task::RawTask>,
    /// Linked-list tail.
    pub(super) tail: Option<task::RawTask>,
}
unsafe impl Send for Synced {}
unsafe impl Sync for Synced {}
impl Synced {
    pub(super) fn pop<T: 'static>(&mut self) -> Option<task::Notified<T>> {
        panic!("STUB: not implemented");
    }
}
