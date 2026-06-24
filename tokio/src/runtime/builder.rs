#![cfg_attr(loom, allow(unused_imports))]
use crate::runtime::handle::Handle;
use crate::runtime::{
    blocking, driver, Callback, HistogramBuilder, Runtime, TaskCallback, TimerFlavor,
};
#[cfg(tokio_unstable)]
use crate::runtime::{metrics::HistogramConfiguration, TaskMeta};
use crate::runtime::{LocalOptions, LocalRuntime};
use crate::util::rand::{RngSeed, RngSeedGenerator};
use crate::runtime::blocking::BlockingPool;
use crate::runtime::scheduler::CurrentThread;
use std::fmt;
use std::io;
use std::thread::ThreadId;
use std::time::Duration;
/// Builds Tokio Runtime with custom configuration values.
///
/// Methods can be chained in order to set the configuration values. The
/// Runtime is constructed by calling [`build`].
///
/// New instances of `Builder` are obtained via [`Builder::new_multi_thread`]
/// or [`Builder::new_current_thread`].
///
/// See function level documentation for details on the various configuration
/// settings.
///
/// [`build`]: method@Self::build
/// [`Builder::new_multi_thread`]: method@Self::new_multi_thread
/// [`Builder::new_current_thread`]: method@Self::new_current_thread
///
/// # Examples
///
/// ```
/// # #[cfg(not(target_family = "wasm"))]
/// # {
/// use tokio::runtime::Builder;
///
/// fn main() {
///     // build runtime
///     let runtime = Builder::new_multi_thread()
///         .worker_threads(4)
///         .thread_name("my-custom-name")
///         .thread_stack_size(3 * 1024 * 1024)
///         .build()
///         .unwrap();
///
///     // use runtime ...
/// }
/// # }
/// ```
pub struct Builder {
    /// Runtime type
    kind: Kind,
    /// Name of the runtime.
    name: Option<String>,
    /// Whether or not to enable the I/O driver
    enable_io: bool,
    nevents: usize,
    /// Whether or not to enable the time driver
    enable_time: bool,
    /// Whether or not the clock should start paused.
    start_paused: bool,
    /// The number of worker threads, used by Runtime.
    ///
    /// Only used when not using the current-thread executor.
    worker_threads: Option<usize>,
    /// Cap on thread usage.
    max_blocking_threads: usize,
    /// Name fn used for threads spawned by the runtime.
    pub(super) thread_name: ThreadNameFn,
    /// Stack size used for threads spawned by the runtime.
    pub(super) thread_stack_size: Option<usize>,
    /// Callback to run after each thread starts.
    pub(super) after_start: Option<Callback>,
    /// To run before each worker thread stops
    pub(super) before_stop: Option<Callback>,
    /// To run before each worker thread is parked.
    pub(super) before_park: Option<Callback>,
    /// To run after each thread is unparked.
    pub(super) after_unpark: Option<Callback>,
    /// To run before each task is spawned.
    pub(super) before_spawn: Option<TaskCallback>,
    /// To run before each poll
    #[cfg(tokio_unstable)]
    pub(super) before_poll: Option<TaskCallback>,
    /// To run after each poll
    #[cfg(tokio_unstable)]
    pub(super) after_poll: Option<TaskCallback>,
    /// To run after each task is terminated.
    pub(super) after_termination: Option<TaskCallback>,
    /// Customizable keep alive timeout for `BlockingPool`
    pub(super) keep_alive: Option<Duration>,
    /// How many ticks before pulling a task from the global/remote queue?
    ///
    /// When `None`, the value is unspecified and behavior details are left to
    /// the scheduler. Each scheduler flavor could choose to either pick its own
    /// default value or use some other strategy to decide when to poll from the
    /// global queue. For example, the multi-threaded scheduler uses a
    /// self-tuning strategy based on mean task poll times.
    pub(super) global_queue_interval: Option<u32>,
    /// How many ticks before yielding to the driver for timer and I/O events?
    pub(super) event_interval: u32,
    /// When true, the multi-threade scheduler LIFO slot should not be used.
    ///
    /// This option should only be exposed as unstable.
    pub(super) disable_lifo_slot: bool,
    /// Specify a random number generator seed to provide deterministic results
    pub(super) seed_generator: RngSeedGenerator,
    /// When true, enables task poll count histogram instrumentation.
    pub(super) metrics_poll_count_histogram_enable: bool,
    /// Configures the task poll count histogram
    pub(super) metrics_poll_count_histogram: HistogramBuilder,
    /// When true, enables task schedule latency instrumentation.
    pub(super) metrics_schedule_latency_histogram_enabled: bool,
    /// Configures the task schedule latency histogram.
    pub(super) metrics_schedule_latency_histogram: HistogramBuilder,
    #[cfg(tokio_unstable)]
    pub(super) unhandled_panic: UnhandledPanic,
    timer_flavor: TimerFlavor,
    /// Whether or not to enable eager hand-off for the I/O and time drivers (in
    /// `tokio_unstable`).
    enable_eager_driver_handoff: bool,
}
cfg_unstable! {
    #[doc = " How the runtime should respond to unhandled panics."] #[doc = ""] #[doc =
    " Instances of `UnhandledPanic` are passed to `Builder::unhandled_panic`"] #[doc =
    " to configure the runtime behavior when a spawned task panics."] #[doc = ""] #[doc =
    " See [`Builder::unhandled_panic`] for more details."] #[derive(Debug, Clone)]
    #[non_exhaustive] pub enum UnhandledPanic { #[doc =
    " The runtime should ignore panics on spawned tasks."] #[doc = ""] #[doc =
    " The panic is forwarded to the task's [`JoinHandle`] and all spawned"] #[doc =
    " tasks continue running normally."] #[doc = ""] #[doc =
    " This is the default behavior."] #[doc = ""] #[doc = " # Examples"] #[doc = ""]
    #[doc = " ```"] #[doc = " # #[cfg(not(target_family = \"wasm\"))]"] #[doc = " # {"]
    #[doc = " use tokio::runtime::{self, UnhandledPanic};"] #[doc = ""] #[doc =
    " # pub fn main() {"] #[doc = " let rt = runtime::Builder::new_current_thread()"]
    #[doc = "     .unhandled_panic(UnhandledPanic::Ignore)"] #[doc = "     .build()"]
    #[doc = "     .unwrap();"] #[doc = ""] #[doc =
    " let task1 = rt.spawn(async { panic!(\"boom\"); });"] #[doc =
    " let task2 = rt.spawn(async {"] #[doc = "     // This task completes normally"]
    #[doc = "     \"done\""] #[doc = " });"] #[doc = ""] #[doc = " rt.block_on(async {"]
    #[doc = "     // The panic on the first task is forwarded to the `JoinHandle`"] #[doc
    = "     assert!(task1.await.is_err());"] #[doc = ""] #[doc =
    "     // The second task completes normally"] #[doc =
    "     assert!(task2.await.is_ok());"] #[doc = " })"] #[doc = " # }"] #[doc = " # }"]
    #[doc = " ```"] #[doc = ""] #[doc =
    " [`JoinHandle`]: struct@crate::task::JoinHandle"] Ignore, #[doc =
    " The runtime should immediately shutdown if a spawned task panics."] #[doc = ""]
    #[doc = " The runtime will immediately shutdown even if the panicked task's"] #[doc =
    " [`JoinHandle`] is still available. All further spawned tasks will be"] #[doc =
    " immediately dropped and call to [`Runtime::block_on`] will panic."] #[doc = ""]
    #[doc = " # Examples"] #[doc = ""] #[doc = " ```should_panic"] #[doc =
    " use tokio::runtime::{self, UnhandledPanic};"] #[doc = ""] #[doc =
    " # pub fn main() {"] #[doc = " let rt = runtime::Builder::new_current_thread()"]
    #[doc = "     .unhandled_panic(UnhandledPanic::ShutdownRuntime)"] #[doc =
    "     .build()"] #[doc = "     .unwrap();"] #[doc = ""] #[doc =
    " rt.spawn(async { panic!(\"boom\"); });"] #[doc = " rt.spawn(async {"] #[doc =
    "     // This task never completes."] #[doc = " });"] #[doc = ""] #[doc =
    " rt.block_on(async {"] #[doc = "     // Do some work"] #[doc =
    " # loop { tokio::task::yield_now().await; }"] #[doc = " })"] #[doc = " # }"] #[doc =
    " ```"] #[doc = ""] #[doc = " [`JoinHandle`]: struct@crate::task::JoinHandle"]
    ShutdownRuntime, }
}
pub(crate) type ThreadNameFn = std::sync::Arc<
    dyn Fn() -> String + Send + Sync + 'static,
