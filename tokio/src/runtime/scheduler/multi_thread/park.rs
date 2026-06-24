//! Parks the runtime.
//!
//! A combination of the various resource driver park handles.
use crate::loom::sync::atomic::AtomicUsize;
use crate::loom::sync::{Arc, Condvar, Mutex};
use crate::runtime::driver::{self, Driver};
use crate::util::TryLock;
use std::sync::atomic::Ordering::SeqCst;
use std::time::{Duration, Instant};
#[cfg(loom)]
use crate::runtime::park::CURRENT_THREAD_PARK_COUNT;
pub(crate) struct Parker {
    inner: Arc<Inner>,
}
pub(crate) struct Unparker {
    inner: Arc<Inner>,
}
/// Represents how a worker thread was parked
#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) enum HadDriver {
    Yes,
    No,
}
struct Inner {
    /// Avoids entering the park if possible
    state: AtomicUsize,
    /// Used to coordinate access to the driver / `condvar`
    mutex: Mutex<()>,
    /// `Condvar` to block on if the driver is unavailable.
    condvar: Condvar,
    /// Resource (I/O, time, ...) driver
    shared: Arc<Shared>,
}
const EMPTY: usize = 0;
const PARKED_CONDVAR: usize = 1;
const PARKED_DRIVER: usize = 2;
const NOTIFIED: usize = 3;
/// Shared across multiple Parker handles
struct Shared {
    /// Shared driver. Only one thread at a time can use this
    driver: TryLock<Driver>,
}
impl Parker {
    pub(crate) fn new(driver: Driver) -> Parker {
        panic!("STUB: not implemented");
    }
    pub(crate) fn unpark(&self) -> Unparker {
        panic!("STUB: not implemented");
    }
    pub(crate) fn park(&mut self, handle: &driver::Handle) -> HadDriver {
        panic!("STUB: not implemented");
    }
    /// Parks the current thread for up to `duration`.
    ///
    /// This function tries to acquire the driver lock. If it succeeds, it
    /// parks using the driver. Otherwise, it fails back to using a condvar,
    /// unless the duration is zero, in which case it returns immediately.
    pub(crate) fn park_timeout(
        &mut self,
        handle: &driver::Handle,
        duration: Duration,
    ) -> HadDriver {
        panic!("STUB: not implemented");
    }
    pub(crate) fn shutdown(&mut self, handle: &driver::Handle) {
        panic!("STUB: not implemented");
    }
}
impl Clone for Parker {
    fn clone(&self) -> Parker {
        panic!("STUB: not implemented");
    }
}
impl Unparker {
    pub(crate) fn unpark(&self, driver: &driver::Handle) {
        panic!("STUB: not implemented");
    }
}
impl Inner {
    /// Parks the current thread for at most `dur`.
    fn park(&self, handle: &driver::Handle) -> HadDriver {
        panic!("STUB: not implemented");
    }
    /// Parks the current thread using a condvar for up to `duration`.
    ///
    /// If `duration` is `None`, parks indefinitely until notified.
    ///
    /// # Panics
    ///
    /// Panics if `duration` is `Some` and the duration is zero.
    fn park_condvar(&self, duration: Option<Duration>) {
        panic!("STUB: not implemented");
    }
    fn park_driver(
        &self,
        driver: &mut Driver,
        handle: &driver::Handle,
        duration: Option<Duration>,
    ) -> HadDriver {
        panic!("STUB: not implemented");
    }
    fn unpark(&self, driver: &driver::Handle) {
        panic!("STUB: not implemented");
    }
    fn unpark_condvar(&self) {
        panic!("STUB: not implemented");
    }
    fn shutdown(&self, handle: &driver::Handle) {
        panic!("STUB: not implemented");
    }
}
