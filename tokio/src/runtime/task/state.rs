use crate::loom::sync::atomic::AtomicUsize;
use std::fmt;
use std::sync::atomic::Ordering::{AcqRel, Acquire, Release};
pub(super) struct State {
    val: AtomicUsize,
}
/// Current state value.
#[derive(Copy, Clone)]
pub(super) struct Snapshot(usize);
type UpdateResult = Result<Snapshot, Snapshot>;
/// The task is currently being run.
const RUNNING: usize = 0b0001;
/// The task is complete.
///
/// Once this bit is set, it is never unset.
const COMPLETE: usize = 0b0010;
/// Extracts the task's lifecycle value from the state.
const LIFECYCLE_MASK: usize = 0b11;
/// Flag tracking if the task has been pushed into a run queue.
const NOTIFIED: usize = 0b100;
/// The join handle is still around.
const JOIN_INTEREST: usize = 0b1_000;
/// A join handle waker has been set.
const JOIN_WAKER: usize = 0b10_000;
/// The task has been forcibly cancelled.
const CANCELLED: usize = 0b100_000;
/// All bits.
const STATE_MASK: usize = LIFECYCLE_MASK | NOTIFIED | JOIN_INTEREST | JOIN_WAKER
    | CANCELLED;
/// Bits used by the ref count portion of the state.
const REF_COUNT_MASK: usize = !STATE_MASK;
/// Number of positions to shift the ref count.
const REF_COUNT_SHIFT: usize = REF_COUNT_MASK.count_zeros() as usize;
/// One ref count.
const REF_ONE: usize = 1 << REF_COUNT_SHIFT;
/// State a task is initialized with.
///
/// A task is initialized with three references:
///
///  * A reference that will be stored in an `OwnedTasks` or `LocalOwnedTasks`.
///  * A reference that will be sent to the scheduler as an ordinary notification.
///  * A reference for the `JoinHandle`.
///
/// As the task starts with a `JoinHandle`, `JOIN_INTEREST` is set.
/// As the task starts with a `Notified`, `NOTIFIED` is set.
const INITIAL_STATE: usize = (REF_ONE * 3) | JOIN_INTEREST | NOTIFIED;
#[must_use]
pub(super) enum TransitionToRunning {
    Success,
    Cancelled,
    Failed,
    Dealloc,
}
#[must_use]
pub(super) enum TransitionToIdle {
    Ok,
    OkNotified,
    OkDealloc,
    Cancelled,
}
#[must_use]
pub(super) enum TransitionToNotifiedByVal {
    DoNothing,
    Submit,
    Dealloc,
}
#[must_use]
pub(crate) enum TransitionToNotifiedByRef {
    DoNothing,
    Submit,
}
#[must_use]
pub(super) struct TransitionToJoinHandleDrop {
    pub(super) drop_waker: bool,
    pub(super) drop_output: bool,
}
/// All transitions are performed via RMW operations. This establishes an
/// unambiguous modification order.
impl State {
    /// Returns a task's initial state.
    pub(super) fn new() -> State {
        panic!("STUB: not implemented");
    }
    /// Loads the current state, establishes `Acquire` ordering.
    pub(super) fn load(&self) -> Snapshot {
        panic!("STUB: not implemented");
    }
    /// Attempts to transition the lifecycle to `Running`. This sets the
    /// notified bit to false so notifications during the poll can be detected.
    pub(super) fn transition_to_running(&self) -> TransitionToRunning {
        panic!("STUB: not implemented");
    }
    /// Transitions the task from `Running` -> `Idle`.
    ///
    /// The transition to `Idle` fails if the task has been flagged to be
    /// cancelled.
    pub(super) fn transition_to_idle(&self) -> TransitionToIdle {
        panic!("STUB: not implemented");
    }
    /// Transitions the task from `Running` -> `Complete`.
    pub(super) fn transition_to_complete(&self) -> Snapshot {
        panic!("STUB: not implemented");
    }
    /// Transitions from `Complete` -> `Terminal`, decrementing the reference
    /// count the specified number of times.
    ///
    /// Returns true if the task should be deallocated.
    pub(super) fn transition_to_terminal(&self, count: usize) -> bool {
        panic!("STUB: not implemented");
    }
    /// Transitions the state to `NOTIFIED`.
    ///
    /// If no task needs to be submitted, a ref-count is consumed.
    ///
    /// If a task needs to be submitted, the ref-count is incremented for the
    /// new Notified.
    pub(super) fn transition_to_notified_by_val(&self) -> TransitionToNotifiedByVal {
        panic!("STUB: not implemented");
    }
    /// Transitions the state to `NOTIFIED`.
    pub(super) fn transition_to_notified_by_ref(&self) -> TransitionToNotifiedByRef {
        panic!("STUB: not implemented");
    }
    cfg_taskdump! {
        #[doc =
        " Transitions the state to `NOTIFIED`, unconditionally increasing the ref"] #[doc
        = " count."] #[doc = ""] #[doc =
        " Returns `true` if the notified bit was transitioned from `0` to `1`;"] #[doc =
        " otherwise `false.`"] pub (super) fn transition_to_notified_for_tracing(& self)
        -> bool { self.fetch_update_action(| mut snapshot | { if snapshot.is_notified() {
        (false, None) } else { snapshot.set_notified(); snapshot.ref_inc(); (true,
        Some(snapshot)) } }) }
    }
    /// Sets the cancelled bit and transitions the state to `NOTIFIED` if idle.
    ///
    /// Returns `true` if the task needs to be submitted to the pool for
    /// execution.
    pub(super) fn transition_to_notified_and_cancel(&self) -> bool {
        panic!("STUB: not implemented");
    }
    /// Sets the `CANCELLED` bit and attempts to transition to `Running`.
    ///
    /// Returns `true` if the transition to `Running` succeeded.
    pub(super) fn transition_to_shutdown(&self) -> bool {
        panic!("STUB: not implemented");
    }
    /// Optimistically tries to swap the state assuming the join handle is
    /// __immediately__ dropped on spawn.
    pub(super) fn drop_join_handle_fast(&self) -> Result<(), ()> {
        panic!("STUB: not implemented");
    }
    /// Unsets the `JOIN_INTEREST` flag. If `COMPLETE` is not set, the `JOIN_WAKER`
    /// flag is also unset.
    /// The returned `TransitionToJoinHandleDrop` indicates whether the `JoinHandle` should drop
    /// the output of the future or the join waker after the transition.
    pub(super) fn transition_to_join_handle_dropped(
        &self,
    ) -> TransitionToJoinHandleDrop {
        panic!("STUB: not implemented");
    }
    /// Sets the `JOIN_WAKER` bit.
    ///
    /// Returns `Ok` if the bit is set, `Err` otherwise. This operation fails if
    /// the task has completed.
    pub(super) fn set_join_waker(&self) -> UpdateResult {
        panic!("STUB: not implemented");
    }
    /// Unsets the `JOIN_WAKER` bit.
    ///
    /// Returns `Ok` has been unset, `Err` otherwise. This operation fails if
    /// the task has completed.
    pub(super) fn unset_waker(&self) -> UpdateResult {
        panic!("STUB: not implemented");
    }
    /// Unsets the `JOIN_WAKER` bit unconditionally after task completion.
    ///
    /// This operation requires the task to be completed.
    pub(super) fn unset_waker_after_complete(&self) -> Snapshot {
        panic!("STUB: not implemented");
    }
    pub(super) fn ref_inc(&self) {
        panic!("STUB: not implemented");
    }
    /// Returns `true` if the task should be released.
    pub(super) fn ref_dec(&self) -> bool {
        panic!("STUB: not implemented");
    }
    /// Returns `true` if the task should be released.
    pub(super) fn ref_dec_twice(&self) -> bool {
        panic!("STUB: not implemented");
    }
    fn fetch_update_action<F, T>(&self, mut f: F) -> T
    where
        F: FnMut(Snapshot) -> (T, Option<Snapshot>),
    {
        panic!("STUB: not implemented");
    }
    fn fetch_update<F>(&self, mut f: F) -> Result<Snapshot, Snapshot>
    where
        F: FnMut(Snapshot) -> Option<Snapshot>,
    {
        panic!("STUB: not implemented");
    }
}
impl Snapshot {
    /// Returns `true` if the task is in an idle state.
    pub(super) fn is_idle(self) -> bool {
        panic!("STUB: not implemented");
    }
    /// Returns `true` if the task has been flagged as notified.
    pub(super) fn is_notified(self) -> bool {
        panic!("STUB: not implemented");
    }
    fn unset_notified(&mut self) {
        panic!("STUB: not implemented");
    }
    fn set_notified(&mut self) {
        panic!("STUB: not implemented");
    }
    pub(super) fn is_running(self) -> bool {
        panic!("STUB: not implemented");
    }
    fn set_running(&mut self) {
        panic!("STUB: not implemented");
    }
    fn unset_running(&mut self) {
        panic!("STUB: not implemented");
    }
    pub(super) fn is_cancelled(self) -> bool {
        panic!("STUB: not implemented");
    }
    fn set_cancelled(&mut self) {
        panic!("STUB: not implemented");
    }
    /// Returns `true` if the task's future has completed execution.
    pub(super) fn is_complete(self) -> bool {
        panic!("STUB: not implemented");
    }
    pub(super) fn is_join_interested(self) -> bool {
        panic!("STUB: not implemented");
    }
    fn unset_join_interested(&mut self) {
        panic!("STUB: not implemented");
    }
    pub(super) fn is_join_waker_set(self) -> bool {
        panic!("STUB: not implemented");
    }
    fn set_join_waker(&mut self) {
        panic!("STUB: not implemented");
    }
    fn unset_join_waker(&mut self) {
        panic!("STUB: not implemented");
    }
    pub(super) fn ref_count(self) -> usize {
        panic!("STUB: not implemented");
    }
    fn ref_inc(&mut self) {
        panic!("STUB: not implemented");
    }
    pub(super) fn ref_dec(&mut self) {
        panic!("STUB: not implemented");
    }
}
impl fmt::Debug for State {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl fmt::Debug for Snapshot {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