>;
#[derive(Clone, Copy)]
pub(crate) enum Kind {
    CurrentThread,
    #[cfg(feature = "rt-multi-thread")]
    MultiThread,
}
impl Builder {
    /// Returns a new builder with the current thread scheduler selected.
    ///
    /// Configuration methods can be chained on the return value.
    ///
    /// To spawn non-`Send` tasks on the resulting runtime, combine it with a
    /// [`LocalSet`], or call [`build_local`] to create a [`LocalRuntime`].
    ///
    /// [`LocalSet`]: crate::task::LocalSet
    /// [`LocalRuntime`]: crate::runtime::LocalRuntime
    /// [`build_local`]: crate::runtime::Builder::build_local
    pub fn new_current_thread() -> Builder {
        panic!("STUB: not implemented");
    }
    /// Returns a new builder with the multi thread scheduler selected.
    ///
    /// Configuration methods can be chained on the return value.
    #[cfg(feature = "rt-multi-thread")]
    #[cfg_attr(docsrs, doc(cfg(feature = "rt-multi-thread")))]
    pub fn new_multi_thread() -> Builder {
        panic!("STUB: not implemented");
    }
    /// Returns a new runtime builder initialized with default configuration
    /// values.
    ///
    /// Configuration methods can be chained on the return value.
    pub(crate) fn new(kind: Kind, event_interval: u32) -> Builder {
        panic!("STUB: not implemented");
    }
    /// Enables both I/O and time drivers.
    ///
    /// Doing this is a shorthand for calling `enable_io` and `enable_time`
    /// individually. If additional components are added to Tokio in the future,
    /// `enable_all` will include these future components.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// use tokio::runtime;
    ///
    /// let rt = runtime::Builder::new_multi_thread()
    ///     .enable_all()
    ///     .build()
    ///     .unwrap();
    /// # }
    /// ```
    pub fn enable_all(&mut self) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Enables the alternative timer implementation, which is disabled by default.
    ///
    /// The alternative timer implementation is an unstable feature that may
    /// provide better performance on multi-threaded runtimes with a large number
    /// of worker threads.
    ///
    /// This option only applies to multi-threaded runtimes. Attempting to use
    /// this option with any other runtime type will have no effect.
    ///
    /// [Click here to share your experience with the alternative timer](https://github.com/tokio-rs/tokio/issues/7745)
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// use tokio::runtime;
    ///
    /// let rt = runtime::Builder::new_multi_thread()
    ///   .enable_alt_timer()
    ///   .build()
    ///   .unwrap();
    /// # }
    /// ```
    #[cfg(all(tokio_unstable, feature = "time", feature = "rt-multi-thread"))]
    #[cfg_attr(
        docsrs,
        doc(cfg(all(tokio_unstable, feature = "time", feature = "rt-multi-thread")))
    )]
    pub fn enable_alt_timer(&mut self) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Enable eager hand-off of the I/O and time drivers for multi-threaded
    /// runtimes, which is disabled by default.
    ///
    /// When this option is enabled, a worker thread which has parked on the I/O
    /// or time driver will notify another worker thread once it is preparing to
    /// begin polling a task from the run queue, so that the notified worker can
    /// begin polling the I/O or time driver. This can reduce the latency with
    /// which I/O and timer notifications are processed, especially when some
    /// tasks have polls that take a long time to complete. In addition, it can
    /// reduce the risk of a deadlock which may occur when a task blocks the
    /// worker thread which is holding the I/O or time driver until some other
    /// task, which is waiting for a notification from *that* driver, unblocks
    /// it.
    ///
    /// This option is disabled by default, as enabling it may potentially
    /// increase contention due to extra synchronization in cross-driver
    /// wakeups.
    ///
    /// This option only applies to multi-threaded runtimes. Attempting to use
    /// this option with any other runtime type will have no effect.
    ///
    /// **Note**: This is an [unstable API][unstable]. Eager driver hand-off is
    /// an experimental feature whose behavior may be removed or changed in 1.x
    /// releases. See [the documentation on unstable features][unstable] for
    /// details.
    ///
    /// [unstable]: crate#unstable-features
    #[cfg(all(tokio_unstable, feature = "rt-multi-thread"))]
    #[cfg_attr(docsrs, doc(cfg(all(tokio_unstable, feature = "rt-multi-thread"))))]
    pub fn enable_eager_driver_handoff(&mut self) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Sets the number of worker threads the `Runtime` will use.
    ///
    /// This can be any number above 0 though it is advised to keep this value
    /// on the smaller side.
    ///
    /// This will override the value read from environment variable `TOKIO_WORKER_THREADS`.
    ///
    /// # Default
    ///
    /// The default value is the number of cores available to the system.
    ///
    /// When using the `current_thread` runtime this method has no effect.
    ///
    /// # Examples
    ///
    /// ## Multi threaded runtime with 4 threads
    ///
    /// ```
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// use tokio::runtime;
    ///
    /// // This will spawn a work-stealing runtime with 4 worker threads.
    /// let rt = runtime::Builder::new_multi_thread()
    ///     .worker_threads(4)
    ///     .build()
    ///     .unwrap();
    ///
    /// rt.spawn(async move {});
    /// # }
    /// ```
    ///
    /// ## Current thread runtime (will only run on the current thread via `Runtime::block_on`)
    ///
    /// ```
    /// use tokio::runtime;
    ///
    /// // Create a runtime that _must_ be driven from a call
    /// // to `Runtime::block_on`.
    /// let rt = runtime::Builder::new_current_thread()
    ///     .build()
    ///     .unwrap();
    ///
    /// // This will run the runtime and future on the current thread
    /// rt.block_on(async move {});
    /// ```
    ///
    /// # Panics
    ///
    /// This will panic if `val` is not larger than `0`.
    #[track_caller]
    pub fn worker_threads(&mut self, val: usize) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Specifies the limit for additional threads spawned by the Runtime.
    ///
    /// These threads are used for blocking operations like tasks spawned
    /// through [`spawn_blocking`], this includes but is not limited to:
    /// - [`fs`] operations
    /// - dns resolution through [`ToSocketAddrs`]
    /// - writing to [`Stdout`] or [`Stderr`]
    /// - reading from [`Stdin`]
    ///
    /// Unlike the [`worker_threads`], they are not always active and will exit
    /// if left idle for too long. You can change this timeout duration with [`thread_keep_alive`].
    ///
    /// It's recommended to not set this limit too low in order to avoid hanging on operations
    /// requiring [`spawn_blocking`].
    ///
    /// The default value is 512.
    ///
    /// # Queue Behavior
    ///
    /// When a blocking task is submitted, it will be inserted into a queue. If available, one of
    /// the idle threads will be notified to run the task. Otherwise, if the threshold set by this
    /// method has not been reached, a new thread will be spawned. If no idle thread is available
    /// and no more threads are allowed to be spawned, the task will remain in the queue until one
    /// of the busy threads pick it up. Note that since the queue does not apply any backpressure,
    /// it could potentially grow unbounded.
    ///
    /// # Panics
    ///
    /// This will panic if `val` is not larger than `0`.
    ///
    /// # Upgrading from 0.x
    ///
    /// In old versions `max_threads` limited both blocking and worker threads, but the
    /// current `max_blocking_threads` does not include async worker threads in the count.
    ///
    /// [`spawn_blocking`]: fn@crate::task::spawn_blocking
    /// [`fs`]: mod@crate::fs
    /// [`ToSocketAddrs`]: trait@crate::net::ToSocketAddrs
    /// [`Stdout`]: struct@crate::io::Stdout
    /// [`Stdin`]: struct@crate::io::Stdin
    /// [`Stderr`]: struct@crate::io::Stderr
    /// [`worker_threads`]: Self::worker_threads
    /// [`thread_keep_alive`]: Self::thread_keep_alive
    #[track_caller]
    #[cfg_attr(docsrs, doc(alias = "max_threads"))]
    pub fn max_blocking_threads(&mut self, val: usize) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Sets name of threads spawned by the `Runtime`'s thread pool.
    ///
    /// The default name is "tokio-rt-worker".
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use tokio::runtime;
    ///
    /// # pub fn main() {
    /// let rt = runtime::Builder::new_multi_thread()
    ///     .thread_name("my-pool")
    ///     .build();
    /// # }
    /// # }
    /// ```
    pub fn thread_name(&mut self, val: impl Into<String>) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Sets the name of the runtime.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use tokio::runtime;
    ///
    /// # pub fn main() {
    /// let rt = runtime::Builder::new_multi_thread()
    ///     .name("my-runtime")
    ///     .build();
    /// # }
    /// # }
    /// ```
    /// # Panics
    ///
    /// This function will panic if an empty value is passed as an argument.
    ///
    #[track_caller]
    pub fn name(&mut self, val: impl Into<String>) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Sets a function used to generate the name of threads spawned by the `Runtime`'s thread pool.
    ///
    /// The default name fn is `|| "tokio-rt-worker".into()`.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use tokio::runtime;
    /// # use std::sync::atomic::{AtomicUsize, Ordering};
    /// # pub fn main() {
    /// let rt = runtime::Builder::new_multi_thread()
    ///     .thread_name_fn(|| {
    ///        static ATOMIC_ID: AtomicUsize = AtomicUsize::new(0);
    ///        let id = ATOMIC_ID.fetch_add(1, Ordering::SeqCst);
    ///        format!("my-pool-{}", id)
    ///     })
    ///     .build();
    /// # }
    /// # }
    /// ```
    pub fn thread_name_fn<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn() -> String + Send + Sync + 'static,
    {
        panic!("STUB: not implemented");
    }
    /// Sets the stack size (in bytes) for worker threads.
    ///
    /// The actual stack size may be greater than this value if the platform
    /// specifies minimal stack size.
    ///
    /// The default stack size for spawned threads is 2 MiB, though this
    /// particular stack size is subject to change in the future.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use tokio::runtime;
    ///
    /// # pub fn main() {
    /// let rt = runtime::Builder::new_multi_thread()
    ///     .thread_stack_size(32 * 1024)
    ///     .build();
    /// # }
    /// # }
    /// ```
    pub fn thread_stack_size(&mut self, val: usize) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Executes function `f` after each thread is started but before it starts
    /// doing work.
    ///
    /// This is intended for bookkeeping and monitoring use cases.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use tokio::runtime;
    /// # pub fn main() {
    /// let runtime = runtime::Builder::new_multi_thread()
    ///     .on_thread_start(|| {
    ///         println!("thread started");
    ///     })
    ///     .build();
    /// # }
    /// # }
    /// ```
    #[cfg(not(loom))]
    pub fn on_thread_start<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        panic!("STUB: not implemented");
    }
    /// Executes function `f` before each thread stops.
    ///
    /// This is intended for bookkeeping and monitoring use cases.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(not(target_family = "wasm"))]
    /// {
    /// # use tokio::runtime;
    /// # pub fn main() {
    /// let runtime = runtime::Builder::new_multi_thread()
    ///     .on_thread_stop(|| {
    ///         println!("thread stopping");
    ///     })
    ///     .build();
    /// # }
    /// # }
    /// ```
    #[cfg(not(loom))]
    pub fn on_thread_stop<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        panic!("STUB: not implemented");
    }
    /// Executes function `f` just before a thread is parked (goes idle).
    /// `f` is called within the Tokio context, so functions like [`tokio::spawn`](crate::spawn)
    /// can be called, and may result in this thread being unparked immediately.
    ///
    /// This can be used to start work only when the executor is idle, or for bookkeeping
    /// and monitoring purposes.
    ///
    /// Note: There can only be one park callback for a runtime; calling this function
    /// more than once replaces the last callback defined, rather than adding to it.
    ///
    /// # Examples
    ///
    /// ## Multithreaded executor
    /// ```
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use std::sync::Arc;
    /// # use std::sync::atomic::{AtomicBool, Ordering};
    /// # use tokio::runtime;
    /// # use tokio::sync::Barrier;
    /// # pub fn main() {
    /// let once = AtomicBool::new(true);
    /// let barrier = Arc::new(Barrier::new(2));
    ///
    /// let runtime = runtime::Builder::new_multi_thread()
    ///     .worker_threads(1)
    ///     .on_thread_park({
    ///         let barrier = barrier.clone();
    ///         move || {
    ///             let barrier = barrier.clone();
    ///             if once.swap(false, Ordering::Relaxed) {
    ///                 tokio::spawn(async move { barrier.wait().await; });
    ///            }
    ///         }
    ///     })
    ///     .build()
    ///     .unwrap();
    ///
    /// runtime.block_on(async {
    ///    barrier.wait().await;
    /// })
    /// # }
    /// # }
    /// ```
    /// ## Current thread executor
    /// ```
    /// # use std::sync::Arc;
    /// # use std::sync::atomic::{AtomicBool, Ordering};
    /// # use tokio::runtime;
    /// # use tokio::sync::Barrier;
    /// # pub fn main() {
    /// let once = AtomicBool::new(true);
    /// let barrier = Arc::new(Barrier::new(2));
    ///
    /// let runtime = runtime::Builder::new_current_thread()
    ///     .on_thread_park({
    ///         let barrier = barrier.clone();
    ///         move || {
    ///             let barrier = barrier.clone();
    ///             if once.swap(false, Ordering::Relaxed) {
    ///                 tokio::spawn(async move { barrier.wait().await; });
    ///            }
    ///         }
    ///     })
    ///     .build()
    ///     .unwrap();
    ///
    /// runtime.block_on(async {
    ///    barrier.wait().await;
    /// })
    /// # }
    /// ```
    #[cfg(not(loom))]
    pub fn on_thread_park<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        panic!("STUB: not implemented");
    }
    /// Executes function `f` just after a thread unparks (starts executing tasks).
    ///
    /// This is intended for bookkeeping and monitoring use cases; note that work
    /// in this callback will increase latencies when the application has allowed one or
    /// more runtime threads to go idle.
    ///
    /// Note: There can only be one unpark callback for a runtime; calling this function
    /// more than once replaces the last callback defined, rather than adding to it.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use tokio::runtime;
    /// # pub fn main() {
    /// let runtime = runtime::Builder::new_multi_thread()
    ///     .on_thread_unpark(|| {
    ///         println!("thread unparking");
    ///     })
    ///     .build();
    ///
    /// runtime.unwrap().block_on(async {
    ///    tokio::task::yield_now().await;
    ///    println!("Hello from Tokio!");
    /// })
    /// # }
    /// # }
    /// ```
    #[cfg(not(loom))]
    pub fn on_thread_unpark<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        panic!("STUB: not implemented");
    }
    /// Executes function `f` just before a task is spawned.
    ///
    /// `f` is called within the Tokio context, so functions like
    /// [`tokio::spawn`](crate::spawn) can be called, and may result in this callback being
    /// invoked immediately.
    ///
    /// This can be used for bookkeeping or monitoring purposes.
    ///
    /// Note: There can only be one spawn callback for a runtime; calling this function more
    /// than once replaces the last callback defined, rather than adding to it.
    ///
    /// This *does not* support [`LocalSet`](crate::task::LocalSet) at this time.
    ///
    /// **Note**: This is an [unstable API][unstable]. The public API of this type
    /// may break in 1.x releases. See [the documentation on unstable
    /// features][unstable] for details.
    ///
    /// [unstable]: crate#unstable-features
    ///
    /// # Examples
    ///
    /// ```
    /// # use tokio::runtime;
    /// # pub fn main() {
    /// let runtime = runtime::Builder::new_current_thread()
    ///     .on_task_spawn(|_| {
    ///         println!("spawning task");
    ///     })
    ///     .build()
    ///     .unwrap();
    ///
    /// runtime.block_on(async {
    ///     tokio::task::spawn(std::future::ready(()));
    ///
    ///     for _ in 0..64 {
    ///         tokio::task::yield_now().await;
    ///     }
    /// })
    /// # }
    /// ```
    #[cfg(all(not(loom), tokio_unstable))]
    #[cfg_attr(docsrs, doc(cfg(tokio_unstable)))]
    pub fn on_task_spawn<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn(&TaskMeta<'_>) + Send + Sync + 'static,
    {
        panic!("STUB: not implemented");
    }
    /// Executes function `f` just before a task is polled
    ///
    /// `f` is called within the Tokio context, so functions like
    /// [`tokio::spawn`](crate::spawn) can be called, and may result in this callback being
    /// invoked immediately.
    ///
    /// **Note**: This is an [unstable API][unstable]. The public API of this type
    /// may break in 1.x releases. See [the documentation on unstable
    /// features][unstable] for details.
    ///
    /// [unstable]: crate#unstable-features
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use std::sync::{atomic::AtomicUsize, Arc};
    /// # use tokio::task::yield_now;
    /// # pub fn main() {
    /// let poll_start_counter = Arc::new(AtomicUsize::new(0));
    /// let poll_start = poll_start_counter.clone();
    /// let rt = tokio::runtime::Builder::new_multi_thread()
    ///     .enable_all()
    ///     .on_before_task_poll(move |meta| {
    ///         println!("task {} is about to be polled", meta.id())
    ///     })
    ///     .build()
    ///     .unwrap();
    /// let task = rt.spawn(async {
    ///     yield_now().await;
    /// });
    /// let _ = rt.block_on(task);
    ///
    /// # }
    /// # }
    /// ```
    #[cfg(tokio_unstable)]
    #[cfg_attr(docsrs, doc(cfg(tokio_unstable)))]
    pub fn on_before_task_poll<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn(&TaskMeta<'_>) + Send + Sync + 'static,
    {
        panic!("STUB: not implemented");
    }
    /// Executes function `f` just after a task is polled
    ///
    /// `f` is called within the Tokio context, so functions like
    /// [`tokio::spawn`](crate::spawn) can be called, and may result in this callback being
    /// invoked immediately.
    ///
    /// **Note**: This is an [unstable API][unstable]. The public API of this type
    /// may break in 1.x releases. See [the documentation on unstable
    /// features][unstable] for details.
    ///
    /// [unstable]: crate#unstable-features
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use std::sync::{atomic::AtomicUsize, Arc};
    /// # use tokio::task::yield_now;
    /// # pub fn main() {
    /// let poll_stop_counter = Arc::new(AtomicUsize::new(0));
    /// let poll_stop = poll_stop_counter.clone();
    /// let rt = tokio::runtime::Builder::new_multi_thread()
    ///     .enable_all()
    ///     .on_after_task_poll(move |meta| {
    ///         println!("task {} completed polling", meta.id());
    ///     })
    ///     .build()
    ///     .unwrap();
    /// let task = rt.spawn(async {
    ///     yield_now().await;
    /// });
    /// let _ = rt.block_on(task);
    ///
    /// # }
    /// # }
    /// ```
    #[cfg(tokio_unstable)]
    #[cfg_attr(docsrs, doc(cfg(tokio_unstable)))]
    pub fn on_after_task_poll<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn(&TaskMeta<'_>) + Send + Sync + 'static,
    {
        panic!("STUB: not implemented");
    }
    /// Executes function `f` just after a task is terminated.
    ///
    /// `f` is called within the Tokio context, so functions like
    /// [`tokio::spawn`](crate::spawn) can be called.
    ///
    /// This can be used for bookkeeping or monitoring purposes.
    ///
    /// Note: There can only be one task termination callback for a runtime; calling this
    /// function more than once replaces the last callback defined, rather than adding to it.
    ///
    /// This *does not* support [`LocalSet`](crate::task::LocalSet) at this time.
    ///
    /// **Note**: This is an [unstable API][unstable]. The public API of this type
    /// may break in 1.x releases. See [the documentation on unstable
    /// features][unstable] for details.
    ///
    /// [unstable]: crate#unstable-features
    ///
    /// # Examples
    ///
    /// ```
    /// # use tokio::runtime;
    /// # pub fn main() {
    /// let runtime = runtime::Builder::new_current_thread()
    ///     .on_task_terminate(|_| {
    ///         println!("killing task");
    ///     })
    ///     .build()
    ///     .unwrap();
    ///
    /// runtime.block_on(async {
    ///     tokio::task::spawn(std::future::ready(()));
    ///
    ///     for _ in 0..64 {
    ///         tokio::task::yield_now().await;
    ///     }
    /// })
    /// # }
    /// ```
    #[cfg(all(not(loom), tokio_unstable))]
    #[cfg_attr(docsrs, doc(cfg(tokio_unstable)))]
    pub fn on_task_terminate<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn(&TaskMeta<'_>) + Send + Sync + 'static,
    {
        panic!("STUB: not implemented");
    }
    /// Creates the configured `Runtime`.
    ///
    /// The returned `Runtime` instance is ready to spawn tasks.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// use tokio::runtime::Builder;
    ///
    /// let rt  = Builder::new_multi_thread().build().unwrap();
    ///
    /// rt.block_on(async {
    ///     println!("Hello from the Tokio runtime");
    /// });
    /// # }
    /// ```
    pub fn build(&mut self) -> io::Result<Runtime> {
        panic!("STUB: not implemented");
    }
    /// Creates the configured [`LocalRuntime`].
    ///
    /// The returned [`LocalRuntime`] instance is ready to spawn tasks.
    ///
    /// # Panics
    ///
    /// This will panic if the runtime is configured with [`new_multi_thread()`].
    ///
    /// [`new_multi_thread()`]: Builder::new_multi_thread
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::runtime::{Builder, LocalOptions};
    ///
    /// let rt = Builder::new_current_thread()
    ///     .build_local(LocalOptions::default())
    ///     .unwrap();
    ///
    /// rt.spawn_local(async {
    ///     println!("Hello from the Tokio runtime");
    /// });
    /// ```
    #[allow(unused_variables, unreachable_patterns)]
    pub fn build_local(&mut self, options: LocalOptions) -> io::Result<LocalRuntime> {
        panic!("STUB: not implemented");
    }
    fn get_cfg(&self) -> driver::Cfg {
        panic!("STUB: not implemented");
    }
    /// Sets a custom timeout for a thread in the blocking pool.
    ///
    /// By default, the timeout for a thread is set to 10 seconds. This can
    /// be overridden using `.thread_keep_alive()`.
    ///
    /// # Example
    ///
    /// ```
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use tokio::runtime;
    /// # use std::time::Duration;
    /// # pub fn main() {
    /// let rt = runtime::Builder::new_multi_thread()
    ///     .thread_keep_alive(Duration::from_millis(100))
    ///     .build();
    /// # }
    /// # }
    /// ```
    pub fn thread_keep_alive(&mut self, duration: Duration) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Sets the number of scheduler ticks after which the scheduler will poll the global
    /// task queue.
    ///
    /// A scheduler "tick" roughly corresponds to one `poll` invocation on a task.
    ///
    /// By default the global queue interval is 31 for the current-thread scheduler. Please see
    /// [the module documentation] for the default behavior of the multi-thread scheduler.
    ///
    /// Schedulers have a local queue of already-claimed tasks, and a global queue of incoming
    /// tasks. Setting the interval to a smaller value increases the fairness of the scheduler,
    /// at the cost of more synchronization overhead. That can be beneficial for prioritizing
    /// getting started on new work, especially if tasks frequently yield rather than complete
    /// or await on further I/O. Setting the interval to `1` will prioritize the global queue and
    /// tasks from the local queue will be executed only if the global queue is empty.
    /// Conversely, a higher value prioritizes existing work, and is a good choice when most
    /// tasks quickly complete polling.
    ///
    /// [the module documentation]: crate::runtime#multi-threaded-runtime-behavior-at-the-time-of-writing
    ///
    /// # Panics
    ///
    /// This function will panic if 0 is passed as an argument.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use tokio::runtime;
    /// # pub fn main() {
    /// let rt = runtime::Builder::new_multi_thread()
    ///     .global_queue_interval(31)
    ///     .build();
    /// # }
    /// # }
    /// ```
    #[track_caller]
    pub fn global_queue_interval(&mut self, val: u32) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Sets the number of scheduler ticks after which the scheduler will poll for
    /// external events (timers, I/O, and so on).
    ///
    /// A scheduler "tick" roughly corresponds to one `poll` invocation on a task.
    ///
    /// By default, the event interval is `61` for all scheduler types.
    ///
    /// Setting the event interval determines the effective "priority" of delivering
    /// these external events (which may wake up additional tasks), compared to
    /// executing tasks that are currently ready to run. A smaller value is useful
    /// when tasks frequently spend a long time in polling, or infrequently yield,
    /// which can result in overly long delays picking up I/O events. Conversely,
    /// picking up new events requires extra synchronization and syscall overhead,
    /// so if tasks generally complete their polling quickly, a higher event interval
    /// will minimize that overhead while still keeping the scheduler responsive to
    /// events.
    ///
    /// # Panics
    ///
    /// This function will panic if 0 is passed as an argument.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// # use tokio::runtime;
    /// # pub fn main() {
    /// let rt = runtime::Builder::new_multi_thread()
    ///     .event_interval(31)
    ///     .build();
    /// # }
    /// # }
    /// ```
    #[track_caller]
    pub fn event_interval(&mut self, val: u32) -> &mut Self {
        panic!("STUB: not implemented");
    }
    cfg_unstable! {
        #[doc = " Configure how the runtime responds to an unhandled panic on a"] #[doc =
        " spawned task."] #[doc = ""] #[doc =
        " By default, an unhandled panic (i.e. a panic not caught by"] #[doc =
        " [`std::panic::catch_unwind`]) has no impact on the runtime's"] #[doc =
        " execution. The panic's error value is forwarded to the task's"] #[doc =
        " [`JoinHandle`] and all other spawned tasks continue running."] #[doc = ""]
        #[doc = " The `unhandled_panic` option enables configuring this behavior."] #[doc
        = ""] #[doc = " * `UnhandledPanic::Ignore` is the default behavior. Panics on"]
        #[doc = "   spawned tasks have no impact on the runtime's execution."] #[doc =
        " * `UnhandledPanic::ShutdownRuntime` will force the runtime to"] #[doc =
        "   shutdown immediately when a spawned task panics even if that"] #[doc =
        "   task's `JoinHandle` has not been dropped. All other spawned tasks"] #[doc =
        "   will immediately terminate and further calls to"] #[doc =
        "   [`Runtime::block_on`] will panic."] #[doc = ""] #[doc = " # Panics"] #[doc =
        " This method panics if called with [`UnhandledPanic::ShutdownRuntime`]"] #[doc =
        " on a runtime other than the current thread runtime."] #[doc = ""] #[doc =
        " # Unstable"] #[doc = ""] #[doc =
        " This option is currently unstable and its implementation is"] #[doc =
        " incomplete. The API may change or be removed in the future. See"] #[doc =
        " issue [tokio-rs/tokio#4516] for more details."] #[doc = ""] #[doc =
        " # Examples"] #[doc = ""] #[doc =
        " The following demonstrates a runtime configured to shutdown on"] #[doc =
        " panic. The first spawned task panics and results in the runtime"] #[doc =
        " shutting down. The second spawned task never has a chance to"] #[doc =
        " execute. The call to `block_on` will panic due to the runtime being"] #[doc =
        " forcibly shutdown."] #[doc = ""] #[doc = " ```should_panic"] #[doc =
        " use tokio::runtime::{self, UnhandledPanic};"] #[doc = ""] #[doc =
        " # pub fn main() {"] #[doc = " let rt = runtime::Builder::new_current_thread()"]
        #[doc = "     .unhandled_panic(UnhandledPanic::ShutdownRuntime)"] #[doc =
        "     .build()"] #[doc = "     .unwrap();"] #[doc = ""] #[doc =
        " rt.spawn(async { panic!(\"boom\"); });"] #[doc = " rt.spawn(async {"] #[doc =
        "     // This task never completes."] #[doc = " });"] #[doc = ""] #[doc =
        " rt.block_on(async {"] #[doc = "     // Do some work"] #[doc =
        " # loop { tokio::task::yield_now().await; }"] #[doc = " })"] #[doc = " # }"]
        #[doc = " ```"] #[doc = ""] #[doc =
        " [`JoinHandle`]: struct@crate::task::JoinHandle"] #[doc =
        " [tokio-rs/tokio#4516]: https://github.com/tokio-rs/tokio/issues/4516"] pub fn
        unhandled_panic(& mut self, behavior : UnhandledPanic) -> & mut Self { if !
        matches!(self.kind, Kind::CurrentThread) && matches!(behavior,
        UnhandledPanic::ShutdownRuntime) {
        panic!("UnhandledPanic::ShutdownRuntime is only supported in current thread runtime");
        } self.unhandled_panic = behavior; self } #[doc =
        " Disables the LIFO task scheduler heuristic."] #[doc = ""] #[doc =
        " The multi-threaded scheduler includes a heuristic for optimizing"] #[doc =
        " message-passing patterns. This heuristic results in the **last**"] #[doc =
        " scheduled task being polled first."] #[doc = ""] #[doc =
        " To implement this heuristic, each worker thread has a slot which"] #[doc =
        " holds the task that should be polled next. However, this slot cannot"] #[doc =
        " be stolen by other worker threads, which can result in lower total"] #[doc =
        " throughput when tasks tend to have longer poll times."] #[doc = ""] #[doc =
        " This configuration option will disable this heuristic resulting in"] #[doc =
        " all scheduled tasks being pushed into the worker-local queue, which"] #[doc =
        " is stealable."] #[doc = ""] #[doc =
        " Consider trying this option when the task \"scheduled\" time is high"] #[doc =
        " but the runtime is underutilized. Use [tokio-rs/tokio-metrics] to"] #[doc =
        " collect this data."] #[doc = ""] #[doc = " # Unstable"] #[doc = ""] #[doc =
        " This configuration option is considered a workaround for the LIFO"] #[doc =
        " slot not being stealable. When the slot becomes stealable, we will"] #[doc =
        " revisit whether or not this option is necessary. See"] #[doc =
        " issue [tokio-rs/tokio#4941]."] #[doc = ""] #[doc = " # Examples"] #[doc = ""]
        #[doc = " ```"] #[doc = " # #[cfg(not(target_family = \"wasm\"))]"] #[doc =
        " # {"] #[doc = " use tokio::runtime;"] #[doc = ""] #[doc =
        " let rt = runtime::Builder::new_multi_thread()"] #[doc =
        "     .disable_lifo_slot()"] #[doc = "     .build()"] #[doc = "     .unwrap();"]
        #[doc = " # }"] #[doc = " ```"] #[doc = ""] #[doc =
        " [tokio-rs/tokio-metrics]: https://github.com/tokio-rs/tokio-metrics"] #[doc =
        " [tokio-rs/tokio#4941]: https://github.com/tokio-rs/tokio/issues/4941"] pub fn
        disable_lifo_slot(& mut self) -> & mut Self { self.disable_lifo_slot = true; self
        } #[doc = " Specifies the random number generation seed to use within all"] #[doc
        = " threads associated with the runtime being built."] #[doc = ""] #[doc =
        " This option is intended to make certain parts of the runtime"] #[doc =
        " deterministic (e.g. the [`tokio::select!`] macro). In the case of"] #[doc =
        " [`tokio::select!`] it will ensure that the order that branches are"] #[doc =
        " polled is deterministic."] #[doc = ""] #[doc =
        " In addition to the code specifying `rng_seed` and interacting with"] #[doc =
        " the runtime, the internals of Tokio and the Rust compiler may affect"] #[doc =
        " the sequences of random numbers. In order to ensure repeatable"] #[doc =
        " results, the version of Tokio, the versions of all other"] #[doc =
        " dependencies that interact with Tokio, and the Rust compiler version"] #[doc =
        " should also all remain constant."] #[doc = ""] #[doc = " # Examples"] #[doc =
        ""] #[doc = " ```"] #[doc = " # use tokio::runtime::{self, RngSeed};"] #[doc =
        " # pub fn main() {"] #[doc =
        " let seed = RngSeed::from_bytes(b\"place your seed here\");"] #[doc =
        " let rt = runtime::Builder::new_current_thread()"] #[doc =
        "     .rng_seed(seed)"] #[doc = "     .build();"] #[doc = " # }"] #[doc = " ```"]
        #[doc = ""] #[doc = " [`tokio::select!`]: crate::select"] pub fn rng_seed(& mut
        self, seed : RngSeed) -> & mut Self { self.seed_generator =
        RngSeedGenerator::new(seed); self }
    }
    cfg_unstable_metrics! {
        #[doc = " Enables tracking the distribution of task poll times."] #[doc = ""]
        #[doc = " Task poll times are not instrumented by default as doing so requires"]
        #[doc = " calling [`Instant::now()`] twice per task poll, which could add"] #[doc
        = " measurable overhead. Use the [`Handle::metrics()`] to access the"] #[doc =
        " metrics data."] #[doc = ""] #[doc =
        " The histogram uses fixed bucket sizes. In other words, the histogram"] #[doc =
        " buckets are not dynamic based on input values. Use the"] #[doc =
        " `metrics_poll_time_histogram` builder methods to configure the"] #[doc =
        " histogram details."] #[doc = ""] #[doc =
        " By default, a linear histogram with 10 buckets each 100 microseconds wide will be used."]
        #[doc =
        " This has an extremely low memory footprint, but may not provide enough granularity. For"]
        #[doc =
        " better granularity with low memory usage, use [`metrics_poll_time_histogram_configuration()`]"]
        #[doc = " to select [`LogHistogram`] instead."] #[doc = ""] #[doc =
        " # Examples"] #[doc = ""] #[doc = " ```"] #[doc =
        " # #[cfg(not(target_family = \"wasm\"))]"] #[doc = " # {"] #[doc =
        " use tokio::runtime;"] #[doc = ""] #[doc =
        " let rt = runtime::Builder::new_multi_thread()"] #[doc =
        "     .enable_metrics_poll_time_histogram()"] #[doc = "     .build()"] #[doc =
        "     .unwrap();"] #[doc = " # // Test default values here"] #[doc =
        " # fn us(n: u64) -> std::time::Duration { std::time::Duration::from_micros(n) }"]
        #[doc = " # let m = rt.handle().metrics();"] #[doc =
        " # assert_eq!(m.poll_time_histogram_num_buckets(), 10);"] #[doc =
        " # assert_eq!(m.poll_time_histogram_bucket_range(0), us(0)..us(100));"] #[doc =
        " # assert_eq!(m.poll_time_histogram_bucket_range(1), us(100)..us(200));"] #[doc
        = " # }"] #[doc = " ```"] #[doc = ""] #[doc =
        " [`Handle::metrics()`]: crate::runtime::Handle::metrics"] #[doc =
        " [`Instant::now()`]: std::time::Instant::now"] #[doc =
        " [`LogHistogram`]: crate::runtime::LogHistogram"] #[doc =
        " [`metrics_poll_time_histogram_configuration()`]: Builder::metrics_poll_time_histogram_configuration"]
        pub fn enable_metrics_poll_time_histogram(& mut self) -> & mut Self { self
        .metrics_poll_count_histogram_enable = true; self } #[doc =
        " Deprecated. Use [`enable_metrics_poll_time_histogram()`] instead."] #[doc = ""]
        #[doc =
        " [`enable_metrics_poll_time_histogram()`]: Builder::enable_metrics_poll_time_histogram"]
        #[deprecated(note =
        "`poll_count_histogram` related methods have been renamed `poll_time_histogram` to better reflect their functionality.")]
        #[doc(hidden)] pub fn enable_metrics_poll_count_histogram(& mut self) -> & mut
        Self { self.enable_metrics_poll_time_histogram() } #[doc =
        " Sets the histogram scale for tracking the distribution of task poll"] #[doc =
        " times."] #[doc = ""] #[doc =
        " Tracking the distribution of task poll times can be done using a"] #[doc =
        " linear or log scale. When using linear scale, each histogram bucket"] #[doc =
        " will represent the same range of poll times. When using log scale,"] #[doc =
        " each histogram bucket will cover a range twice as big as the"] #[doc =
        " previous bucket."] #[doc = ""] #[doc = " **Default:** linear scale."] #[doc =
        ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc =
        " # #[cfg(not(target_family = \"wasm\"))]"] #[doc = " # {"] #[doc =
        " use tokio::runtime::{self, HistogramScale};"] #[doc = ""] #[doc =
        " # #[allow(deprecated)]"] #[doc =
        " let rt = runtime::Builder::new_multi_thread()"] #[doc =
        "     .enable_metrics_poll_time_histogram()"] #[doc =
        "     .metrics_poll_count_histogram_scale(HistogramScale::Log)"] #[doc =
        "     .build()"] #[doc = "     .unwrap();"] #[doc = " # }"] #[doc = " ```"]
        #[deprecated(note = "use `metrics_poll_time_histogram_configuration`")] pub fn
        metrics_poll_count_histogram_scale(& mut self, histogram_scale : crate
        ::runtime::HistogramScale) -> & mut Self { self.metrics_poll_count_histogram
        .legacy_mut(| b | b.scale = histogram_scale); self } #[doc =
        " Configure the histogram for tracking poll times"] #[doc = ""] #[doc =
        " By default, a linear histogram with 10 buckets each 100 microseconds wide will be used."]
        #[doc =
        " This has an extremely low memory footprint, but may not provide enough granularity. For"]
        #[doc =
        " better granularity with low memory usage, use [`LogHistogram`] instead."] #[doc
        = ""] #[doc = " # Examples"] #[doc =
        " Configure a [`LogHistogram`] with [default configuration]:"] #[doc = " ```"]
        #[doc = " # #[cfg(not(target_family = \"wasm\"))]"] #[doc = " # {"] #[doc =
        " use tokio::runtime;"] #[doc =
        " use tokio::runtime::{HistogramConfiguration, LogHistogram};"] #[doc = ""] #[doc
        = " let rt = runtime::Builder::new_multi_thread()"] #[doc =
        "     .enable_metrics_poll_time_histogram()"] #[doc =
        "     .metrics_poll_time_histogram_configuration("] #[doc =
        "         HistogramConfiguration::log(LogHistogram::default())"] #[doc =
        "     )"] #[doc = "     .build()"] #[doc = "     .unwrap();"] #[doc = " # }"]
        #[doc = " ```"] #[doc = ""] #[doc =
        " Configure a linear histogram with 100 buckets, each 10μs wide"] #[doc =
        " ```"] #[doc = " # #[cfg(not(target_family = \"wasm\"))]"] #[doc = " # {"] #[doc
        = " use tokio::runtime;"] #[doc = " use std::time::Duration;"] #[doc =
        " use tokio::runtime::HistogramConfiguration;"] #[doc = ""] #[doc =
        " let rt = runtime::Builder::new_multi_thread()"] #[doc =
        "     .enable_metrics_poll_time_histogram()"] #[doc =
        "     .metrics_poll_time_histogram_configuration("] #[doc =
        "         HistogramConfiguration::linear(Duration::from_micros(10), 100)"] #[doc
        = "     )"] #[doc = "     .build()"] #[doc = "     .unwrap();"] #[doc = " # }"]
        #[doc = " ```"] #[doc = ""] #[doc =
        " Configure a [`LogHistogram`] with the following settings:"] #[doc =
        " - Measure times from 100ns to 120s"] #[doc = " - Max error of 0.1"] #[doc =
        " - No more than 1024 buckets"] #[doc = " ```"] #[doc =
        " # #[cfg(not(target_family = \"wasm\"))]"] #[doc = " # {"] #[doc =
        " use std::time::Duration;"] #[doc = " use tokio::runtime;"] #[doc =
        " use tokio::runtime::{HistogramConfiguration, LogHistogram};"] #[doc = ""] #[doc
        = " let rt = runtime::Builder::new_multi_thread()"] #[doc =
        "     .enable_metrics_poll_time_histogram()"] #[doc =
        "     .metrics_poll_time_histogram_configuration("] #[doc =
        "         HistogramConfiguration::log(LogHistogram::builder()"] #[doc =
        "             .max_value(Duration::from_secs(120))"] #[doc =
        "             .min_value(Duration::from_nanos(100))"] #[doc =
        "             .max_error(0.1)"] #[doc = "             .max_buckets(1024)"] #[doc
        = "             .expect(\"configuration uses 488 buckets\")"] #[doc =
        "         )"] #[doc = "     )"] #[doc = "     .build()"] #[doc =
        "     .unwrap();"] #[doc = " # }"] #[doc = " ```"] #[doc = ""] #[doc =
        " When migrating from the legacy histogram ([`HistogramScale::Log`]) and wanting"]
        #[doc =
        " to match the previous behavior, use `precision_exact(0)`. This creates a histogram"]
        #[doc = " where each bucket is twice the size of the previous bucket."] #[doc =
        " ```rust"] #[doc = " use std::time::Duration;"] #[doc =
        " use tokio::runtime::{HistogramConfiguration, LogHistogram};"] #[doc =
        " let rt = tokio::runtime::Builder::new_current_thread()"] #[doc =
        "     .enable_all()"] #[doc = "     .enable_metrics_poll_time_histogram()"] #[doc
        = "     .metrics_poll_time_histogram_configuration(HistogramConfiguration::log("]
        #[doc = "         LogHistogram::builder()"] #[doc =
        "             .min_value(Duration::from_micros(20))"] #[doc =
        "             .max_value(Duration::from_millis(4))"] #[doc =
        "             // Set `precision_exact` to `0` to match `HistogramScale::Log`"]
        #[doc = "             .precision_exact(0)"] #[doc =
        "             .max_buckets(10)"] #[doc = "             .unwrap(),"] #[doc =
        "     ))"] #[doc = "     .build()"] #[doc = "     .unwrap();"] #[doc = " ```"]
        #[doc = ""] #[doc = " [`LogHistogram`]: crate::runtime::LogHistogram"] #[doc =
        " [default configuration]: crate::runtime::LogHistogramBuilder"] #[doc =
        " [`HistogramScale::Log`]: crate::runtime::HistogramScale::Log"] pub fn
        metrics_poll_time_histogram_configuration(& mut self, configuration :
        HistogramConfiguration) -> & mut Self { self.metrics_poll_count_histogram
        .histogram_type = configuration.inner; self } #[doc =
        " Sets the histogram resolution for tracking the distribution of task"] #[doc =
        " poll times."] #[doc = ""] #[doc =
        " The resolution is the histogram's first bucket's range. When using a"] #[doc =
        " linear histogram scale, each bucket will cover the same range. When"] #[doc =
        " using a log scale, each bucket will cover a range twice as big as"] #[doc =
        " the previous bucket. In the log case, the resolution represents the"] #[doc =
        " smallest bucket range."] #[doc = ""] #[doc =
        " Note that, when using log scale, the resolution is rounded up to the"] #[doc =
        " nearest power of 2 in nanoseconds."] #[doc = ""] #[doc =
        " **Default:** 100 microseconds."] #[doc = ""] #[doc = " # Examples"] #[doc = ""]
        #[doc = " ```"] #[doc = " # #[cfg(not(target_family = \"wasm\"))]"] #[doc =
        " # {"] #[doc = " use tokio::runtime;"] #[doc = " use std::time::Duration;"]
        #[doc = ""] #[doc = " # #[allow(deprecated)]"] #[doc =
        " let rt = runtime::Builder::new_multi_thread()"] #[doc =
        "     .enable_metrics_poll_time_histogram()"] #[doc =
        "     .metrics_poll_count_histogram_resolution(Duration::from_micros(100))"]
        #[doc = "     .build()"] #[doc = "     .unwrap();"] #[doc = " # }"] #[doc =
        " ```"] #[deprecated(note = "use `metrics_poll_time_histogram_configuration`")]
        pub fn metrics_poll_count_histogram_resolution(& mut self, resolution : Duration)
        -> & mut Self { assert!(resolution > Duration::from_secs(0)); assert!(resolution
        <= Duration::from_secs(1)); let resolution = resolution.as_nanos() as u64; self
        .metrics_poll_count_histogram.legacy_mut(| b | b.resolution = resolution); self }
        #[doc = " Sets the number of buckets for the histogram tracking the"] #[doc =
        " distribution of task poll times."] #[doc = ""] #[doc =
        " The last bucket tracks all greater values that fall out of other"] #[doc =
        " ranges. So, configuring the histogram using a linear scale,"] #[doc =
        " resolution of 50ms, and 10 buckets, the 10th bucket will track task"] #[doc =
        " polls that take more than 450ms to complete."] #[doc = ""] #[doc =
        " **Default:** 10"] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc =
        " ```"] #[doc = " # #[cfg(not(target_family = \"wasm\"))]"] #[doc = " # {"] #[doc
        = " use tokio::runtime;"] #[doc = ""] #[doc = " # #[allow(deprecated)]"] #[doc =
        " let rt = runtime::Builder::new_multi_thread()"] #[doc =
        "     .enable_metrics_poll_time_histogram()"] #[doc =
        "     .metrics_poll_count_histogram_buckets(15)"] #[doc = "     .build()"] #[doc
        = "     .unwrap();"] #[doc = " # }"] #[doc = " ```"] #[deprecated(note =
        "use `metrics_poll_time_histogram_configuration`")] pub fn
        metrics_poll_count_histogram_buckets(& mut self, buckets : usize) -> & mut Self {
        self.metrics_poll_count_histogram.legacy_mut(| b | b.num_buckets = buckets); self
        }
    }
    fn build_current_thread_runtime(&mut self) -> io::Result<Runtime> {
        panic!("STUB: not implemented");
    }
    fn build_current_thread_local_runtime(&mut self) -> io::Result<LocalRuntime> {
        panic!("STUB: not implemented");
    }
    fn build_current_thread_runtime_components(
        &mut self,
        local_tid: Option<ThreadId>,
    ) -> io::Result<(CurrentThread, Handle, BlockingPool)> {
        panic!("STUB: not implemented");
    }
    fn metrics_poll_count_histogram_builder(&self) -> Option<HistogramBuilder> {
        panic!("STUB: not implemented");
    }
    fn metrics_schedule_latency_histogram_builder(&self) -> Option<HistogramBuilder> {
        panic!("STUB: not implemented");
    }
}
cfg_io_driver! {
    impl Builder { #[doc = " Enables the I/O driver."] #[doc = ""] #[doc =
    " Doing this enables using net, process, signal, and some I/O types on"] #[doc =
    " the runtime."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc
    = " use tokio::runtime;"] #[doc = ""] #[doc =
    " let rt = runtime::Builder::new_multi_thread()"] #[doc = "     .enable_io()"] #[doc
    = "     .build()"] #[doc = "     .unwrap();"] #[doc = " ```"] pub fn enable_io(& mut
    self) -> & mut Self { self.enable_io = true; self } #[doc =
    " Enables the I/O driver and configures the max number of events to be"] #[doc =
    " processed per tick."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc =
    " ```"] #[doc = " use tokio::runtime;"] #[doc = ""] #[doc =
    " let rt = runtime::Builder::new_current_thread()"] #[doc = "     .enable_io()"]
    #[doc = "     .max_io_events_per_tick(1024)"] #[doc = "     .build()"] #[doc =
    "     .unwrap();"] #[doc = " ```"] pub fn max_io_events_per_tick(& mut self, capacity
    : usize) -> & mut Self { self.nevents = capacity; self } }
}
cfg_time! {
    impl Builder { #[doc = " Enables the time driver."] #[doc = ""] #[doc =
    " Doing this enables using `tokio::time` on the runtime."] #[doc = ""] #[doc =
    " # Examples"] #[doc = ""] #[doc = " ```"] #[doc =
    " # #[cfg(not(target_family = \"wasm\"))]"] #[doc = " # {"] #[doc =
    " use tokio::runtime;"] #[doc = ""] #[doc =
    " let rt = runtime::Builder::new_multi_thread()"] #[doc = "     .enable_time()"]
    #[doc = "     .build()"] #[doc = "     .unwrap();"] #[doc = " # }"] #[doc = " ```"]
    pub fn enable_time(& mut self) -> & mut Self { self.enable_time = true; self } }
}
cfg_io_uring! {
    impl Builder { #[doc = " Enables the tokio's io_uring driver."] #[doc = ""] #[doc =
    " Doing this enables using io_uring operations on the runtime."] #[doc = ""] #[doc =
    " # Examples"] #[doc = ""] #[doc = " ```"] #[doc = " use tokio::runtime;"] #[doc =
    ""] #[doc = " let rt = runtime::Builder::new_multi_thread()"] #[doc =
    "     .enable_io_uring()"] #[doc = "     .build()"] #[doc = "     .unwrap();"] #[doc
    = " ```"] #[cfg_attr(docsrs, doc(cfg(feature = "io-uring")))] pub fn
    enable_io_uring(& mut self) -> & mut Self { self.enable_io = true; self } }
}
cfg_test_util! {
    impl Builder { #[doc =
    " Controls if the runtime's clock starts paused or advancing."] #[doc = ""] #[doc =
    " Pausing time requires the current-thread runtime; construction of"] #[doc =
    " the runtime will panic otherwise."] #[doc = ""] #[doc = " # Examples"] #[doc = ""]
    #[doc = " ```"] #[doc = " use tokio::runtime;"] #[doc = ""] #[doc =
    " let rt = runtime::Builder::new_current_thread()"] #[doc = "     .enable_time()"]
    #[doc = "     .start_paused(true)"] #[doc = "     .build()"] #[doc =
    "     .unwrap();"] #[doc = " ```"] pub fn start_paused(& mut self, start_paused :
    bool) -> & mut Self { self.start_paused = start_paused; self } }
}
cfg_schedule_latency! {
    impl Builder { #[doc =
    " Enables tracking the distribution of task schedule latencies. Task"] #[doc =
    " schedule latency is the time between when a task is scheduled for"] #[doc =
    " execution and when it is polled."] #[doc = ""] #[doc =
    " **This feature is only supported on 64-bit targets.**"] #[doc = ""] #[doc =
    " Task schedule latencies are not instrumented by default as doing"] #[doc =
    " so requires calling [`Instant::now()`] when a task is scheduled"] #[doc =
    " and when it is polled, which could add measurable overhead. Use"] #[doc =
    " the [`Handle::metrics()`] to access the metrics data."] #[doc = ""] #[doc =
    " By default, a linear histogram with 10 buckets each 100 microseconds wide will be used."]
    #[doc =
    " This has an extremely low memory footprint, but may not provide enough granularity. For"]
    #[doc =
    " better granularity with low memory usage, use [`metrics_schedule_latency_histogram_configuration()`]"]
    #[doc = " to select [`LogHistogram`] instead."] #[doc = ""] #[doc = " # Examples"]
    #[doc = ""] #[doc = " ```"] #[doc = " # #[cfg(not(target_family = \"wasm\"))]"] #[doc
    = " # {"] #[doc = " use tokio::runtime;"] #[doc = ""] #[doc =
    " let rt = runtime::Builder::new_multi_thread()"] #[doc =
    "     .enable_metrics_schedule_latency_histogram()"] #[doc = "     .build()"] #[doc =
    "     .unwrap();"] #[doc = " # // Test default values here"] #[doc =
    " # fn us(n: u64) -> std::time::Duration { std::time::Duration::from_micros(n) }"]
    #[doc = " # let m = rt.handle().metrics();"] #[doc =
    " # assert_eq!(m.schedule_latency_histogram_num_buckets(), 10);"] #[doc =
    " # assert_eq!(m.schedule_latency_histogram_bucket_range(0), us(0)..us(100));"] #[doc
    = " # assert_eq!(m.schedule_latency_histogram_bucket_range(1), us(100)..us(200));"]
    #[doc = " # }"] #[doc = " ```"] #[doc = ""] #[doc =
    " [`Handle::metrics()`]: crate::runtime::Handle::metrics"] #[doc =
    " [`Instant::now()`]: std::time::Instant::now"] #[doc =
    " [`LogHistogram`]: crate::runtime::LogHistogram"] #[doc =
    " [`metrics_schedule_latency_histogram_configuration()`]: Builder::metrics_schedule_latency_histogram_configuration"]
    pub fn enable_metrics_schedule_latency_histogram(& mut self) -> & mut Self { self
    .metrics_schedule_latency_histogram_enabled = true; self } #[doc =
    " Configure the histogram for tracking task schedule latencies."] #[doc = ""] #[doc =
    " Tracking of task schedule latencies must be enabled with"] #[doc =
    " [`enable_metrics_schedule_latency_histogram()`] for this function"] #[doc =
    " to have any effect."] #[doc = ""] #[doc =
    " By default, a linear histogram with 10 buckets each 100 microseconds wide will be used."]
    #[doc =
    " This has an extremely low memory footprint, but may not provide enough granularity. For"]
    #[doc = " better granularity with low memory usage, use [`LogHistogram`] instead."]
    #[doc = ""] #[doc = " # Examples"] #[doc =
    " Configure a [`LogHistogram`] with [default configuration]:"] #[doc = " ```"] #[doc
    = " # #[cfg(not(target_family = \"wasm\"))]"] #[doc = " # {"] #[doc =
    " use tokio::runtime;"] #[doc =
    " use tokio::runtime::{HistogramConfiguration, LogHistogram};"] #[doc = ""] #[doc =
    " let rt = runtime::Builder::new_multi_thread()"] #[doc =
    "     .enable_metrics_schedule_latency_histogram()"] #[doc =
    "     .metrics_schedule_latency_histogram_configuration("] #[doc =
    "         HistogramConfiguration::log(LogHistogram::default())"] #[doc = "     )"]
    #[doc = "     .build()"] #[doc = "     .unwrap();"] #[doc = " # }"] #[doc = " ```"]
    #[doc = ""] #[doc =
    " Configure a linear histogram with 100 buckets, each 10μs wide"] #[doc = " ```"]
    #[doc = " # #[cfg(not(target_family = \"wasm\"))]"] #[doc = " # {"] #[doc =
    " use tokio::runtime;"] #[doc = " use std::time::Duration;"] #[doc =
    " use tokio::runtime::HistogramConfiguration;"] #[doc = ""] #[doc =
    " let rt = runtime::Builder::new_multi_thread()"] #[doc =
    "     .enable_metrics_schedule_latency_histogram()"] #[doc =
    "     .metrics_schedule_latency_histogram_configuration("] #[doc =
    "         HistogramConfiguration::linear(Duration::from_micros(10), 100)"] #[doc =
    "     )"] #[doc = "     .build()"] #[doc = "     .unwrap();"] #[doc = " # }"] #[doc =
    " ```"] #[doc = ""] #[doc =
    " Configure a [`LogHistogram`] with the following settings:"] #[doc =
    " - Measure times from 100ns to 120s"] #[doc = " - Max error of 0.1"] #[doc =
    " - No more than 1024 buckets"] #[doc = " ```"] #[doc =
    " # #[cfg(not(target_family = \"wasm\"))]"] #[doc = " # {"] #[doc =
    " use std::time::Duration;"] #[doc = " use tokio::runtime;"] #[doc =
    " use tokio::runtime::{HistogramConfiguration, LogHistogram};"] #[doc = ""] #[doc =
    " let rt = runtime::Builder::new_multi_thread()"] #[doc =
    "     .enable_metrics_schedule_latency_histogram()"] #[doc =
    "     .metrics_schedule_latency_histogram_configuration("] #[doc =
    "         HistogramConfiguration::log(LogHistogram::builder()"] #[doc =
    "             .max_value(Duration::from_secs(120))"] #[doc =
    "             .min_value(Duration::from_nanos(100))"] #[doc =
    "             .max_error(0.1)"] #[doc = "             .max_buckets(1024)"] #[doc =
    "             .expect(\"configuration uses 488 buckets\")"] #[doc = "         )"]
    #[doc = "     )"] #[doc = "     .build()"] #[doc = "     .unwrap();"] #[doc = " # }"]
    #[doc = " ```"] #[doc = ""] #[doc =
    " [`LogHistogram`]: crate::runtime::LogHistogram"] #[doc =
    " [`enable_metrics_schedule_latency_histogram()`]: Builder::enable_metrics_schedule_latency_histogram"]
    pub fn metrics_schedule_latency_histogram_configuration(& mut self, configuration :
    HistogramConfiguration) -> & mut Self { self.metrics_schedule_latency_histogram
    .histogram_type = configuration.inner; self } }
}
cfg_rt_multi_thread! {
    impl Builder { fn build_threaded_runtime(& mut self) -> io::Result < Runtime > { use
    crate ::loom::sys::num_cpus; use crate ::runtime:: { Config, runtime::Scheduler };
    use crate ::runtime::scheduler:: { self, MultiThread }; let worker_threads = self
    .worker_threads.unwrap_or_else(num_cpus); let (driver, driver_handle) =
    driver::Driver::new(self.get_cfg()) ?; let blocking_pool =
    blocking::create_blocking_pool(self, self.max_blocking_threads + worker_threads); let
    blocking_spawner = blocking_pool.spawner().clone(); let seed_generator_1 = self
    .seed_generator.next_generator(); let seed_generator_2 = self.seed_generator
    .next_generator(); let (scheduler, handle, launch) = MultiThread::new(worker_threads,
    driver, driver_handle, blocking_spawner, seed_generator_2, Config { before_park :
    self.before_park.clone(), after_unpark : self.after_unpark.clone(), before_spawn :
    self.before_spawn.clone(), #[cfg(tokio_unstable)] before_poll : self.before_poll
    .clone(), #[cfg(tokio_unstable)] after_poll : self.after_poll.clone(),
    after_termination : self.after_termination.clone(), global_queue_interval : self
    .global_queue_interval, event_interval : self.event_interval, #[cfg(tokio_unstable)]
    unhandled_panic : self.unhandled_panic.clone(), disable_lifo_slot : self
    .disable_lifo_slot, enable_eager_driver_handoff : self.enable_eager_driver_handoff,
    seed_generator : seed_generator_1, metrics_poll_count_histogram : self
    .metrics_poll_count_histogram_builder(), metrics_schedule_latency_histogram : self
    .metrics_schedule_latency_histogram_builder(), }, self.timer_flavor, self.name
    .clone(),); let handle = Handle { inner : scheduler::Handle::MultiThread(handle) };
    let _enter = handle.enter(); launch.launch();
    Ok(Runtime::from_parts(Scheduler::MultiThread(scheduler), handle, blocking_pool)) } }
}
impl fmt::Debug for Builder {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
