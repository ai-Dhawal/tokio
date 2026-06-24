use crate::loom::sync::atomic::AtomicBool;
use crate::loom::sync::Arc;
use crate::runtime::driver::{self, Driver};
use crate::runtime::scheduler::{self, Defer, Inject};
use crate::runtime::task::{
    self, JoinHandle, LocalNotified, OwnedTasks, Schedule, SpawnLocation, Task,
    TaskHarnessScheduleHooks,
};
use crate::runtime::{
    blocking, context, Config, MetricsBatch, SchedulerMetrics, TaskHooks, TaskMeta,
    WorkerMetrics,
};
use crate::sync::notify::Notify;
use crate::util::atomic_cell::AtomicCell;
use crate::util::{waker_ref, RngSeedGenerator, Wake, WakerRef};
use std::cell::RefCell;
use std::collections::VecDeque;
use std::future::{poll_fn, Future};
use std::sync::atomic::Ordering::{AcqRel, Acquire, Release};
use std::task::Poll::{Pending, Ready};
use std::task::Waker;
use std::thread::ThreadId;
use std::time::Duration;
use std::time::Instant;
use std::{fmt, thread};
/// Executes tasks on the current thread
pub(crate) struct CurrentThread {
    /// Core scheduler data is acquired by a thread entering `block_on`.
    core: AtomicCell<Core>,
    /// Notifier for waking up other threads to steal the
    /// driver.
    notify: Notify,
}
/// Handle to the current thread scheduler
pub(crate) struct Handle {
    /// The name of the runtime
    name: Option<String>,
    /// Scheduler state shared across threads
    shared: Shared,
    /// Resource driver handles
    pub(crate) driver: driver::Handle,
    /// Blocking pool spawner
    pub(crate) blocking_spawner: blocking::Spawner,
    /// Current random number generator seed
    pub(crate) seed_generator: RngSeedGenerator,
    /// User-supplied hooks to invoke for things
    pub(crate) task_hooks: TaskHooks,
    /// If this is a `LocalRuntime`, flags the owning thread ID.
    pub(crate) local_tid: Option<ThreadId>,
}
/// Data required for executing the scheduler. The struct is passed around to
/// a function that will perform the scheduling work and acts as a capability token.
struct Core {
    /// Scheduler run queue
    tasks: VecDeque<Notified>,
    /// Current tick
    tick: u32,
    /// Runtime driver
    ///
    /// The driver is removed before starting to park the thread
    driver: Option<Driver>,
    /// Metrics batch
    metrics: MetricsBatch,
    /// How often to check the global queue
    global_queue_interval: u32,
    /// True if a task panicked without being handled and the runtime is
    /// configured to shutdown on unhandled panic.
    unhandled_panic: bool,
}
/// Scheduler state shared between threads.
struct Shared {
    /// Remote run queue
    inject: Inject<Arc<Handle>>,
    /// Collection of all active tasks spawned onto this executor.
    owned: OwnedTasks<Arc<Handle>>,
    /// Indicates whether the blocked on thread was woken.
    woken: AtomicBool,
    /// Scheduler configuration options
    config: Config,
    /// Keeps track of various runtime metrics.
    scheduler_metrics: SchedulerMetrics,
    /// This scheduler only has one worker.
    worker_metrics: WorkerMetrics,
    /// Startup time of this scheduler.
    ///
    /// This instant is used as the basis of task `scheduled_at` measurements.
    started_at: Option<Instant>,
}
/// Thread-local context.
///
/// pub(crate) to store in `runtime::context`.
pub(crate) struct Context {
    /// Scheduler handle
    handle: Arc<Handle>,
    /// Scheduler core, enabling the holder of `Context` to execute the
    /// scheduler.
    core: RefCell<Option<Box<Core>>>,
    /// Deferred tasks, usually ones that called `task::yield_now()`.
    pub(crate) defer: Defer,
}
type Notified = task::Notified<Arc<Handle>>;
/// Initial queue capacity.
const INITIAL_CAPACITY: usize = 64;
/// Used if none is specified. This is a temporary constant and will be removed
/// as we unify tuning logic between the multi-thread and current-thread
/// schedulers.
const DEFAULT_GLOBAL_QUEUE_INTERVAL: u32 = 31;
impl CurrentThread {
    pub(crate) fn new(
        driver: Driver,
        driver_handle: driver::Handle,
        blocking_spawner: blocking::Spawner,
        seed_generator: RngSeedGenerator,
        config: Config,
        local_tid: Option<ThreadId>,
        name: Option<String>,
    ) -> (CurrentThread, Arc<Handle>) {
        panic!("STUB: not implemented");
    }
    #[track_caller]
    pub(crate) fn block_on<F: Future>(
        &self,
        handle: &scheduler::Handle,
        future: F,
    ) -> F::Output {
        panic!("STUB: not implemented");
    }
    fn take_core(&self, handle: &Arc<Handle>) -> Option<CoreGuard<'_>> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn shutdown(&mut self, handle: &scheduler::Handle) {
        panic!("STUB: not implemented");
    }
}
fn shutdown2(mut core: Box<Core>, handle: &Handle) -> Box<Core> {
    panic!("STUB: not implemented");
}
impl fmt::Debug for CurrentThread {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl Core {
    /// Get and increment the current tick
    fn tick(&mut self) {
        panic!("STUB: not implemented");
    }
    fn next_task(&mut self, handle: &Handle) -> Option<Notified> {
        panic!("STUB: not implemented");
    }
    fn next_local_task(&mut self, handle: &Handle) -> Option<Notified> {
        panic!("STUB: not implemented");
    }
    fn push_task(&mut self, handle: &Handle, task: Notified) {
        panic!("STUB: not implemented");
    }
    fn submit_metrics(&mut self, handle: &Handle) {
        panic!("STUB: not implemented");
    }
}
#[cfg(feature = "taskdump")]
fn wake_deferred_tasks_and_free(context: &Context) {
    panic!("STUB: not implemented");
}
impl Context {
    /// Execute the closure with the given scheduler core stored in the
    /// thread-local context.
    fn run_task(
        &self,
        task: LocalNotified<Arc<Handle>>,
        mut core: Box<Core>,
    ) -> Box<Core> {
        panic!("STUB: not implemented");
    }
    /// Blocks the current thread until an event is received by the driver,
    /// including I/O events, timer events, ...
    fn park(&self, mut core: Box<Core>, handle: &Handle) -> Box<Core> {
        panic!("STUB: not implemented");
    }
    /// Checks the driver for new events without blocking the thread.
    fn park_yield(&self, mut core: Box<Core>, handle: &Handle) -> Box<Core> {
        panic!("STUB: not implemented");
    }
    fn has_pending_work(&self, core: &Core) -> bool {
        panic!("STUB: not implemented");
    }
    fn park_internal(
        &self,
        core: Box<Core>,
        handle: &Handle,
        driver: &mut Driver,
        duration: Option<Duration>,
    ) -> Box<Core> {
        panic!("STUB: not implemented");
    }
    fn enter<R>(&self, core: Box<Core>, f: impl FnOnce() -> R) -> (Box<Core>, R) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn defer(&self, waker: &Waker) {
        panic!("STUB: not implemented");
    }
}
impl Handle {
    /// Spawns a future onto the `CurrentThread` scheduler
    #[track_caller]
    pub(crate) fn spawn<F>(
        me: &Arc<Self>,
        future: F,
        id: crate::runtime::task::Id,
        spawned_at: SpawnLocation,
    ) -> JoinHandle<F::Output>
    where
        F: crate::future::Future + Send + 'static,
        F::Output: Send + 'static,
    {
        panic!("STUB: not implemented");
    }
    /// Spawn a task which isn't safe to send across thread boundaries onto the runtime.
    ///
    /// # Safety
    ///
    /// This should only be used when this is a `LocalRuntime` or in another case where the runtime
    /// provably cannot be driven from or moved to different threads from the one on which the task
    /// is spawned.
    #[track_caller]
    pub(crate) unsafe fn spawn_local<F>(
        me: &Arc<Self>,
        future: F,
        id: crate::runtime::task::Id,
        spawned_at: SpawnLocation,
    ) -> JoinHandle<F::Output>
    where
        F: crate::future::Future + 'static,
        F::Output: 'static,
    {
        panic!("STUB: not implemented");
    }
    /// Capture a snapshot of this runtime's state.
    #[cfg(
        all(
            tokio_unstable,
            feature = "taskdump",
            target_os = "linux",
            any(
                target_arch = "aarch64",
                target_arch = "x86",
                target_arch = "x86_64",
                target_arch = "s390x"
            )
        )
    )]
    pub(crate) fn dump(&self) -> crate::runtime::Dump {
        panic!("STUB: not implemented");
    }
    fn next_remote_task(&self) -> Option<Notified> {
        panic!("STUB: not implemented");
    }
    fn waker_ref(me: &Arc<Self>) -> WakerRef<'_> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn reset_woken(&self) -> bool {
        panic!("STUB: not implemented");
    }
    pub(crate) fn num_alive_tasks(&self) -> usize {
        panic!("STUB: not implemented");
    }
    pub(crate) fn injection_queue_depth(&self) -> usize {
        panic!("STUB: not implemented");
    }
    pub(crate) fn worker_metrics(&self, worker: usize) -> &WorkerMetrics {
        panic!("STUB: not implemented");
    }
}
cfg_unstable_metrics! {
    impl Handle { pub (crate) fn scheduler_metrics(& self) -> & SchedulerMetrics { & self
    .shared.scheduler_metrics } pub (crate) fn worker_local_queue_depth(& self, worker :
    usize) -> usize { self.worker_metrics(worker).queue_depth() } pub (crate) fn
    num_blocking_threads(& self) -> usize { self.blocking_spawner.num_threads() } pub
    (crate) fn num_idle_blocking_threads(& self) -> usize { self.blocking_spawner
    .num_idle_threads() } pub (crate) fn blocking_queue_depth(& self) -> usize { self
    .blocking_spawner.queue_depth() } cfg_64bit_metrics! { pub (crate) fn
    spawned_tasks_count(& self) -> u64 { self.shared.owned.spawned_tasks_count() } } }
}
use crate::runtime::metrics::ScheduleLatencyInstant;
use std::num::NonZeroU64;
impl Handle {
    pub(crate) fn owned_id(&self) -> NonZeroU64 {
        panic!("STUB: not implemented");
    }
    pub(crate) fn name(&self) -> Option<&str> {
        panic!("STUB: not implemented");
    }
}
impl fmt::Debug for Handle {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl Schedule for Arc<Handle> {
    fn release(&self, task: &Task<Self>) -> Option<Task<Self>> {
        panic!("STUB: not implemented");
    }
    fn schedule(&self, task: task::Notified<Self>) {
        panic!("STUB: not implemented");
    }
    fn hooks(&self) -> TaskHarnessScheduleHooks {
        panic!("STUB: not implemented");
    }
    cfg_unstable! {
        fn unhandled_panic(& self) { use crate ::runtime::UnhandledPanic; match self
        .shared.config.unhandled_panic { UnhandledPanic::Ignore => {}
        UnhandledPanic::ShutdownRuntime => { use scheduler::Context::CurrentThread;
        context::with_scheduler(| maybe_cx | match maybe_cx { Some(CurrentThread(cx)) if
        Arc::ptr_eq(self, & cx.handle) => { let mut core = cx.core.borrow_mut(); if let
        Some(core) = core.as_mut() { core.unhandled_panic = true; self.shared.owned
        .close_and_shutdown_all(0); } } _ =>
        unreachable!("runtime core not set in CURRENT thread-local"), }) } } }
    }
}
impl Wake for Handle {
    fn wake(arc_self: Arc<Self>) {
        panic!("STUB: not implemented");
    }
    /// Wake by reference
    fn wake_by_ref(arc_self: &Arc<Self>) {
        panic!("STUB: not implemented");
    }
}
/// Used to ensure we always place the `Core` value back into its slot in
/// `CurrentThread`, even if the future panics.
struct CoreGuard<'a> {
    context: scheduler::Context,
    scheduler: &'a CurrentThread,
}
impl CoreGuard<'_> {
    #[track_caller]
    fn block_on<F: Future>(self, future: F) -> F::Output {
        panic!("STUB: not implemented");
    }
    /// Enters the scheduler context. This sets the queue and other necessary
    /// scheduler state in the thread-local.
    fn enter<F, R>(self, f: F) -> R
    where
        F: FnOnce(Box<Core>, &Context) -> (Box<Core>, R),
    {
        panic!("STUB: not implemented");
    }
}
impl Drop for CoreGuard<'_> {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
