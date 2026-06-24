//! A scheduler is initialized with a fixed number of workers. Each worker is
//! driven by a thread. Each worker has a "core" which contains data such as the
//! run queue and other state. When `block_in_place` is called, the worker's
//! "core" is handed off to a new thread allowing the scheduler to continue to
//! make progress while the originating thread blocks.
//!
//! # Shutdown
//!
//! Shutting down the runtime involves the following steps:
//!
//!  1. The Shared::close method is called. This closes the inject queue and
//!     `OwnedTasks` instance and wakes up all worker threads.
//!
//!  2. Each worker thread observes the close signal next time it runs
//!     Core::maintenance by checking whether the inject queue is closed.
//!     The `Core::is_shutdown` flag is set to true.
//!
//!  3. The worker thread calls `pre_shutdown` in parallel. Here, the worker
//!     will keep removing tasks from `OwnedTasks` until it is empty. No new
//!     tasks can be pushed to the `OwnedTasks` during or after this step as it
//!     was closed in step 1.
//!
//!  5. The workers call Shared::shutdown to enter the single-threaded phase of
//!     shutdown. These calls will push their core to `Shared::shutdown_cores`,
//!     and the last thread to push its core will finish the shutdown procedure.
//!
//!  6. The local run queue of each core is emptied, then the inject queue is
//!     emptied.
//!
//! At this point, shutdown has completed. It is not possible for any of the
//! collections to contain any tasks at this point, as each collection was
//! closed first, then emptied afterwards.
//!
//! ## Spawns during shutdown
//!
//! When spawning tasks during shutdown, there are two cases:
//!
//!  * The spawner observes the `OwnedTasks` being open, and the inject queue is
//!    closed.
//!  * The spawner observes the `OwnedTasks` being closed and doesn't check the
//!    inject queue.
//!
//! The first case can only happen if the `OwnedTasks::bind` call happens before
//! or during step 1 of shutdown. In this case, the runtime will clean up the
//! task in step 3 of shutdown.
//!
//! In the latter case, the task was not spawned and the task is immediately
//! cancelled by the spawner.
//!
//! The correctness of shutdown requires both the inject queue and `OwnedTasks`
//! collection to have a closed bit. With a close bit on only the inject queue,
//! spawning could run in to a situation where a task is successfully bound long
//! after the runtime has shut down. With a close bit on only the `OwnedTasks`,
//! the first spawning situation could result in the notification being pushed
//! to the inject queue after step 6 of shutdown, which would leave a task in
//! the inject queue indefinitely. This would be a ref-count cycle and a memory
//! leak.
use crate::loom::sync::{Arc, Mutex};
use crate::runtime;
use crate::runtime::scheduler::multi_thread::{
    idle, park, queue, Counters, Handle, Idle, Overflow, Parker, Stats, TraceStatus,
    Unparker,
};
use crate::runtime::scheduler::{inject, Defer, Lock};
use crate::runtime::task::OwnedTasks;
use crate::runtime::{
    blocking, driver, scheduler, task, Config, SchedulerMetrics, TimerFlavor,
    WorkerMetrics,
};
use crate::runtime::{context, TaskHooks};
use crate::task::coop;
use crate::util::atomic_cell::AtomicCell;
use crate::util::rand::{FastRand, RngSeedGenerator};
use std::cell::RefCell;
use std::task::Waker;
use std::thread;
use std::time::{Duration, Instant};
mod metrics;
cfg_taskdump! {
    mod taskdump;
}
cfg_not_taskdump! {
    mod taskdump_mock;
}
#[cfg(all(tokio_unstable, feature = "time"))]
use crate::loom::sync::atomic::AtomicBool;
#[cfg(all(tokio_unstable, feature = "time"))]
use crate::runtime::time_alt;
use crate::runtime::metrics::ScheduleLatencyInstant;
#[cfg(all(tokio_unstable, feature = "time"))]
use crate::runtime::scheduler::util;
/// A scheduler worker
pub(super) struct Worker {
    /// Reference to scheduler's handle
    handle: Arc<Handle>,
    /// Index holding this worker's remote state
    index: usize,
    /// Used to hand-off a worker's core to another thread.
    core: AtomicCell<Core>,
}
/// Core data
struct Core {
    /// Used to schedule bookkeeping tasks every so often.
    tick: u32,
    /// When a task is scheduled from a worker, it is stored in this slot. The
    /// worker will check this slot for a task **before** checking the run
    /// queue. This effectively results in the **last** scheduled task to be run
    /// next (LIFO). This is an optimization for improving locality which
    /// benefits message passing patterns and helps to reduce latency.
    lifo_slot: Option<Notified>,
    /// When `true`, locally scheduled tasks go to the LIFO slot. When `false`,
    /// they go to the back of the `run_queue`.
    lifo_enabled: bool,
    /// The worker-local run queue.
    run_queue: queue::Local<Arc<Handle>>,
    #[cfg(all(tokio_unstable, feature = "time"))]
    time_context: time_alt::LocalContext,
    /// True if the worker is currently searching for more work. Searching
    /// involves attempting to steal from other workers.
    is_searching: bool,
    /// True if the scheduler is being shutdown
    is_shutdown: bool,
    /// True if the scheduler is being traced
    is_traced: bool,
    /// Whether or not the worker has just returned from a park in which we
    /// parked on the I/O driver.
    had_driver: park::HadDriver,
    /// If `true`, the worker should eagerly notify another worker when polling
    /// the first task after returning from a park in which it parked on the I/O
    /// or time driver.
    enable_eager_driver_handoff: bool,
    /// Parker
    ///
    /// Stored in an `Option` as the parker is added / removed to make the
    /// borrow checker happy.
    park: Option<Parker>,
    /// Per-worker runtime stats
    stats: Stats,
    /// How often to check the global queue
    global_queue_interval: u32,
    /// Fast random number generator.
    rand: FastRand,
}
/// State shared across all workers
pub(crate) struct Shared {
    /// Per-worker remote state. All other workers have access to this and is
    /// how they communicate between each other.
    remotes: Box<[Remote]>,
    /// Global task queue used for:
    ///  1. Submit work to the scheduler while **not** currently on a worker thread.
    ///  2. Submit work to the scheduler when a worker run queue is saturated
    pub(super) inject: inject::Shared<Arc<Handle>>,
    /// Coordinates idle workers
    idle: Idle,
    /// Collection of all active tasks spawned onto this executor.
    pub(crate) owned: OwnedTasks<Arc<Handle>>,
    /// Data synchronized by the scheduler mutex
    pub(super) synced: Mutex<Synced>,
    /// Cores that have observed the shutdown signal
    ///
    /// The core is **not** placed back in the worker to avoid it from being
    /// stolen by a thread that was spawned as part of `block_in_place`.
    #[allow(clippy::vec_box)]
    shutdown_cores: Mutex<Vec<Box<Core>>>,
    /// The number of cores that have observed the trace signal.
    pub(super) trace_status: TraceStatus,
    /// Scheduler configuration options
    config: Config,
    /// Collects metrics from the runtime.
    pub(super) scheduler_metrics: SchedulerMetrics,
    pub(super) worker_metrics: Box<[WorkerMetrics]>,
    /// Startup time of this scheduler.
    ///
    /// This instant is used as the basis of task `scheduled_at` measurements.
    started_at: Option<Instant>,
    /// Only held to trigger some code on drop. This is used to get internal
    /// runtime metrics that can be useful when doing performance
    /// investigations. This does nothing (empty struct, no drop impl) unless
    /// the `tokio_internal_mt_counters` `cfg` flag is set.
    _counters: Counters,
}
/// Data synchronized by the scheduler mutex
pub(crate) struct Synced {
    /// Synchronized state for `Idle`.
    pub(super) idle: idle::Synced,
    /// Synchronized state for `Inject`.
    pub(crate) inject: inject::Synced,
    #[cfg(all(tokio_unstable, feature = "time"))]
    /// Timers pending to be registered.
    /// This is used to register a timer but the [`Core`]
    /// is not available in the current thread.
    inject_timers: Vec<time_alt::EntryHandle>,
}
/// Used to communicate with a worker from other threads.
struct Remote {
    /// Steals tasks from this worker.
    pub(super) steal: queue::Steal<Arc<Handle>>,
    /// Unparks the associated worker thread
    unpark: Unparker,
}
/// Thread-local context
pub(crate) struct Context {
    /// Worker
    worker: Arc<Worker>,
    /// Core data
    core: RefCell<Option<Box<Core>>>,
    /// Tasks to wake after resource drivers are polled. This is mostly to
    /// handle yielded tasks.
    pub(crate) defer: Defer,
}
/// Starts the workers
pub(crate) struct Launch(Vec<Arc<Worker>>);
/// Running a task may consume the core. If the core is still available when
/// running the task completes, it is returned. Otherwise, the worker will need
/// to stop processing.
type RunResult = Result<Box<Core>, ()>;
/// A notified task handle
type Notified = task::Notified<Arc<Handle>>;
/// Value picked out of thin-air. Running the LIFO slot a handful of times
/// seems sufficient to benefit from locality. More than 3 times probably is
/// over-weighting. The value can be tuned in the future with data that shows
/// improvements.
const MAX_LIFO_POLLS_PER_TICK: usize = 3;
#[allow(clippy::too_many_arguments)]
pub(super) fn create(
    size: usize,
    park: Parker,
    driver_handle: driver::Handle,
    blocking_spawner: blocking::Spawner,
    seed_generator: RngSeedGenerator,
    config: Config,
    timer_flavor: TimerFlavor,
    name: Option<String>,
) -> (Arc<Handle>, Launch) {
    panic!("STUB: not implemented");
}
#[track_caller]
pub(crate) fn block_in_place<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    panic!("STUB: not implemented");
}
impl Launch {
    pub(crate) fn launch(mut self) {
        panic!("STUB: not implemented");
    }
}
fn run(worker: Arc<Worker>) {
    panic!("STUB: not implemented");
}
impl Context {
    fn run(&self, mut core: Box<Core>) -> RunResult {
        panic!("STUB: not implemented");
    }
    fn run_task(&self, task: Notified, mut core: Box<Core>) -> RunResult {
        panic!("STUB: not implemented");
    }
    fn reset_lifo_enabled(&self, core: &mut Core) {
        panic!("STUB: not implemented");
    }
    fn assert_lifo_enabled_is_correct(&self, core: &Core) {
        panic!("STUB: not implemented");
    }
    fn maintenance(&self, mut core: Box<Core>) -> Box<Core> {
        panic!("STUB: not implemented");
    }
    /// Parks the worker thread while waiting for tasks to execute.
    ///
    /// This function checks if indeed there's no more work left to be done before parking.
    /// Also important to notice that, before parking, the worker thread will try to take
    /// ownership of the Driver (IO/Time) and dispatch any events that might have fired.
    /// Whenever a worker thread executes the Driver loop, all waken tasks are scheduled
    /// in its own local queue until the queue saturates (ntasks > `LOCAL_QUEUE_CAPACITY`).
    /// When the local queue is saturated, the overflow tasks are added to the injection queue
    /// from where other workers can pick them up.
    /// Also, we rely on the workstealing algorithm to spread the tasks amongst workers
    /// after all the IOs get dispatched
    fn park(&self, mut core: Box<Core>) -> Box<Core> {
        panic!("STUB: not implemented");
    }
    fn park_yield(&self, core: Box<Core>) -> Box<Core> {
        panic!("STUB: not implemented");
    }
    fn park_internal(
        &self,
        mut core: Box<Core>,
        duration: Option<Duration>,
    ) -> Box<Core> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn defer(&self, waker: &Waker) {
        panic!("STUB: not implemented");
    }
    #[cfg(all(tokio_unstable, feature = "time"))]
    /// Maintain local timers before parking the resource driver.
    ///
    /// * Remove cancelled timers from the local timer wheel.
    /// * Register remote timers to the local timer wheel.
    /// * Adjust the park duration based on
    ///   * the next timer expiration time.
    ///   * whether auto-advancing is required (feature = "test-util").
    ///
    /// # Returns
    ///
    /// `(Box<Core>, park_duration, auto_advance_duration)`
    fn maintain_local_timers_before_parking(
        &self,
        park_duration: Option<Duration>,
    ) -> MaintainLocalTimer {
        panic!("STUB: not implemented");
    }
    #[cfg(all(tokio_unstable, feature = "time"))]
    /// Maintain local timers after unparking the resource driver.
    ///
    /// * Auto-advance time, if required (feature = "test-util").
    /// * Process expired timers.
    fn maintain_local_timers_after_parking(
        &self,
        auto_advance_duration: Option<Duration>,
    ) {
        panic!("STUB: not implemented");
    }
    #[cfg(all(tokio_unstable, feature = "time"))]
    fn with_core<F, R>(&self, f: F) -> R
    where
        F: FnOnce(Option<&mut Core>) -> R,
    {
        panic!("STUB: not implemented");
    }
    #[cfg(all(tokio_unstable, feature = "time"))]
    pub(crate) fn with_time_temp_local_context<F, R>(&self, f: F) -> R
    where
        F: FnOnce(Option<time_alt::TempLocalContext<'_>>) -> R,
    {
        panic!("STUB: not implemented");
    }
    #[cfg(tokio_unstable)]
    pub(crate) fn worker_index(&self) -> usize {
        panic!("STUB: not implemented");
    }
}
impl Core {
    /// Increment the tick
    fn tick(&mut self) {
        panic!("STUB: not implemented");
    }
    /// Return the next notified task available to this worker.
    fn next_task(&mut self, worker: &Worker) -> Option<Notified> {
        panic!("STUB: not implemented");
    }
    fn next_local_task(&mut self) -> Option<Notified> {
        panic!("STUB: not implemented");
    }
    /// Function responsible for stealing tasks from another worker
    ///
    /// Note: Only if less than half the workers are searching for tasks to steal
    /// a new worker will actually try to steal. The idea is to make sure not all
    /// workers will be trying to steal at the same time.
    fn steal_work(&mut self, worker: &Worker) -> Option<Notified> {
        panic!("STUB: not implemented");
    }
    fn transition_to_searching(&mut self, worker: &Worker) -> bool {
        panic!("STUB: not implemented");
    }
    fn transition_from_searching(&mut self, worker: &Worker) -> bool {
        panic!("STUB: not implemented");
    }
    fn has_tasks(&self) -> bool {
        panic!("STUB: not implemented");
    }
    fn should_notify_others(&self) -> bool {
        panic!("STUB: not implemented");
    }
    /// Prepares the worker state for parking.
    ///
    /// Returns true if the transition happened, false if there is work to do first.
    fn transition_to_parked(&mut self, worker: &Worker) -> bool {
        panic!("STUB: not implemented");
    }
    /// Returns `true` if the transition happened.
    fn transition_from_parked(&mut self, worker: &Worker) -> bool {
        panic!("STUB: not implemented");
    }
    /// Runs maintenance work such as checking the pool's state.
    fn maintenance(&mut self, worker: &Worker) {
        panic!("STUB: not implemented");
    }
    /// Signals all tasks to shut down, and waits for them to complete. Must run
    /// before we enter the single-threaded phase of shutdown processing.
    fn pre_shutdown(&mut self, worker: &Worker) {
        panic!("STUB: not implemented");
    }
    /// Shuts down the core.
    fn shutdown(&mut self, handle: &Handle) {
        panic!("STUB: not implemented");
    }
    fn tune_global_queue_interval(&mut self, worker: &Worker) {
        panic!("STUB: not implemented");
    }
}
impl Worker {
    /// Returns a reference to the scheduler's injection queue.
    fn inject(&self) -> &inject::Shared<Arc<Handle>> {
        panic!("STUB: not implemented");
    }
}
impl Handle {
    pub(super) fn schedule_task(&self, task: Notified, is_yield: bool) {
        panic!("STUB: not implemented");
    }
    pub(super) fn schedule_option_task_without_yield(&self, task: Option<Notified>) {
        panic!("STUB: not implemented");
    }
    fn schedule_local(&self, core: &mut Core, task: Notified, is_yield: bool) {
        panic!("STUB: not implemented");
    }
    fn next_remote_task(&self) -> Option<Notified> {
        panic!("STUB: not implemented");
    }
    fn push_remote_task(&self, task: Notified) {
        panic!("STUB: not implemented");
    }
    #[cfg(all(tokio_unstable, feature = "time"))]
    pub(crate) fn push_remote_timer(&self, hdl: time_alt::EntryHandle) {
        panic!("STUB: not implemented");
    }
    #[cfg(all(tokio_unstable, feature = "time"))]
    pub(crate) fn take_remote_timers(&self) -> Vec<time_alt::EntryHandle> {
        panic!("STUB: not implemented");
    }
    pub(super) fn close(&self) {
        panic!("STUB: not implemented");
    }
    /// Notify a parked worker.
    ///
    /// Returns `true` if a worker was notified, `false` otherwise.
    fn notify_parked_local(&self) -> bool {
        panic!("STUB: not implemented");
    }
    fn notify_parked_remote(&self) {
        panic!("STUB: not implemented");
    }
    pub(super) fn notify_all(&self) {
        panic!("STUB: not implemented");
    }
    fn notify_if_work_pending(&self) {
        panic!("STUB: not implemented");
    }
    /// Returns `true` if another parked worker was notified, `false` otherwise.
    fn transition_worker_from_searching(&self) -> bool {
        panic!("STUB: not implemented");
    }
    /// Signals that a worker has observed the shutdown signal and has replaced
    /// its core back into its handle.
    ///
    /// If all workers have reached this point, the final cleanup is performed.
    fn shutdown_core(&self, core: Box<Core>) {
        panic!("STUB: not implemented");
    }
    fn ptr_eq(&self, other: &Handle) -> bool {
        panic!("STUB: not implemented");
    }
}
impl Overflow<Arc<Handle>> for Handle {
    fn push(&self, task: task::Notified<Arc<Handle>>) {
        panic!("STUB: not implemented");
    }
    fn push_batch<I>(&self, iter: I)
    where
        I: Iterator<Item = task::Notified<Arc<Handle>>>,
    {
        panic!("STUB: not implemented");
    }
}
pub(crate) struct InjectGuard<'a> {
    lock: crate::loom::sync::MutexGuard<'a, Synced>,
}
impl<'a> AsMut<inject::Synced> for InjectGuard<'a> {
    fn as_mut(&mut self) -> &mut inject::Synced {
        panic!("STUB: not implemented");
    }
}
impl<'a> Lock<inject::Synced> for &'a Handle {
    type Handle = InjectGuard<'a>;
    fn lock(self) -> Self::Handle {
        panic!("STUB: not implemented");
    }
}
#[cfg(all(tokio_unstable, feature = "time"))]
/// Returned by [`Context::maintain_local_timers_before_parking`].
struct MaintainLocalTimer {
    park_duration: Option<Duration>,
    auto_advance_duration: Option<Duration>,
}
#[track_caller]
fn with_current<R>(f: impl FnOnce(Option<&Context>) -> R) -> R {
    panic!("STUB: not implemented");
}
