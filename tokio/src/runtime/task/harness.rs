use crate::future::Future;
use crate::runtime::task::core::{Cell, Core, Header, Trailer};
use crate::runtime::task::state::{Snapshot, State};
use crate::runtime::task::waker::waker_ref;
use crate::runtime::task::{Id, JoinError, Notified, RawTask, Schedule, Task};
#[cfg(tokio_unstable)]
use crate::runtime::TaskMeta;
use std::any::Any;
use std::mem;
use std::mem::ManuallyDrop;
use std::panic;
use std::ptr::NonNull;
use std::task::{Context, Poll, Waker};
/// Typed raw task handle.
pub(super) struct Harness<T: Future, S: 'static> {
    cell: NonNull<Cell<T, S>>,
}
impl<T, S> Harness<T, S>
where
    T: Future,
    S: 'static,
{
    pub(super) unsafe fn from_raw(ptr: NonNull<Header>) -> Harness<T, S> {
        panic!("STUB: not implemented");
    }
    fn header_ptr(&self) -> NonNull<Header> {
        panic!("STUB: not implemented");
    }
    fn header(&self) -> &Header {
        panic!("STUB: not implemented");
    }
    fn state(&self) -> &State {
        panic!("STUB: not implemented");
    }
    fn trailer(&self) -> &Trailer {
        panic!("STUB: not implemented");
    }
    fn core(&self) -> &Core<T, S> {
        panic!("STUB: not implemented");
    }
}
/// Task operations that can be implemented without being generic over the
/// scheduler or task. Only one version of these methods should exist in the
/// final binary.
impl RawTask {
    pub(super) fn drop_reference(self) {
        panic!("STUB: not implemented");
    }
    /// This call consumes a ref-count and notifies the task. This will create a
    /// new Notified and submit it if necessary.
    ///
    /// The caller does not need to hold a ref-count besides the one that was
    /// passed to this call.
    pub(super) fn wake_by_val(&self) {
        panic!("STUB: not implemented");
    }
    /// This call notifies the task. It will not consume any ref-counts, but the
    /// caller should hold a ref-count.  This will create a new Notified and
    /// submit it if necessary.
    pub(super) fn wake_by_ref(&self) {
        panic!("STUB: not implemented");
    }
    /// Remotely aborts the task.
    ///
    /// The caller should hold a ref-count, but we do not consume it.
    ///
    /// This is similar to `shutdown` except that it asks the runtime to perform
    /// the shutdown. This is necessary to avoid the shutdown happening in the
    /// wrong thread for non-Send tasks.
    pub(super) fn remote_abort(&self) {
        panic!("STUB: not implemented");
    }
    /// Try to set the waker notified when the task is complete. Returns true if
    /// the task has already completed. If this call returns false, then the
    /// waker will not be notified.
    pub(super) fn try_set_join_waker(&self, waker: &Waker) -> bool {
        panic!("STUB: not implemented");
    }
}
impl<T, S> Harness<T, S>
where
    T: Future,
    S: Schedule,
{
    pub(super) fn drop_reference(self) {
        panic!("STUB: not implemented");
    }
    /// Polls the inner future. A ref-count is consumed.
    ///
    /// All necessary state checks and transitions are performed.
    /// Panics raised while polling the future are handled.
    pub(super) fn poll(self) {
        panic!("STUB: not implemented");
    }
    /// Polls the task and cancel it if necessary. This takes ownership of a
    /// ref-count.
    ///
    /// If the return value is Notified, the caller is given ownership of two
    /// ref-counts.
    ///
    /// If the return value is Complete, the caller is given ownership of a
    /// single ref-count, which should be passed on to `complete`.
    ///
    /// If the return value is `Dealloc`, then this call consumed the last
    /// ref-count and the caller should call `dealloc`.
    ///
    /// Otherwise the ref-count is consumed and the caller should not access
    /// `self` again.
    fn poll_inner(&self) -> PollFuture {
        panic!("STUB: not implemented");
    }
    /// Forcibly shuts down the task.
    ///
    /// Attempt to transition to `Running` in order to forcibly shutdown the
    /// task. If the task is currently running or in a state of completion, then
    /// there is nothing further to do. When the task completes running, it will
    /// notice the `CANCELLED` bit and finalize the task.
    pub(super) fn shutdown(self) {
        panic!("STUB: not implemented");
    }
    pub(super) fn dealloc(self) {
        panic!("STUB: not implemented");
    }
    /// Read the task output into `dst`.
    pub(super) fn try_read_output(
        self,
        dst: &mut Poll<super::Result<T::Output>>,
        waker: &Waker,
    ) {
        panic!("STUB: not implemented");
    }
    pub(super) fn drop_join_handle_slow(self) {
        panic!("STUB: not implemented");
    }
    /// Completes the task. This method assumes that the state is RUNNING.
    fn complete(self) {
        panic!("STUB: not implemented");
    }
    /// Releases the task from the scheduler. Returns the number of ref-counts
    /// that should be decremented.
    fn release(&self) -> usize {
        panic!("STUB: not implemented");
    }
    /// Creates a new task that holds its own ref-count.
    ///
    /// # Safety
    ///
    /// Any use of `self` after this call must ensure that a ref-count to the
    /// task holds the task alive until after the use of `self`. Passing the
    /// returned Task to any method on `self` is unsound if dropping the Task
    /// could drop `self` before the call on `self` returned.
    fn get_new_task(&self) -> Task<S> {
        panic!("STUB: not implemented");
    }
}
fn can_read_output(header: &Header, trailer: &Trailer, waker: &Waker) -> bool {
    panic!("STUB: not implemented");
}
fn set_join_waker(
    header: &Header,
    trailer: &Trailer,
    waker: Waker,
    snapshot: Snapshot,
) -> Result<Snapshot, Snapshot> {
    panic!("STUB: not implemented");
}
enum PollFuture {
    Complete,
    Notified,
    Done,
    Dealloc,
}
/// Cancels the task and store the appropriate error in the stage field.
fn cancel_task<T: Future, S: Schedule>(core: &Core<T, S>) {
    panic!("STUB: not implemented");
}
fn panic_result_to_join_error(
    task_id: Id,
    res: Result<(), Box<dyn Any + Send + 'static>>,
) -> JoinError {
    panic!("STUB: not implemented");
}
/// Polls the future. If the future completes, the output is written to the
/// stage field.
fn poll_future<T: Future, S: Schedule>(core: &Core<T, S>, cx: Context<'_>) -> Poll<()> {
    panic!("STUB: not implemented");
}
#[cold]
fn panic_to_error<S: Schedule>(
    scheduler: &S,
    task_id: Id,
    panic: Box<dyn Any + Send + 'static>,
) -> JoinError {
    panic!("STUB: not implemented");
}
