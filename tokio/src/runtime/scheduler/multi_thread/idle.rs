//! Coordinates idling workers
use crate::loom::sync::atomic::AtomicUsize;
use crate::runtime::scheduler::multi_thread::Shared;
use std::fmt;
use std::sync::atomic::Ordering::{self, SeqCst};
pub(super) struct Idle {
    /// Tracks both the number of searching workers and the number of unparked
    /// workers.
    ///
    /// Used as a fast-path to avoid acquiring the lock when needed.
    state: AtomicUsize,
    /// Total number of workers.
    num_workers: usize,
}
/// Data synchronized by the scheduler mutex
pub(super) struct Synced {
    /// Sleeping workers
    sleepers: Vec<usize>,
}
const UNPARK_SHIFT: usize = 16;
const UNPARK_MASK: usize = !SEARCH_MASK;
const SEARCH_MASK: usize = (1 << UNPARK_SHIFT) - 1;
#[derive(Copy, Clone)]
struct State(usize);
impl Idle {
    pub(super) fn new(num_workers: usize) -> (Idle, Synced) {
        panic!("STUB: not implemented");
    }
    /// If there are no workers actively searching, returns the index of a
    /// worker currently sleeping.
    pub(super) fn worker_to_notify(&self, shared: &Shared) -> Option<usize> {
        panic!("STUB: not implemented");
    }
    /// Returns `true` if the worker needs to do a final check for submitted
    /// work.
    pub(super) fn transition_worker_to_parked(
        &self,
        shared: &Shared,
        worker: usize,
        is_searching: bool,
    ) -> bool {
        panic!("STUB: not implemented");
    }
    pub(super) fn transition_worker_to_searching(&self) -> bool {
        panic!("STUB: not implemented");
    }
    /// A lightweight transition from searching -> running.
    ///
    /// Returns `true` if this is the final searching worker. The caller
    /// **must** notify a new worker.
    pub(super) fn transition_worker_from_searching(&self) -> bool {
        panic!("STUB: not implemented");
    }
    /// Unpark a specific worker. This happens if tasks are submitted from
    /// within the worker's park routine.
    ///
    /// Returns `true` if the worker was parked before calling the method.
    pub(super) fn unpark_worker_by_id(&self, shared: &Shared, worker_id: usize) -> bool {
        panic!("STUB: not implemented");
    }
    /// Returns `true` if `worker_id` is contained in the sleep set.
    pub(super) fn is_parked(&self, shared: &Shared, worker_id: usize) -> bool {
        panic!("STUB: not implemented");
    }
    fn notify_should_wakeup(&self) -> bool {
        panic!("STUB: not implemented");
    }
}
impl State {
    fn new(num_workers: usize) -> State {
        panic!("STUB: not implemented");
    }
    fn load(cell: &AtomicUsize, ordering: Ordering) -> State {
        panic!("STUB: not implemented");
    }
    fn unpark_one(cell: &AtomicUsize, num_searching: usize) {
        panic!("STUB: not implemented");
    }
    fn inc_num_searching(cell: &AtomicUsize, ordering: Ordering) {
        panic!("STUB: not implemented");
    }
    /// Returns `true` if this is the final searching worker
    fn dec_num_searching(cell: &AtomicUsize) -> bool {
        panic!("STUB: not implemented");
    }
    /// Track a sleeping worker
    ///
    /// Returns `true` if this is the final searching worker.
    fn dec_num_unparked(cell: &AtomicUsize, is_searching: bool) -> bool {
        panic!("STUB: not implemented");
    }
    /// Number of workers currently searching
    fn num_searching(self) -> usize {
        panic!("STUB: not implemented");
    }
    /// Number of workers currently unparked
    fn num_unparked(self) -> usize {
        panic!("STUB: not implemented");
    }
}
impl From<usize> for State {
    fn from(src: usize) -> State {
        panic!("STUB: not implemented");
    }
}
impl From<State> for usize {
    fn from(src: State) -> usize {
        panic!("STUB: not implemented");
    }
}
impl fmt::Debug for State {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
#[test]
fn test_state() {
    assert_eq!(0, UNPARK_MASK & SEARCH_MASK);
    assert_eq!(0, ! (UNPARK_MASK | SEARCH_MASK));
    let state = State::new(10);
    assert_eq!(10, state.num_unparked());
    assert_eq!(0, state.num_searching());
}
