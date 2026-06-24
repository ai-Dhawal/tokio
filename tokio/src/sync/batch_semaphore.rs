#![cfg_attr(not(feature = "sync"), allow(unreachable_pub, dead_code))]
//! # Implementation Details.
//!
//! The semaphore is implemented using an intrusive linked list of waiters. An
//! atomic counter tracks the number of available permits. If the semaphore does
//! not contain the required number of permits, the task attempting to acquire
//! permits places its waker at the end of a queue. When new permits are made
//! available (such as by releasing an initial acquisition), they are assigned
//! to the task at the front of the queue, waking that task if its requested
//! number of permits is met.
//!
//! Because waiters are enqueued at the back of the linked list and dequeued
//! from the front, the semaphore is fair. Tasks trying to acquire large numbers
//! of permits at a time will always be woken eventually, even if many other
//! tasks are acquiring smaller numbers of permits. This means that in a
//! use-case like tokio's read-write lock, writers will not be starved by
//! readers.
use crate::loom::cell::UnsafeCell;
use crate::loom::sync::atomic::AtomicUsize;
use crate::loom::sync::{Mutex, MutexGuard};
use crate::util::linked_list::{self, LinkedList};
#[cfg(all(tokio_unstable, feature = "tracing"))]
use crate::util::trace;
use crate::util::WakeList;
use std::future::Future;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr::NonNull;
use std::sync::atomic::Ordering::*;
use std::task::{ready, Context, Poll, Waker};
use std::{cmp, fmt};
/// An asynchronous counting semaphore which permits waiting on multiple permits at once.
pub(crate) struct Semaphore {
    waiters: Mutex<Waitlist>,
    /// The current number of available permits in the semaphore.
    permits: AtomicUsize,
    #[cfg(all(tokio_unstable, feature = "tracing"))]
    resource_span: tracing::Span,
}
struct Waitlist {
    queue: LinkedList<Waiter>,
    closed: bool,
}
/// Error returned from the [`Semaphore::try_acquire`] function.
///
/// [`Semaphore::try_acquire`]: crate::sync::Semaphore::try_acquire
#[derive(Debug, PartialEq, Eq)]
pub enum TryAcquireError {
    /// The semaphore has been [closed] and cannot issue new permits.
    ///
    /// [closed]: crate::sync::Semaphore::close
    Closed,
    /// The semaphore has no available permits.
    NoPermits,
}
/// Error returned from the [`Semaphore::acquire`] function.
///
/// An `acquire` operation can only fail if the semaphore has been
/// [closed].
///
/// [closed]: crate::sync::Semaphore::close
/// [`Semaphore::acquire`]: crate::sync::Semaphore::acquire
#[derive(Debug)]
pub struct AcquireError(());
pub(crate) struct Acquire<'a> {
    node: Waiter,
    semaphore: &'a Semaphore,
    num_permits: usize,
    queued: bool,
}
/// An entry in the wait queue.
struct Waiter {
    /// The current state of the waiter.
    ///
    /// This is either the number of remaining permits required by
    /// the waiter, or a flag indicating that the waiter is not yet queued.
    state: AtomicUsize,
    /// The waker to notify the task awaiting permits.
    ///
    /// # Safety
    ///
    /// This may only be accessed while the wait queue is locked.
    waker: UnsafeCell<Option<Waker>>,
    /// Intrusive linked-list pointers.
    ///
    /// # Safety
    ///
    /// This may only be accessed while the wait queue is locked.
    ///
    /// TODO: Ideally, we would be able to use loom to enforce that
    /// this isn't accessed concurrently. However, it is difficult to
    /// use a `UnsafeCell` here, since the `Link` trait requires _returning_
    /// references to `Pointers`, and `UnsafeCell` requires that checked access
    /// take place inside a closure. We should consider changing `Pointers` to
    /// use `UnsafeCell` internally.
    pointers: linked_list::Pointers<Waiter>,
    #[cfg(all(tokio_unstable, feature = "tracing"))]
    ctx: trace::AsyncOpTracingCtx,
    /// Should not be `Unpin`.
    _p: PhantomPinned,
}
generate_addr_of_methods! {
    impl <> Waiter { unsafe fn addr_of_pointers(self : NonNull < Self >) -> NonNull <
    linked_list::Pointers < Waiter >> { & self.pointers } }
}
impl Semaphore {
    /// The maximum number of permits which a semaphore can hold.
    ///
    /// Note that this reserves three bits of flags in the permit counter, but
    /// we only actually use one of them. However, the previous semaphore
    /// implementation used three bits, so we will continue to reserve them to
    /// avoid a breaking change if additional flags need to be added in the
    /// future.
    pub(crate) const MAX_PERMITS: usize = usize::MAX >> 3;
    const CLOSED: usize = 1;
    const PERMIT_SHIFT: usize = 1;
    /// Creates a new semaphore with the initial number of permits
    ///
    /// Maximum number of permits on 32-bit platforms is `1<<29`.
    pub(crate) fn new(permits: usize) -> Self {
        panic!("STUB: not implemented");
    }
    /// Creates a new semaphore with the initial number of permits.
    ///
    /// Maximum number of permits on 32-bit platforms is `1<<29`.
    #[cfg(not(all(loom, test)))]
    pub(crate) const fn const_new(permits: usize) -> Self {
        assert!(permits <= Self::MAX_PERMITS);
        Self {
            permits: AtomicUsize::new(permits << Self::PERMIT_SHIFT),
            waiters: Mutex::const_new(Waitlist {
                queue: LinkedList::new(),
                closed: false,
            }),
            #[cfg(all(tokio_unstable, feature = "tracing"))]
            resource_span: tracing::Span::none(),
        }
    }
    /// Creates a new closed semaphore with 0 permits.
    pub(crate) fn new_closed() -> Self {
        panic!("STUB: not implemented");
    }
    /// Creates a new closed semaphore with 0 permits.
    #[cfg(not(all(loom, test)))]
    pub(crate) const fn const_new_closed() -> Self {
        Self {
            permits: AtomicUsize::new(Self::CLOSED),
            waiters: Mutex::const_new(Waitlist {
                queue: LinkedList::new(),
                closed: true,
            }),
            #[cfg(all(tokio_unstable, feature = "tracing"))]
            resource_span: tracing::Span::none(),
        }
    }
    /// Returns the current number of available permits.
    pub(crate) fn available_permits(&self) -> usize {
        panic!("STUB: not implemented");
    }
    /// Adds `added` new permits to the semaphore.
    ///
    /// The maximum number of permits is `usize::MAX >> 3`, and this function will panic if the limit is exceeded.
    pub(crate) fn release(&self, added: usize) {
        panic!("STUB: not implemented");
    }
    /// Closes the semaphore. This prevents the semaphore from issuing new
    /// permits and notifies all pending waiters.
    pub(crate) fn close(&self) {
        panic!("STUB: not implemented");
    }
    /// Returns true if the semaphore is closed.
    pub(crate) fn is_closed(&self) -> bool {
        panic!("STUB: not implemented");
    }
    pub(crate) fn try_acquire(&self, num_permits: usize) -> Result<(), TryAcquireError> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn acquire(&self, num_permits: usize) -> Acquire<'_> {
        panic!("STUB: not implemented");
    }
    /// Release `rem` permits to the semaphore's wait list, starting from the
    /// end of the queue.
    ///
    /// If `rem` exceeds the number of permits needed by the wait list, the
    /// remainder are assigned back to the semaphore.
    fn add_permits_locked(&self, mut rem: usize, waiters: MutexGuard<'_, Waitlist>) {
        panic!("STUB: not implemented");
    }
    /// Decrease a semaphore's permits by a maximum of `n`.
    ///
    /// If there are insufficient permits and it's not possible to reduce by `n`,
    /// return the number of permits that were actually reduced.
    pub(crate) fn forget_permits(&self, n: usize) -> usize {
        panic!("STUB: not implemented");
    }
    fn poll_acquire(
        &self,
        cx: &mut Context<'_>,
        num_permits: usize,
        node: Pin<&mut Waiter>,
        queued: bool,
    ) -> Poll<Result<(), AcquireError>> {
        panic!("STUB: not implemented");
    }
}
impl fmt::Debug for Semaphore {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl Waiter {
    fn new(
        num_permits: usize,
        #[cfg(all(tokio_unstable, feature = "tracing"))]
        ctx: trace::AsyncOpTracingCtx,
    ) -> Self {
        panic!("STUB: not implemented");
    }
    /// Assign permits to the waiter.
    ///
    /// Returns `true` if the waiter should be removed from the queue
    fn assign_permits(&self, n: &mut usize) -> bool {
        panic!("STUB: not implemented");
    }
}
impl Future for Acquire<'_> {
    type Output = Result<(), AcquireError>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        panic!("STUB: not implemented");
    }
}
impl<'a> Acquire<'a> {
    fn new(semaphore: &'a Semaphore, num_permits: usize) -> Self {
        panic!("STUB: not implemented");
    }
    fn project(
        self: Pin<&mut Self>,
    ) -> (Pin<&mut Waiter>, &Semaphore, usize, &mut bool) {
        panic!("STUB: not implemented");
    }
}
impl Drop for Acquire<'_> {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
unsafe impl Sync for Acquire<'_> {}
impl AcquireError {
    fn closed() -> AcquireError {
        panic!("STUB: not implemented");
    }
}
impl fmt::Display for AcquireError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl std::error::Error for AcquireError {}
impl TryAcquireError {
    /// Returns `true` if the error was caused by a closed semaphore.
    #[allow(dead_code)]
    pub(crate) fn is_closed(&self) -> bool {
        panic!("STUB: not implemented");
    }
    /// Returns `true` if the error was caused by calling `try_acquire` on a
    /// semaphore with no available permits.
    #[allow(dead_code)]
    pub(crate) fn is_no_permits(&self) -> bool {
        panic!("STUB: not implemented");
    }
}
impl fmt::Display for TryAcquireError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl std::error::Error for TryAcquireError {}
/// # Safety
///
/// `Waiter` is forced to be !Unpin.
unsafe impl linked_list::Link for Waiter {
    type Handle = NonNull<Waiter>;
    type Target = Waiter;
    fn as_raw(handle: &Self::Handle) -> NonNull<Waiter> {
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
