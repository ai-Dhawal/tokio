use super::cancellation_queue::Sender;
use crate::loom::sync::{Arc, Mutex};
use crate::util::linked_list;
use std::marker::PhantomPinned;
use std::ptr::NonNull;
use std::task::{Context, Poll, Waker};
#[derive(Debug, Default)]
struct State {
    cancelled: bool,
    woken_up: bool,
    waker: Option<Waker>,
    cancel_tx: Option<Sender>,
}
#[derive(Debug)]
pub(crate) struct Entry {
    /// The intrusive pointer used by [`super::cancellation_queue`].
    cancel_pointers: linked_list::Pointers<Entry>,
    /// The intrusive pointer used by any of the following queues:
    ///
    /// - [`Wheel`]
    /// - [`RegistrationQueue`]
    /// - [`WakeQueue`]
    ///
    /// We can guarantee that this pointer is only used by one of the above
    /// at any given time. See below for the journey of this pointer.
    ///
    /// Initially, this pointer is used by the [`RegistrationQueue`].
    ///
    /// And then, before parking the resource driver,
    /// the scheduler removes the entry from the [`RegistrationQueue`]
    /// and insert it into the [`Wheel`].
    ///
    /// Finally, after parking the resource driver, the scheduler removes
    /// the entry from the [`Wheel`] and insert it into the [`WakeQueue`].
    ///
    /// [`RegistrationQueue`]: super::RegistrationQueue
    /// [`Wheel`]: super::Wheel
    /// [`WakeQueue`]: super::WakeQueue
    extra_pointers: linked_list::Pointers<Entry>,
    /// The tick when this entry is scheduled to expire.
    deadline: u64,
    state: Mutex<State>,
    /// Make the type `!Unpin` to prevent LLVM from emitting
    /// the `noalias` attribute for mutable references.
    ///
    /// See <https://github.com/rust-lang/rust/pull/82834>.
    _pin: PhantomPinned,
}
unsafe impl linked_list::Link for Entry {
    type Handle = Handle;
    type Target = Entry;
    fn as_raw(hdl: &Self::Handle) -> NonNull<Self::Target> {
        panic!("STUB: not implemented");
    }
    unsafe fn from_raw(ptr: NonNull<Self::Target>) -> Self::Handle {
        panic!("STUB: not implemented");
    }
    unsafe fn pointers(
        target: NonNull<Self::Target>,
    ) -> NonNull<linked_list::Pointers<Self::Target>> {
        panic!("STUB: not implemented");
    }
}
/// An ZST to allow [`super::registration_queue`] to utilize the [`Entry::extra_pointers`]
/// by impl [`linked_list::Link`] as we cannot impl it on [`Entry`]
/// directly due to the conflicting implementations.
///
/// This type should never be constructed.
pub(super) struct RegistrationQueueEntry;
unsafe impl linked_list::Link for RegistrationQueueEntry {
    type Handle = Handle;
    type Target = Entry;
    fn as_raw(hdl: &Self::Handle) -> NonNull<Self::Target> {
        panic!("STUB: not implemented");
    }
    unsafe fn from_raw(ptr: NonNull<Self::Target>) -> Self::Handle {
        panic!("STUB: not implemented");
    }
    unsafe fn pointers(
        target: NonNull<Self::Target>,
    ) -> NonNull<linked_list::Pointers<Self::Target>> {
        panic!("STUB: not implemented");
    }
}
/// An ZST to allow [`super::cancellation_queue`] to utilize the [`Entry::cancel_pointers`]
/// by impl [`linked_list::Link`] as we cannot impl it on [`Entry`]
/// directly due to the conflicting implementations.
///
/// This type should never be constructed.
pub(super) struct CancellationQueueEntry;
unsafe impl linked_list::Link for CancellationQueueEntry {
    type Handle = Handle;
    type Target = Entry;
    fn as_raw(hdl: &Self::Handle) -> NonNull<Self::Target> {
        panic!("STUB: not implemented");
    }
    unsafe fn from_raw(ptr: NonNull<Self::Target>) -> Self::Handle {
        panic!("STUB: not implemented");
    }
    unsafe fn pointers(
        target: NonNull<Self::Target>,
    ) -> NonNull<linked_list::Pointers<Self::Target>> {
        panic!("STUB: not implemented");
    }
}
/// An ZST to allow [`super::WakeQueue`] to utilize the [`Entry::extra_pointers`]
/// by impl [`linked_list::Link`] as we cannot impl it on [`Entry`]
/// directly due to the conflicting implementations.
///
/// This type should never be constructed.
pub(super) struct WakeQueueEntry;
unsafe impl linked_list::Link for WakeQueueEntry {
    type Handle = Handle;
    type Target = Entry;
    fn as_raw(hdl: &Self::Handle) -> NonNull<Self::Target> {
        panic!("STUB: not implemented");
    }
    unsafe fn from_raw(ptr: NonNull<Self::Target>) -> Self::Handle {
        panic!("STUB: not implemented");
    }
    unsafe fn pointers(
        target: NonNull<Self::Target>,
    ) -> NonNull<linked_list::Pointers<Self::Target>> {
        panic!("STUB: not implemented");
    }
}
#[derive(Debug, Clone)]
pub(crate) struct Handle {
    pub(crate) entry: Arc<Entry>,
}
impl From<&Handle> for NonNull<Entry> {
    fn from(hdl: &Handle) -> Self {
        panic!("STUB: not implemented");
    }
}
impl Handle {
    pub(crate) fn new(deadline: u64) -> Self {
        panic!("STUB: not implemented");
    }
    /// Wake the entry if it is already in the pending queue of the timer wheel.
    pub(crate) fn wake(&self) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn register_cancel_tx(&self, cancel_tx: Sender) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn poll(&self, cx: &mut Context<'_>) -> Poll<()> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn cancel(&self) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn deadline(&self) -> u64 {
        panic!("STUB: not implemented");
    }
    pub(crate) fn is_woken_up(&self) -> bool {
        panic!("STUB: not implemented");
    }
    pub(crate) fn is_cancelled(&self) -> bool {
        panic!("STUB: not implemented");
    }
    #[cfg(test)]
    /// Only used for unit tests.
    pub(crate) fn inner_strong_count(&self) -> usize {
        Arc::strong_count(&self.entry)
    }
}
