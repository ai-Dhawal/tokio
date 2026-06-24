use crate::loom::cell::UnsafeCell;
use crate::loom::future::AtomicWaker;
use crate::loom::sync::atomic::AtomicUsize;
use crate::loom::sync::Arc;
use crate::runtime::park::CachedParkThread;
use crate::sync::mpsc::error::TryRecvError;
use crate::sync::mpsc::{bounded, list, unbounded};
use crate::sync::notify::Notify;
use crate::util::cacheline::CachePadded;
use std::fmt;
use std::panic;
use std::process;
use std::sync::atomic::Ordering::{AcqRel, Acquire, Relaxed, Release};
use std::task::Poll::{Pending, Ready};
use std::task::{ready, Context, Poll};
/// Channel sender.
pub(crate) struct Tx<T, S> {
    inner: Arc<Chan<T, S>>,
}
impl<T, S: fmt::Debug> fmt::Debug for Tx<T, S> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
/// Channel receiver.
pub(crate) struct Rx<T, S: Semaphore> {
    inner: Arc<Chan<T, S>>,
}
impl<T, S: Semaphore + fmt::Debug> fmt::Debug for Rx<T, S> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
pub(crate) trait Semaphore {
    fn is_idle(&self) -> bool;
    fn add_permit(&self);
    fn add_permits(&self, n: usize);
    fn close(&self);
    fn is_closed(&self) -> bool;
}
pub(super) struct Chan<T, S> {
    /// Handle to the push half of the lock-free list.
    tx: CachePadded<list::Tx<T>>,
    /// Receiver waker. Notified when a value is pushed into the channel.
    rx_waker: CachePadded<AtomicWaker>,
    /// Notifies all tasks listening for the receiver being dropped.
    notify_rx_closed: Notify,
    /// Coordinates access to channel's capacity.
    semaphore: S,
    /// Tracks the number of outstanding sender handles.
    ///
    /// When this drops to zero, the send half of the channel is closed.
    tx_count: AtomicUsize,
    /// Tracks the number of outstanding weak sender handles.
    tx_weak_count: AtomicUsize,
    /// Only accessed by `Rx` handle.
    rx_fields: UnsafeCell<RxFields<T>>,
}
impl<T, S> fmt::Debug for Chan<T, S>
where
    S: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
