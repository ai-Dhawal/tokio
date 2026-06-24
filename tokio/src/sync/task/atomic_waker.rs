#![cfg_attr(any(loom, not(feature = "sync")), allow(dead_code, unreachable_pub))]
use crate::loom::cell::UnsafeCell;
use crate::loom::hint;
use crate::loom::sync::atomic::AtomicUsize;
use std::fmt;
use std::panic::{resume_unwind, AssertUnwindSafe, RefUnwindSafe, UnwindSafe};
use std::sync::atomic::Ordering::{AcqRel, Acquire, Release};
use std::task::Waker;
/// A synchronization primitive for task waking.
///
/// `AtomicWaker` will coordinate concurrent wakes with the consumer
/// potentially "waking" the underlying task. This is useful in scenarios
/// where a computation completes in another thread and wants to wake the
/// consumer, but the consumer is in the process of being migrated to a new
/// logical task.
///
/// Consumers should call `register` before checking the result of a computation
/// and producers should call `wake` after producing the computation (this
/// differs from the usual `thread::park` pattern). It is also permitted for
/// `wake` to be called **before** `register`. This results in a no-op.
///
/// A single `AtomicWaker` may be reused for any number of calls to `register` or
/// `wake`.
pub(crate) struct AtomicWaker {
    state: AtomicUsize,
    waker: UnsafeCell<Option<Waker>>,
}
impl RefUnwindSafe for AtomicWaker {}
impl UnwindSafe for AtomicWaker {}
/// Idle state.
const WAITING: usize = 0;
/// A new waker value is being registered with the `AtomicWaker` cell.
const REGISTERING: usize = 0b01;
/// The task currently registered with the `AtomicWaker` cell is being woken.
const WAKING: usize = 0b10;
impl AtomicWaker {
    /// Create an `AtomicWaker`
    pub(crate) fn new() -> AtomicWaker {
        panic!("STUB: not implemented");
    }
    /// Registers the provided waker to be notified on calls to `wake`.
    ///
    /// The new waker will take place of any previous wakers that were registered
    /// by previous calls to `register`. Any calls to `wake` that happen after
    /// a call to `register` (as defined by the memory ordering rules), will
    /// wake the `register` caller's task.
    ///
    /// It is safe to call `register` with multiple other threads concurrently
    /// calling `wake`. This will result in the `register` caller's current
    /// task being woken once.
    ///
    /// This function is safe to call concurrently, but this is generally a bad
    /// idea. Concurrent calls to `register` will attempt to register different
    /// tasks to be woken. One of the callers will win and have its task set,
    /// but there is no guarantee as to which caller will succeed.
    pub(crate) fn register_by_ref(&self, waker: &Waker) {
        panic!("STUB: not implemented");
    }
    fn do_register<W>(&self, waker: W)
    where
        W: WakerRef,
    {
        panic!("STUB: not implemented");
    }
    /// Wakes the task that last called `register`.
    ///
    /// If `register` has not been called yet, then this does nothing.
    pub(crate) fn wake(&self) {
        panic!("STUB: not implemented");
    }
    /// Attempts to take the `Waker` value out of the `AtomicWaker` with the
    /// intention that the caller will wake the task later.
    pub(crate) fn take_waker(&self) -> Option<Waker> {
        panic!("STUB: not implemented");
    }
}
impl Default for AtomicWaker {
    fn default() -> Self {
        panic!("STUB: not implemented");
    }
}
impl fmt::Debug for AtomicWaker {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
unsafe impl Send for AtomicWaker {}
unsafe impl Sync for AtomicWaker {}
trait WakerRef {
    fn wake(self);
    fn into_waker(self) -> Waker;
}
impl WakerRef for Waker {
    fn wake(self) {
        panic!("STUB: not implemented");
    }
    fn into_waker(self) -> Waker {
        panic!("STUB: not implemented");
    }
}
impl WakerRef for &Waker {
    fn wake(self) {
        panic!("STUB: not implemented");
    }
    fn into_waker(self) -> Waker {
        panic!("STUB: not implemented");
    }
}
