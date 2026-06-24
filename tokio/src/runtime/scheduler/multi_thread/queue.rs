//! Run-queue structures to support a work-stealing scheduler
use crate::loom::cell::UnsafeCell;
use crate::loom::sync::Arc;
use crate::runtime::scheduler::multi_thread::{Overflow, Stats};
use crate::runtime::task;
use std::mem::{self, MaybeUninit};
use std::ptr;
use std::sync::atomic::Ordering::{AcqRel, Acquire, Relaxed, Release};
cfg_has_atomic_u64! {
    type UnsignedShort = u32; type UnsignedLong = u64; type AtomicUnsignedShort = crate
    ::loom::sync::atomic::AtomicU32; type AtomicUnsignedLong = crate
    ::loom::sync::atomic::AtomicU64;
}
cfg_not_has_atomic_u64! {
    type UnsignedShort = u16; type UnsignedLong = u32; type AtomicUnsignedShort = crate
    ::loom::sync::atomic::AtomicU16; type AtomicUnsignedLong = crate
    ::loom::sync::atomic::AtomicU32;
}
/// Producer handle. May only be used from a single thread.
pub(crate) struct Local<T: 'static> {
    inner: Arc<Inner<T>>,
}
/// Consumer handle. May be used from many threads.
pub(crate) struct Steal<T: 'static>(Arc<Inner<T>>);
pub(crate) struct Inner<T: 'static> {
    /// Concurrently updated by many threads.
    ///
    /// Contains two `UnsignedShort` values. The `LSB` byte is the "real" head of
    /// the queue. The `UnsignedShort` in the `MSB` is set by a stealer in process
    /// of stealing values. It represents the first value being stolen in the
    /// batch. The `UnsignedShort` indices are intentionally wider than strictly
    /// required for buffer indexing in order to provide ABA mitigation and make
    /// it possible to distinguish between full and empty buffers.
    ///
    /// When both `UnsignedShort` values are the same, there is no active
    /// stealer.
    ///
    /// Tracking an in-progress stealer prevents a wrapping scenario.
    head: AtomicUnsignedLong,
    /// Only updated by producer thread but read by many threads.
    tail: AtomicUnsignedShort,
    /// Elements
    buffer: Box<[UnsafeCell<MaybeUninit<task::Notified<T>>>; LOCAL_QUEUE_CAPACITY]>,
}
unsafe impl<T> Send for Inner<T> {}
unsafe impl<T> Sync for Inner<T> {}
#[cfg(not(loom))]
const LOCAL_QUEUE_CAPACITY: usize = 256;
#[cfg(loom)]
const LOCAL_QUEUE_CAPACITY: usize = 4;
const MASK: usize = LOCAL_QUEUE_CAPACITY - 1;
fn make_fixed_size<T>(buffer: Box<[T]>) -> Box<[T; LOCAL_QUEUE_CAPACITY]> {
    panic!("STUB: not implemented");
}
/// Create a new local run-queue
pub(crate) fn local<T: 'static>() -> (Steal<T>, Local<T>) {
    panic!("STUB: not implemented");
}
impl<T> Local<T> {
    /// Returns the number of entries in the queue
    pub(crate) fn len(&self) -> usize {
        panic!("STUB: not implemented");
    }
    /// How many tasks can be pushed into the queue
    pub(crate) fn remaining_slots(&self) -> usize {
        panic!("STUB: not implemented");
    }
    pub(crate) fn max_capacity(&self) -> usize {
        panic!("STUB: not implemented");
    }
    /// Returns false if there are any entries in the queue
    ///
    /// Separate to `is_stealable` so that refactors of `is_stealable` to "protect"
    /// some tasks from stealing won't affect this
    pub(crate) fn has_tasks(&self) -> bool {
        panic!("STUB: not implemented");
    }
    /// Pushes a batch of tasks to the back of the queue. All tasks must fit in
    /// the local queue.
    ///
    /// # Panics
    ///
    /// The method panics if there is not enough capacity to fit in the queue.
    pub(crate) fn push_back(
        &mut self,
        tasks: impl ExactSizeIterator<Item = task::Notified<T>>,
    ) {
        panic!("STUB: not implemented");
    }
    /// Pushes a task to the back of the local queue, if there is not enough
    /// capacity in the queue, this triggers the overflow operation.
    ///
    /// When the queue overflows, half of the current contents of the queue is
    /// moved to the given Injection queue. This frees up capacity for more
    /// tasks to be pushed into the local queue.
    pub(crate) fn push_back_or_overflow<O: Overflow<T>>(
        &mut self,
        mut task: task::Notified<T>,
        overflow: &O,
        stats: &mut Stats,
    ) {
        panic!("STUB: not implemented");
    }
    fn push_back_finish(&self, task: task::Notified<T>, tail: UnsignedShort) {
        panic!("STUB: not implemented");
    }
    /// Moves a batch of tasks into the inject queue.
    ///
    /// This will temporarily make some of the tasks unavailable to stealers.
    /// Once `push_overflow` is done, a notification is sent out, so if other
    /// workers "missed" some of the tasks during a steal, they will get
    /// another opportunity.
    #[inline(never)]
    fn push_overflow<O: Overflow<T>>(
        &mut self,
        task: task::Notified<T>,
        head: UnsignedShort,
        tail: UnsignedShort,
        overflow: &O,
        stats: &mut Stats,
    ) -> Result<(), task::Notified<T>> {
        panic!("STUB: not implemented");
    }
    /// Pops a task from the local queue.
    pub(crate) fn pop(&mut self) -> Option<task::Notified<T>> {
        panic!("STUB: not implemented");
    }
}
impl<T> Steal<T> {
    /// Returns the number of entries in the queue
    pub(crate) fn len(&self) -> usize {
        panic!("STUB: not implemented");
    }
    /// Return true if the queue is empty,
    /// false if there are any entries in the queue
    pub(crate) fn is_empty(&self) -> bool {
        panic!("STUB: not implemented");
    }
    /// Steals half the tasks from self and place them into `dst`.
    pub(crate) fn steal_into(
        &self,
        dst: &mut Local<T>,
        dst_stats: &mut Stats,
    ) -> Option<task::Notified<T>> {
        panic!("STUB: not implemented");
    }
    fn steal_into2(&self, dst: &mut Local<T>, dst_tail: UnsignedShort) -> UnsignedShort {
        panic!("STUB: not implemented");
    }
}
impl<T> Clone for Steal<T> {
    fn clone(&self) -> Steal<T> {
        panic!("STUB: not implemented");
    }
}
impl<T> Drop for Local<T> {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
/// Calculate the length of the queue using the head and tail.
/// The `head` can be the `steal` or `real` head.
fn len(head: UnsignedShort, tail: UnsignedShort) -> usize {
    panic!("STUB: not implemented");
}
/// Split the head value into the real head and the index a stealer is working
/// on.
fn unpack(n: UnsignedLong) -> (UnsignedShort, UnsignedShort) {
    panic!("STUB: not implemented");
}
/// Join the two head values
fn pack(steal: UnsignedShort, real: UnsignedShort) -> UnsignedLong {
    panic!("STUB: not implemented");
}
#[test]
fn test_local_queue_capacity() {
    assert!(LOCAL_QUEUE_CAPACITY - 1 <= u8::MAX as usize);
}
