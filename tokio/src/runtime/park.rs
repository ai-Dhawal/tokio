#![cfg_attr(not(feature = "full"), allow(dead_code))]
use crate::loom::sync::atomic::AtomicUsize;
use crate::loom::sync::{Arc, Condvar, Mutex};
use std::sync::atomic::Ordering::SeqCst;
use std::time::Duration;
#[derive(Debug)]
pub(crate) struct ParkThread {
    inner: Arc<Inner>,
}
/// Unblocks a thread that was blocked by `ParkThread`.
#[derive(Clone, Debug)]
pub(crate) struct UnparkThread {
    inner: Arc<Inner>,
}
#[derive(Debug)]
struct Inner {
    state: AtomicUsize,
    mutex: Mutex<()>,
    condvar: Condvar,
}
const EMPTY: usize = 0;
const PARKED: usize = 1;
const NOTIFIED: usize = 2;
tokio_thread_local! {
    static CURRENT_PARKER : ParkThread = ParkThread::new();
}
#[cfg(loom)]
tokio_thread_local! {
    pub (crate) static CURRENT_THREAD_PARK_COUNT : AtomicUsize = AtomicUsize::new(0);
}
impl ParkThread {
    pub(crate) fn new() -> Self {
        panic!("STUB: not implemented");
    }
    pub(crate) fn unpark(&self) -> UnparkThread {
        panic!("STUB: not implemented");
    }
    pub(crate) fn park(&mut self) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn park_timeout(&mut self, duration: Duration) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn shutdown(&mut self) {
        panic!("STUB: not implemented");
    }
}
impl Inner {
    fn park(&self) {
        panic!("STUB: not implemented");
    }
    /// Parks the current thread for at most `dur`.
    fn park_timeout(&self, dur: Duration) {
        panic!("STUB: not implemented");
    }
    fn unpark(&self) {
        panic!("STUB: not implemented");
    }
    fn shutdown(&self) {
        panic!("STUB: not implemented");
    }
}
impl Default for ParkThread {
    fn default() -> Self {
        panic!("STUB: not implemented");
    }
}
impl UnparkThread {
    pub(crate) fn unpark(&self) {
        panic!("STUB: not implemented");
    }
}
use crate::loom::thread::AccessError;
use std::future::Future;
use std::marker::PhantomData;
use std::rc::Rc;
use std::task::{RawWaker, RawWakerVTable, Waker};
/// Blocks the current thread using a condition variable.
#[derive(Debug)]
pub(crate) struct CachedParkThread {
    _anchor: PhantomData<Rc<()>>,
}
impl CachedParkThread {
    /// Creates a new `ParkThread` handle for the current thread.
    ///
    /// This type cannot be moved to other threads, so it should be created on
    /// the thread that the caller intends to park.
    pub(crate) fn new() -> CachedParkThread {
        panic!("STUB: not implemented");
    }
    pub(crate) fn waker(&self) -> Result<Waker, AccessError> {
        panic!("STUB: not implemented");
    }
    fn unpark(&self) -> Result<UnparkThread, AccessError> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn park(&mut self) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn park_timeout(&mut self, duration: Duration) {
        panic!("STUB: not implemented");
    }
    /// Gets a reference to the `ParkThread` handle for this thread.
    fn with_current<F, R>(&self, f: F) -> Result<R, AccessError>
    where
        F: FnOnce(&ParkThread) -> R,
    {
        panic!("STUB: not implemented");
    }
    pub(crate) fn block_on<F: Future>(
        &mut self,
        f: F,
    ) -> Result<F::Output, AccessError> {
        panic!("STUB: not implemented");
    }
}
impl UnparkThread {
    pub(crate) fn into_waker(self) -> Waker {
        panic!("STUB: not implemented");
    }
}
impl Inner {
    #[allow(clippy::wrong_self_convention)]
    fn into_raw(this: Arc<Inner>) -> *const () {
        panic!("STUB: not implemented");
    }
    /// # Safety
    ///
    /// The pointer must have been created by [`Self::into_raw`].
    unsafe fn from_raw(ptr: *const ()) -> Arc<Inner> {
        panic!("STUB: not implemented");
    }
}
unsafe fn unparker_to_raw_waker(unparker: Arc<Inner>) -> RawWaker {
    panic!("STUB: not implemented");
}
/// # Safety
///
/// The pointer must have been created by [`Inner::into_raw`].
unsafe fn clone(raw: *const ()) -> RawWaker {
    panic!("STUB: not implemented");
}
/// # Safety
///
/// The pointer must have been created by [`Inner::into_raw`].
unsafe fn drop_waker(raw: *const ()) {
    panic!("STUB: not implemented");
}
/// # Safety
///
/// The pointer must have been created by [`Inner::into_raw`].
unsafe fn wake(raw: *const ()) {
    panic!("STUB: not implemented");
}
/// # Safety
///
/// The pointer must have been created by [`Inner::into_raw`].
unsafe fn wake_by_ref(raw: *const ()) {
    panic!("STUB: not implemented");
}
#[cfg(loom)]
pub(crate) fn current_thread_park_count() -> usize {
    panic!("STUB: not implemented");
}
