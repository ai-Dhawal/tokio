//! Thread pool for blocking operations
use crate::loom::sync::{Arc, Condvar, Mutex};
use crate::loom::thread;
use crate::runtime::blocking::schedule::BlockingSchedule;
use crate::runtime::blocking::{shutdown, BlockingTask};
use crate::runtime::builder::ThreadNameFn;
use crate::runtime::task::{self, JoinHandle};
use crate::runtime::{Builder, Callback, Handle, BOX_FUTURE_THRESHOLD};
use crate::util::metric_atomics::MetricAtomicUsize;
use crate::util::trace::{blocking_task, SpawnMeta};
use std::collections::{HashMap, VecDeque};
use std::fmt;
use std::io;
use std::sync::atomic::Ordering;
use std::time::Duration;
pub(crate) struct BlockingPool {
    spawner: Spawner,
    shutdown_rx: shutdown::Receiver,
}
#[derive(Clone)]
pub(crate) struct Spawner {
    inner: Arc<Inner>,
}
#[derive(Default)]
pub(crate) struct SpawnerMetrics {
    num_threads: MetricAtomicUsize,
    num_idle_threads: MetricAtomicUsize,
    queue_depth: MetricAtomicUsize,
}
impl SpawnerMetrics {
    fn num_threads(&self) -> usize {
        panic!("STUB: not implemented");
    }
    fn num_idle_threads(&self) -> usize {
        panic!("STUB: not implemented");
    }
    cfg_unstable_metrics! {
        fn queue_depth(& self) -> usize { self.queue_depth.load(Ordering::Relaxed) }
    }
    fn inc_num_threads(&self) {
        panic!("STUB: not implemented");
    }
    fn dec_num_threads(&self) {
        panic!("STUB: not implemented");
    }
    fn inc_num_idle_threads(&self) {
        panic!("STUB: not implemented");
    }
    fn dec_num_idle_threads(&self) -> usize {
        panic!("STUB: not implemented");
    }
    fn inc_queue_depth(&self) {
        panic!("STUB: not implemented");
    }
    fn dec_queue_depth(&self) {
        panic!("STUB: not implemented");
    }
}
struct Inner {
    /// State shared between worker threads.
    shared: Mutex<Shared>,
    /// Pool threads wait on this.
    condvar: Condvar,
    /// Spawned threads use this name.
    thread_name: ThreadNameFn,
    /// Spawned thread stack size.
    stack_size: Option<usize>,
    /// Call after a thread starts.
    after_start: Option<Callback>,
    /// Call before a thread stops.
    before_stop: Option<Callback>,
    thread_cap: usize,
    keep_alive: Duration,
    metrics: SpawnerMetrics,
}
struct Shared {
    queue: VecDeque<Task>,
    num_notify: u32,
    shutdown: bool,
    shutdown_tx: Option<shutdown::Sender>,
    /// Prior to shutdown, we clean up `JoinHandles` by having each timed-out
    /// thread join on the previous timed-out thread. This is not strictly
    /// necessary but helps avoid Valgrind false positives, see
    /// <https://github.com/tokio-rs/tokio/commit/646fbae76535e397ef79dbcaacb945d4c829f666>
    /// for more information.
    last_exiting_thread: Option<thread::JoinHandle<()>>,
    /// This holds the `JoinHandles` for all running threads; on shutdown, the thread
    /// calling shutdown handles joining on these.
    worker_threads: HashMap<usize, thread::JoinHandle<()>>,
    /// This is a counter used to iterate `worker_threads` in a consistent order (for loom's
    /// benefit).
    worker_thread_index: usize,
}
pub(crate) struct Task {
    task: task::UnownedTask<BlockingSchedule>,
    mandatory: Mandatory,
}
#[derive(PartialEq, Eq)]
pub(crate) enum Mandatory {
    #[cfg_attr(not(feature = "fs"), allow(dead_code))]
    Mandatory,
    NonMandatory,
}
pub(crate) enum SpawnError {
    /// Pool is shutting down and the task was not scheduled
    ShuttingDown,
    /// There are no worker threads available to take the task
    /// and the OS failed to spawn a new one
    NoThreads(io::Error),
}
impl From<SpawnError> for io::Error {
    fn from(e: SpawnError) -> Self {
        panic!("STUB: not implemented");
    }
}
impl Task {
    pub(crate) fn new(
        task: task::UnownedTask<BlockingSchedule>,
        mandatory: Mandatory,
    ) -> Task {
        panic!("STUB: not implemented");
    }
    fn run(self) {
        panic!("STUB: not implemented");
    }
    fn shutdown_or_run_if_mandatory(self) {
        panic!("STUB: not implemented");
    }
}
const KEEP_ALIVE: Duration = Duration::from_secs(10);
/// Runs the provided function on an executor dedicated to blocking operations.
/// Tasks will be scheduled as non-mandatory, meaning they may not get executed
/// in case of runtime shutdown.
#[track_caller]
#[cfg_attr(target_os = "wasi", allow(dead_code))]
pub(crate) fn spawn_blocking<F, R>(func: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    panic!("STUB: not implemented");
}
cfg_fs! {
    #[cfg_attr(any(all(loom, not(test)), test), allow(dead_code))] #[doc =
    " Runs the provided function on an executor dedicated to blocking"] #[doc =
    " operations. Tasks will be scheduled as mandatory, meaning they are"] #[doc =
    " guaranteed to run unless a shutdown is already taking place. In case a"] #[doc =
    " shutdown is already taking place, `None` will be returned."] pub (crate) fn
    spawn_mandatory_blocking < F, R > (func : F) -> Option < JoinHandle < R >> where F :
    FnOnce() -> R + Send + 'static, R : Send + 'static, { let rt = Handle::current(); rt
    .inner.blocking_spawner().spawn_mandatory_blocking(& rt, func) }
}
impl BlockingPool {
    pub(crate) fn new(builder: &Builder, thread_cap: usize) -> BlockingPool {
        panic!("STUB: not implemented");
    }
    pub(crate) fn spawner(&self) -> &Spawner {
        panic!("STUB: not implemented");
    }
    pub(crate) fn shutdown(&mut self, timeout: Option<Duration>) {
        panic!("STUB: not implemented");
    }
}
impl Drop for BlockingPool {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
impl fmt::Debug for BlockingPool {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl Spawner {
    #[track_caller]
    pub(crate) fn spawn_blocking<F, R>(&self, rt: &Handle, func: F) -> JoinHandle<R>
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        panic!("STUB: not implemented");
    }
    cfg_fs! {
        #[track_caller] #[cfg_attr(any(all(loom, not(test)), test), allow(dead_code))]
        pub (crate) fn spawn_mandatory_blocking < F, R > (& self, rt : & Handle, func :
        F) -> Option < JoinHandle < R >> where F : FnOnce() -> R + Send + 'static, R :
        Send + 'static, { let fn_size = std::mem::size_of::< F > (); let (join_handle,
        spawn_result) = if fn_size > BOX_FUTURE_THRESHOLD { self
        .spawn_blocking_inner(Box::new(func), Mandatory::Mandatory,
        SpawnMeta::new_unnamed(fn_size), rt,) } else { self.spawn_blocking_inner(func,
        Mandatory::Mandatory, SpawnMeta::new_unnamed(fn_size), rt,) }; if spawn_result
        .is_ok() { Some(join_handle) } else { None } }
    }
    #[track_caller]
    pub(crate) fn spawn_blocking_inner<F, R>(
        &self,
        func: F,
        is_mandatory: Mandatory,
        spawn_meta: SpawnMeta<'_>,
        rt: &Handle,
    ) -> (JoinHandle<R>, Result<(), SpawnError>)
    where
        F: FnOnce() -> R + Send + 'static,
        R: Send + 'static,
    {
        panic!("STUB: not implemented");
    }
    fn spawn_task(&self, task: Task, rt: &Handle) -> Result<(), SpawnError> {
        panic!("STUB: not implemented");
    }
    fn spawn_thread(
        &self,
        shutdown_tx: shutdown::Sender,
        rt: &Handle,
        id: usize,
    ) -> io::Result<thread::JoinHandle<()>> {
        panic!("STUB: not implemented");
    }
}
cfg_unstable_metrics! {
    impl Spawner { pub (crate) fn num_threads(& self) -> usize { self.inner.metrics
    .num_threads() } pub (crate) fn num_idle_threads(& self) -> usize { self.inner
    .metrics.num_idle_threads() } pub (crate) fn queue_depth(& self) -> usize { self
    .inner.metrics.queue_depth() } }
}
#[inline]
fn is_temporary_os_thread_error(error: &io::Error) -> bool {
    panic!("STUB: not implemented");
}
impl Inner {
    fn run(&self, worker_thread_id: usize) {
        panic!("STUB: not implemented");
    }
}
impl fmt::Debug for Spawner {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
