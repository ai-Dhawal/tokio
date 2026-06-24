use crate::io::interest::Interest;
use crate::io::ready::Ready;
use crate::loom::sync::atomic::AtomicUsize;
use crate::loom::sync::Mutex;
use crate::runtime::io::{Direction, ReadyEvent, Tick};
use crate::util::bit;
use crate::util::linked_list::{self, LinkedList};
use crate::util::WakeList;
use std::cell::UnsafeCell;
use std::future::Future;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr::NonNull;
use std::sync::atomic::Ordering::{AcqRel, Acquire};
use std::task::{Context, Poll, Waker};
/// Stored in the I/O driver resource slab.
#[derive(Debug)]
#[cfg_attr(
    any(target_arch = "x86_64", target_arch = "aarch64", target_arch = "powerpc64"),
    repr(align(128))
)]
#[cfg_attr(
    any(
        target_arch = "arm",
        target_arch = "mips",
        target_arch = "mips64",
        target_arch = "sparc",
        target_arch = "hexagon",
    ),
    repr(align(32))
)]
#[cfg_attr(target_arch = "m68k", repr(align(16)))]
#[cfg_attr(target_arch = "s390x", repr(align(256)))]
#[cfg_attr(
    not(
        any(
            target_arch = "x86_64",
            target_arch = "aarch64",
            target_arch = "powerpc64",
            target_arch = "arm",
            target_arch = "mips",
            target_arch = "mips64",
            target_arch = "sparc",
            target_arch = "hexagon",
            target_arch = "m68k",
            target_arch = "s390x",
        )
    ),
    repr(align(64))
)]
pub(crate) struct ScheduledIo {
    pub(super) linked_list_pointers: UnsafeCell<linked_list::Pointers<Self>>,
    /// Packs the resource's readiness and I/O driver latest tick.
    readiness: AtomicUsize,
    waiters: Mutex<Waiters>,
}
#[derive(Debug, Default)]
struct Waiters {
    /// List of all current waiters.
    list: LinkedList<Waiter>,
    /// Waker used for `AsyncRead`.
    reader: Option<Waker>,
    /// Waker used for `AsyncWrite`.
    writer: Option<Waker>,
}
#[derive(Debug)]
struct Waiter {
    pointers: linked_list::Pointers<Waiter>,
    /// The waker for this task.
    waker: Option<Waker>,
    /// The interest this waiter is waiting on.
    interest: Interest,
    is_ready: bool,
    /// Should never be `Unpin`.
    _p: PhantomPinned,
}
generate_addr_of_methods! {
    impl <> Waiter { unsafe fn addr_of_pointers(self : NonNull < Self >) -> NonNull <
    linked_list::Pointers < Waiter >> { & self.pointers } }
}
/// Future returned by `readiness()`.
struct Readiness<'a> {
    scheduled_io: &'a ScheduledIo,
    state: State,
    /// Entry in the waiter `LinkedList`.
    waiter: UnsafeCell<Waiter>,
}
enum State {
    Init,
    Waiting,
    Done,
}
const READINESS: bit::Pack = bit::Pack::least_significant(16);
const TICK: bit::Pack = READINESS.then(15);
const SHUTDOWN: bit::Pack = TICK.then(1);
impl Default for ScheduledIo {
    fn default() -> ScheduledIo {
        panic!("STUB: not implemented");
    }
}
impl ScheduledIo {
    pub(crate) fn token(&self) -> mio::Token {
        panic!("STUB: not implemented");
    }
    /// Invoked when the IO driver is shut down; forces this `ScheduledIo` into a
    /// permanently shutdown state.
    pub(super) fn shutdown(&self) {
        panic!("STUB: not implemented");
    }
    /// Sets the readiness on this `ScheduledIo` by invoking the given closure on
    /// the current value, returning the previous readiness value.
    ///
    /// # Arguments
    /// - `tick`: whether setting the tick or trying to clear readiness for a
    ///   specific tick.
    /// - `f`: a closure returning a new readiness value given the previous
    ///   readiness.
    pub(super) fn set_readiness(&self, tick_op: Tick, f: impl Fn(Ready) -> Ready) {
        panic!("STUB: not implemented");
    }
    /// Notifies all pending waiters that have registered interest in `ready`.
    ///
    /// There may be many waiters to notify. Waking the pending task **must** be
    /// done from outside of the lock otherwise there is a potential for a
    /// deadlock.
    ///
    /// A stack array of wakers is created and filled with wakers to notify, the
    /// lock is released, and the wakers are notified. Because there may be more
    /// than 32 wakers to notify, if the stack array fills up, the lock is
    /// released, the array is cleared, and the iteration continues.
    pub(super) fn wake(&self, ready: Ready) {
        panic!("STUB: not implemented");
    }
    pub(super) fn ready_event(&self, interest: Interest) -> ReadyEvent {
        panic!("STUB: not implemented");
    }
    /// Polls for readiness events in a given direction.
    ///
    /// These are to support `AsyncRead` and `AsyncWrite` polling methods,
    /// which cannot use the `async fn` version. This uses reserved reader
    /// and writer slots.
    pub(super) fn poll_readiness(
        &self,
        cx: &mut Context<'_>,
        direction: Direction,
    ) -> Poll<ReadyEvent> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn clear_readiness(&self, event: ReadyEvent) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn clear_wakers(&self) {
        panic!("STUB: not implemented");
    }
}
impl Drop for ScheduledIo {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
unsafe impl Send for ScheduledIo {}
unsafe impl Sync for ScheduledIo {}
impl ScheduledIo {
    /// An async version of `poll_readiness` which uses a linked list of wakers.
    pub(crate) async fn readiness(&self, interest: Interest) -> ReadyEvent {
        panic!("STUB: not implemented");
    }
    fn readiness_fut(&self, interest: Interest) -> Readiness<'_> {
        panic!("STUB: not implemented");
    }
}
unsafe impl linked_list::Link for Waiter {
    type Handle = NonNull<Waiter>;
    type Target = Waiter;
    fn as_raw(handle: &NonNull<Waiter>) -> NonNull<Waiter> {
        panic!("STUB: not implemented");
    }
    unsafe fn from_raw(ptr: NonNull<Waiter>) -> NonNull<Waiter> {
        panic!("STUB: not implemented");
    }
    unsafe fn pointers(
        target: NonNull<Waiter>,
    ) -> NonNull<linked_list::Pointers<Waiter>> {
        panic!("STUB: not implemented");
    }
}
impl Future for Readiness<'_> {
    type Output = ReadyEvent;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        panic!("STUB: not implemented");
    }
}
impl Drop for Readiness<'_> {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
unsafe impl Send for Readiness<'_> {}
unsafe impl Sync for Readiness<'_> {}
