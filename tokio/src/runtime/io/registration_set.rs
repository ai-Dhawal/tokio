use crate::loom::sync::atomic::AtomicUsize;
use crate::runtime::io::ScheduledIo;
use crate::util::linked_list::{self, LinkedList};
use std::io;
use std::ptr::NonNull;
use std::sync::atomic::Ordering::{Acquire, Release};
use std::sync::Arc;
const NOTIFY_AFTER: usize = 16;
pub(super) struct RegistrationSet {
    num_pending_release: AtomicUsize,
}
pub(super) struct Synced {
    is_shutdown: bool,
    registrations: LinkedList<Arc<ScheduledIo>>,
    pending_release: Vec<Arc<ScheduledIo>>,
}
impl RegistrationSet {
    pub(super) fn new() -> (RegistrationSet, Synced) {
        panic!("STUB: not implemented");
    }
    pub(super) fn is_shutdown(&self, synced: &Synced) -> bool {
        panic!("STUB: not implemented");
    }
    /// Returns `true` if there are registrations that need to be released
    pub(super) fn needs_release(&self) -> bool {
        panic!("STUB: not implemented");
    }
    pub(super) fn allocate(&self, synced: &mut Synced) -> io::Result<Arc<ScheduledIo>> {
        panic!("STUB: not implemented");
    }
    pub(super) fn deregister(
        &self,
        synced: &mut Synced,
        registration: &Arc<ScheduledIo>,
    ) -> bool {
        panic!("STUB: not implemented");
    }
    pub(super) fn shutdown(&self, synced: &mut Synced) -> Vec<Arc<ScheduledIo>> {
        panic!("STUB: not implemented");
    }
    pub(super) fn release(&self, synced: &mut Synced) {
        panic!("STUB: not implemented");
    }
    pub(super) unsafe fn remove(&self, synced: &mut Synced, io: &Arc<ScheduledIo>) {
        panic!("STUB: not implemented");
    }
}
unsafe impl linked_list::Link for Arc<ScheduledIo> {
    type Handle = Arc<ScheduledIo>;
    type Target = ScheduledIo;
    fn as_raw(handle: &Self::Handle) -> NonNull<ScheduledIo> {
        panic!("STUB: not implemented");
    }
    unsafe fn from_raw(ptr: NonNull<Self::Target>) -> Arc<ScheduledIo> {
        panic!("STUB: not implemented");
    }
    unsafe fn pointers(
        target: NonNull<Self::Target>,
    ) -> NonNull<linked_list::Pointers<ScheduledIo>> {
        panic!("STUB: not implemented");
    }
}