/// Fields only accessed by `Rx` handle.
struct RxFields<T> {
    /// Channel receiver. This field is only accessed by the `Receiver` type.
    list: list::Rx<T>,
    /// `true` if `Rx::close` is called.
    rx_closed: bool,
}
impl<T> fmt::Debug for RxFields<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
unsafe impl<T: Send, S: Send> Send for Chan<T, S> {}
unsafe impl<T: Send, S: Sync> Sync for Chan<T, S> {}
impl<T, S> panic::RefUnwindSafe for Chan<T, S> {}
impl<T, S> panic::UnwindSafe for Chan<T, S> {}
pub(crate) fn channel<T, S: Semaphore>(semaphore: S) -> (Tx<T, S>, Rx<T, S>) {
    panic!("STUB: not implemented");
}
impl<T, S> Tx<T, S> {
    fn new(chan: Arc<Chan<T, S>>) -> Tx<T, S> {
        panic!("STUB: not implemented");
    }
    pub(super) fn strong_count(&self) -> usize {
        panic!("STUB: not implemented");
    }
    pub(super) fn weak_count(&self) -> usize {
        panic!("STUB: not implemented");
    }
    pub(super) fn downgrade(&self) -> Arc<Chan<T, S>> {
        panic!("STUB: not implemented");
    }
    pub(super) fn upgrade(chan: Arc<Chan<T, S>>) -> Option<Self> {
        panic!("STUB: not implemented");
    }
    pub(super) fn semaphore(&self) -> &S {
        panic!("STUB: not implemented");
    }
    /// Send a message and notify the receiver.
    pub(crate) fn send(&self, value: T) {
        panic!("STUB: not implemented");
    }
    /// Wake the receive half
    pub(crate) fn wake_rx(&self) {
        panic!("STUB: not implemented");
    }
    /// Returns `true` if senders belong to the same channel.
    pub(crate) fn same_channel(&self, other: &Self) -> bool {
        panic!("STUB: not implemented");
    }
}
impl<T, S: Semaphore> Tx<T, S> {
    pub(crate) fn is_closed(&self) -> bool {
        panic!("STUB: not implemented");
    }
    pub(crate) async fn closed(&self) {
        panic!("STUB: not implemented");
    }
}
impl<T, S> Clone for Tx<T, S> {
    fn clone(&self) -> Tx<T, S> {
        panic!("STUB: not implemented");
    }
}
impl<T, S> Drop for Tx<T, S> {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
impl<T, S: Semaphore> Rx<T, S> {
    fn new(chan: Arc<Chan<T, S>>) -> Rx<T, S> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn close(&mut self) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn is_closed(&self) -> bool {
        panic!("STUB: not implemented");
    }
    pub(crate) fn is_empty(&self) -> bool {
        panic!("STUB: not implemented");
    }
    pub(crate) fn len(&self) -> usize {
        panic!("STUB: not implemented");
    }
    /// Receive the next value
    pub(crate) fn recv(&mut self, cx: &mut Context<'_>) -> Poll<Option<T>> {
        panic!("STUB: not implemented");
    }
    /// Receives up to `limit` values into `buffer`
    ///
    /// For `limit > 0`, receives up to limit values into `buffer`.
    /// For `limit == 0`, immediately returns Ready(0).
    pub(crate) fn recv_many(
        &mut self,
        cx: &mut Context<'_>,
        buffer: &mut Vec<T>,
        limit: usize,
    ) -> Poll<usize> {
        panic!("STUB: not implemented");
    }
    /// Try to receive the next value.
    pub(crate) fn try_recv(&mut self) -> Result<T, TryRecvError> {
        panic!("STUB: not implemented");
    }
    pub(super) fn semaphore(&self) -> &S {
        panic!("STUB: not implemented");
    }
    pub(super) fn sender_strong_count(&self) -> usize {
        panic!("STUB: not implemented");
    }
    pub(super) fn sender_weak_count(&self) -> usize {
        panic!("STUB: not implemented");
    }
}
impl<T, S: Semaphore> Drop for Rx<T, S> {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
impl<T, S> Chan<T, S> {
    fn send(&self, value: T) {
        panic!("STUB: not implemented");
    }
    pub(super) fn decrement_weak_count(&self) {
        panic!("STUB: not implemented");
    }
    pub(super) fn increment_weak_count(&self) {
        panic!("STUB: not implemented");
    }
    pub(super) fn strong_count(&self) -> usize {
        panic!("STUB: not implemented");
    }
    pub(super) fn weak_count(&self) -> usize {
        panic!("STUB: not implemented");
    }
}
impl<T, S> Drop for Chan<T, S> {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
impl Semaphore for bounded::Semaphore {
    fn add_permit(&self) {
        panic!("STUB: not implemented");
    }
    fn add_permits(&self, n: usize) {
        panic!("STUB: not implemented");
    }
    fn is_idle(&self) -> bool {
        panic!("STUB: not implemented");
    }
    fn close(&self) {
        panic!("STUB: not implemented");
    }
    fn is_closed(&self) -> bool {
        panic!("STUB: not implemented");
    }
}
impl Semaphore for unbounded::Semaphore {
    fn add_permit(&self) {
        panic!("STUB: not implemented");
    }
    fn add_permits(&self, n: usize) {
        panic!("STUB: not implemented");
    }
    fn is_idle(&self) -> bool {
        panic!("STUB: not implemented");
    }
    fn close(&self) {
        panic!("STUB: not implemented");
    }
    fn is_closed(&self) -> bool {
        panic!("STUB: not implemented");
    }
}
