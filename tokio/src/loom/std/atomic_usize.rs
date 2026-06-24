use std::cell::UnsafeCell;
use std::fmt;
use std::ops;
use std::panic;
/// `AtomicUsize` providing an additional `unsync_load` function.
pub(crate) struct AtomicUsize {
    inner: UnsafeCell<std::sync::atomic::AtomicUsize>,
}
unsafe impl Send for AtomicUsize {}
unsafe impl Sync for AtomicUsize {}
impl panic::RefUnwindSafe for AtomicUsize {}
impl panic::UnwindSafe for AtomicUsize {}
impl AtomicUsize {
    pub(crate) const fn new(val: usize) -> AtomicUsize {
        let inner = UnsafeCell::new(std::sync::atomic::AtomicUsize::new(val));
        AtomicUsize { inner }
    }
    /// Performs an unsynchronized load.
    ///
    /// # Safety
    ///
    /// All mutations must have happened before the unsynchronized load.
    /// Additionally, there must be no concurrent mutations.
    pub(crate) unsafe fn unsync_load(&self) -> usize {
        panic!("STUB: not implemented");
    }
    pub(crate) fn with_mut<R>(&mut self, f: impl FnOnce(&mut usize) -> R) -> R {
        panic!("STUB: not implemented");
    }
}
impl ops::Deref for AtomicUsize {
    type Target = std::sync::atomic::AtomicUsize;
    fn deref(&self) -> &Self::Target {
        panic!("STUB: not implemented");
    }
}
impl ops::DerefMut for AtomicUsize {
    fn deref_mut(&mut self) -> &mut Self::Target {
        panic!("STUB: not implemented");
    }
}
impl fmt::Debug for AtomicUsize {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
