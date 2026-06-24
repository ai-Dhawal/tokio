use crate::runtime::Handle;
use std::time::Duration;
cfg_64bit_metrics! {
    use std::sync::atomic::Ordering::Relaxed;
}
cfg_unstable_metrics! {
    use std::ops::Range; use std::thread::ThreadId;
}
/// Handle to the runtime's metrics.
///
/// This handle is internally reference-counted and can be freely cloned. A
/// `RuntimeMetrics` handle is obtained using the [`Runtime::metrics`] method.
///
/// [`Runtime::metrics`]: crate::runtime::Runtime::metrics()
#[derive(Clone, Debug)]
pub struct RuntimeMetrics {
    handle: Handle,
}
impl RuntimeMetrics {
    pub(crate) fn new(handle: Handle) -> RuntimeMetrics {
        panic!("STUB: not implemented");
    }
    /// Returns the number of worker threads used by the runtime.
    ///
    /// The number of workers is set by configuring `worker_threads` on
    /// `runtime::Builder`. When using the `current_thread` runtime, the return
    /// value is always `1`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::runtime::Handle;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let metrics = Handle::current().metrics();
    ///
    /// let n = metrics.num_workers();
    /// println!("Runtime is using {} workers", n);
    /// # }
    /// ```
    pub fn num_workers(&self) -> usize {
        panic!("STUB: not implemented");
    }
    /// Returns the current number of alive tasks in the runtime.
    ///
    /// This counter increases when a task is spawned and decreases when a
    /// task exits.
    ///
    /// Note: When using the multi-threaded runtime this number may not
    /// not have strong consistency i.e. no tasks may be running but the metric
    /// reports otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::runtime::Handle;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let metrics = Handle::current().metrics();
    ///
    /// let n = metrics.num_alive_tasks();
    /// println!("Runtime has {} alive tasks", n);
    /// # }
    /// ```
    pub fn num_alive_tasks(&self) -> usize {
        panic!("STUB: not implemented");
    }
    /// Returns the number of tasks currently scheduled in the runtime's
    /// global queue.
    ///
    /// Tasks that are spawned or notified from a non-runtime thread are
    /// scheduled using the runtime's global queue. This metric returns the
    /// **current** number of tasks pending in the global queue. As such, the
    /// returned value may increase or decrease as new tasks are scheduled and
    /// processed.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::runtime::Handle;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let metrics = Handle::current().metrics();
    ///
    /// let n = metrics.global_queue_depth();
    /// println!("{} tasks currently pending in the runtime's global queue", n);
    /// # }
    /// ```
    pub fn global_queue_depth(&self) -> usize {
        panic!("STUB: not implemented");
    }
    cfg_64bit_metrics! {
        #[doc = " Returns the amount of time the given worker thread has been busy."]
        #[doc = ""] #[doc =
        " The worker busy duration starts at zero when the runtime is created and"] #[doc
        = " increases whenever the worker is spending time processing work. Using"] #[doc
        = " this value can indicate the load of the given worker. If a lot of time"]
        #[doc =
        " is spent busy, then the worker is under load and will check for inbound"] #[doc
        = " events less often."] #[doc = ""] #[doc =
        " The timer is monotonically increasing. It is never decremented or reset"] #[doc
        = " to zero."] #[doc = ""] #[doc = " # Arguments"] #[doc = ""] #[doc =
        " `worker` is the index of the worker being queried. The given value must"] #[doc
        = " be between 0 and `num_workers()`. The index uniquely identifies a single"]
        #[doc =
        " worker and will continue to identify the worker throughout the lifetime"] #[doc
        = " of the runtime instance."] #[doc = ""] #[doc = " # Panics"] #[doc = ""] #[doc
        = " The method panics when `worker` represents an invalid worker, i.e. is"] #[doc
        = " greater than or equal to `num_workers()`."] #[doc = ""] #[doc =
        " # Examples"] #[doc = ""] #[doc = " ```"] #[doc =
        " use tokio::runtime::Handle;"] #[doc = ""] #[doc =
        " # #[tokio::main(flavor = \"current_thread\")]"] #[doc = " # async fn main() {"]
        #[doc = " let metrics = Handle::current().metrics();"] #[doc = ""] #[doc =
        " let n = metrics.worker_total_busy_duration(0);"] #[doc =
        " println!(\"worker 0 was busy for a total of {:?}\", n);"] #[doc = " # }"] #[doc
        = " ```"] pub fn worker_total_busy_duration(& self, worker : usize) -> Duration {
        let nanos = self.handle.inner.worker_metrics(worker).busy_duration_total
        .load(Relaxed); Duration::from_nanos(nanos) } #[doc =
        " Returns the total number of times the given worker thread has parked."] #[doc =
        ""] #[doc =
        " The worker park count starts at zero when the runtime is created and"] #[doc =
        " increases by one each time the worker parks the thread waiting for new"] #[doc
        = " inbound events to process. This usually means the worker has processed"]
        #[doc = " all pending work and is currently idle."] #[doc = ""] #[doc =
        " The counter is monotonically increasing. It is never decremented or"] #[doc =
        " reset to zero."] #[doc = ""] #[doc = " # Arguments"] #[doc = ""] #[doc =
        " `worker` is the index of the worker being queried. The given value must"] #[doc
        = " be between 0 and `num_workers()`. The index uniquely identifies a single"]
        #[doc =
        " worker and will continue to identify the worker throughout the lifetime"] #[doc
        = " of the runtime instance."] #[doc = ""] #[doc = " # Panics"] #[doc = ""] #[doc
        = " The method panics when `worker` represents an invalid worker, i.e. is"] #[doc
        = " greater than or equal to `num_workers()`."] #[doc = ""] #[doc =
        " # Examples"] #[doc = ""] #[doc = " ```"] #[doc =
        " use tokio::runtime::Handle;"] #[doc = ""] #[doc =
        " # #[tokio::main(flavor = \"current_thread\")]"] #[doc = " # async fn main() {"]
        #[doc = " let metrics = Handle::current().metrics();"] #[doc = ""] #[doc =
        " let n = metrics.worker_park_count(0);"] #[doc =
        " println!(\"worker 0 parked {} times\", n);"] #[doc = " # }"] #[doc = " ```"]
        pub fn worker_park_count(& self, worker : usize) -> u64 { self.handle.inner
        .worker_metrics(worker).park_count.load(Relaxed) } #[doc =
        " Returns the total number of times the given worker thread has parked"] #[doc =
        " and unparked."] #[doc = ""] #[doc =
        " The worker park/unpark count starts at zero when the runtime is created"] #[doc
        = " and increases by one each time the worker parks the thread waiting for"]
        #[doc =
        " new inbound events to process. This usually means the worker has processed"]
        #[doc =
        " all pending work and is currently idle. When new work becomes available,"]
        #[doc =
        " the worker is unparked and the park/unpark count is again increased by one."]
        #[doc = ""] #[doc = " An odd count means that the worker is currently parked."]
        #[doc = " An even count means that the worker is currently active."] #[doc = ""]
        #[doc = " The counter is monotonically increasing. It is never decremented or"]
        #[doc = " reset to zero."] #[doc = ""] #[doc = " # Arguments"] #[doc = ""] #[doc
        = " `worker` is the index of the worker being queried. The given value must"]
        #[doc =
        " be between 0 and `num_workers()`. The index uniquely identifies a single"]
        #[doc =
        " worker and will continue to identify the worker throughout the lifetime"] #[doc
        = " of the runtime instance."] #[doc = ""] #[doc = " # Panics"] #[doc = ""] #[doc
        = " The method panics when `worker` represents an invalid worker, i.e. is"] #[doc
        = " greater than or equal to `num_workers()`."] #[doc = ""] #[doc =
        " # Examples"] #[doc = ""] #[doc = " ```"] #[doc =
        " use tokio::runtime::Handle;"] #[doc = ""] #[doc =
        " # #[tokio::main(flavor = \"current_thread\")]"] #[doc = " # async fn main() {"]
        #[doc = " let metrics = Handle::current().metrics();"] #[doc =
        " let n = metrics.worker_park_unpark_count(0);"] #[doc = ""] #[doc =
        " println!(\"worker 0 parked and unparked {} times\", n);"] #[doc = ""] #[doc =
        " if n % 2 == 0 {"] #[doc = "     println!(\"worker 0 is active\");"] #[doc =
        " } else {"] #[doc = "     println!(\"worker 0 is parked\");"] #[doc = " }"]
        #[doc = " # }"] #[doc = " ```"] pub fn worker_park_unpark_count(& self, worker :
        usize) -> u64 { self.handle.inner.worker_metrics(worker).park_unpark_count
        .load(Relaxed) }
    }
    cfg_unstable_metrics! {
        #[doc = " Returns the number of additional threads spawned by the runtime."]
        #[doc = ""] #[doc =
        " The number of workers is set by configuring `max_blocking_threads` on"] #[doc =
        " `runtime::Builder`."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc =
        " ```"] #[doc = " # #[cfg(not(target_family = \"wasm\"))]"] #[doc = " # {"] #[doc
        = " use tokio::runtime::Handle;"] #[doc = ""] #[doc =
        " # #[tokio::main(flavor = \"current_thread\")]"] #[doc = " # async fn main() {"]
        #[doc = " let _ = tokio::task::spawn_blocking(move || {"] #[doc =
        "     // Stand-in for compute-heavy work or using synchronous APIs"] #[doc =
        "     1 + 1"] #[doc = " }).await;"] #[doc =
        " let metrics = Handle::current().metrics();"] #[doc = ""] #[doc =
        " let n = metrics.num_blocking_threads();"] #[doc =
        " println!(\"Runtime has created {} threads\", n);"] #[doc = " # }"] #[doc =
        " # }"] #[doc = " ```"] pub fn num_blocking_threads(& self) -> usize { self
        .handle.inner.num_blocking_threads() } #[deprecated =
        "Renamed to num_alive_tasks"] #[doc =
        " Renamed to [`RuntimeMetrics::num_alive_tasks`]"] pub fn active_tasks_count(&
        self) -> usize { self.num_alive_tasks() } #[doc =
        " Returns the number of idle threads, which have spawned by the runtime"] #[doc =
        " for `spawn_blocking` calls."] #[doc = ""] #[doc = " # Examples"] #[doc = ""]
        #[doc = " ```"] #[doc = " # #[cfg(not(target_family = \"wasm\"))]"] #[doc =
        " # {"] #[doc = " use tokio::runtime::Handle;"] #[doc = ""] #[doc =
        " #[tokio::main]"] #[doc = " async fn main() {"] #[doc =
        "     let _ = tokio::task::spawn_blocking(move || {"] #[doc =
        "         // Stand-in for compute-heavy work or using synchronous APIs"] #[doc =
        "         1 + 1"] #[doc = "     }).await;"] #[doc =
        "     let metrics = Handle::current().metrics();"] #[doc = ""] #[doc =
        "     let n = metrics.num_idle_blocking_threads();"] #[doc =
        "     println!(\"Runtime has {} idle blocking thread pool threads\", n);"] #[doc
        = " }"] #[doc = " # }"] #[doc = " ```"] pub fn num_idle_blocking_threads(& self)
        -> usize { self.handle.inner.num_idle_blocking_threads() } #[doc =
        " Returns the thread id of the given worker thread."] #[doc = ""] #[doc =
        " The returned value is `None` if the worker thread has not yet finished"] #[doc
        = " starting up."] #[doc = ""] #[doc =
        " If additional information about the thread, such as its native id, are"] #[doc
        = " required, those can be collected in [`on_thread_start`] and correlated"]
        #[doc = " using the thread id."] #[doc = ""] #[doc =
        " [`on_thread_start`]: crate::runtime::Builder::on_thread_start"] #[doc = ""]
        #[doc = " # Arguments"] #[doc = ""] #[doc =
        " `worker` is the index of the worker being queried. The given value must"] #[doc
        = " be between 0 and `num_workers()`. The index uniquely identifies a single"]
        #[doc =
        " worker and will continue to identify the worker throughout the lifetime"] #[doc
        = " of the runtime instance."] #[doc = ""] #[doc = " # Panics"] #[doc = ""] #[doc
        = " The method panics when `worker` represents an invalid worker, i.e. is"] #[doc
        = " greater than or equal to `num_workers()`."] #[doc = ""] #[doc =
        " # Examples"] #[doc = ""] #[doc = " ```"] #[doc =
        " use tokio::runtime::Handle;"] #[doc = ""] #[doc =
        " # #[tokio::main(flavor = \"current_thread\")]"] #[doc = " # async fn main() {"]
        #[doc = " let metrics = Handle::current().metrics();"] #[doc = ""] #[doc =
        " let id = metrics.worker_thread_id(0);"] #[doc =
        " println!(\"worker 0 has id {:?}\", id);"] #[doc = " # }"] #[doc = " ```"] pub
        fn worker_thread_id(& self, worker : usize) -> Option < ThreadId > { self.handle
        .inner.worker_metrics(worker).thread_id() } #[doc =
        " Renamed to [`RuntimeMetrics::global_queue_depth`]"] #[deprecated =
        "Renamed to global_queue_depth"] #[doc(hidden)] pub fn injection_queue_depth(&
        self) -> usize { self.handle.inner.injection_queue_depth() } #[doc =
        " Returns the number of tasks currently scheduled in the given worker's"] #[doc =
        " local queue."] #[doc = ""] #[doc =
        " Tasks that are spawned or notified from within a runtime thread are"] #[doc =
        " scheduled using that worker's local queue. This metric returns the"] #[doc =
        " **current** number of tasks pending in the worker's local queue. As"] #[doc =
        " such, the returned value may increase or decrease as new tasks are"] #[doc =
        " scheduled and processed."] #[doc = ""] #[doc = " # Arguments"] #[doc = ""]
        #[doc =
        " `worker` is the index of the worker being queried. The given value must"] #[doc
        = " be between 0 and `num_workers()`. The index uniquely identifies a single"]
        #[doc =
        " worker and will continue to identify the worker throughout the lifetime"] #[doc
        = " of the runtime instance."] #[doc = ""] #[doc = " # Panics"] #[doc = ""] #[doc
        = " The method panics when `worker` represents an invalid worker, i.e. is"] #[doc
        = " greater than or equal to `num_workers()`."] #[doc = ""] #[doc =
        " # Examples"] #[doc = ""] #[doc = " ```"] #[doc =
        " use tokio::runtime::Handle;"] #[doc = ""] #[doc =
        " # #[tokio::main(flavor = \"current_thread\")]"] #[doc = " # async fn main() {"]
        #[doc = " let metrics = Handle::current().metrics();"] #[doc = ""] #[doc =
        " let n = metrics.worker_local_queue_depth(0);"] #[doc =
        " println!(\"{} tasks currently pending in worker 0's local queue\", n);"] #[doc
        = " # }"] #[doc = " ```"] pub fn worker_local_queue_depth(& self, worker : usize)
        -> usize { self.handle.inner.worker_local_queue_depth(worker) } #[doc =
        " Returns `true` if the runtime is tracking the distribution of task poll"] #[doc
        = " times."] #[doc = ""] #[doc =
        " Task poll times are not instrumented by default as doing so requires"] #[doc =
        " calling [`Instant::now()`] twice per task poll. The feature is enabled"] #[doc
        = " by calling [`enable_metrics_poll_time_histogram()`] when building the"] #[doc
        = " runtime."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"]
        #[doc = " use tokio::runtime::{self, Handle};"] #[doc = ""] #[doc =
        " fn main() {"] #[doc = "     runtime::Builder::new_current_thread()"] #[doc =
        "         .enable_metrics_poll_time_histogram()"] #[doc = "         .build()"]
        #[doc = "         .unwrap()"] #[doc = "         .block_on(async {"] #[doc =
        "             let metrics = Handle::current().metrics();"] #[doc =
        "             let enabled = metrics.poll_time_histogram_enabled();"] #[doc = ""]
        #[doc =
        "             println!(\"Tracking task poll time distribution: {:?}\", enabled);"]
        #[doc = "         });"] #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc =
        " [`enable_metrics_poll_time_histogram()`]: crate::runtime::Builder::enable_metrics_poll_time_histogram"]
        #[doc = " [`Instant::now()`]: std::time::Instant::now"] pub fn
        poll_time_histogram_enabled(& self) -> bool { self.handle.inner.worker_metrics(0)
        .poll_count_histogram.is_some() } #[deprecated(note =
        "Renamed to `poll_time_histogram_enabled`")] #[doc(hidden)] pub fn
        poll_count_histogram_enabled(& self) -> bool { self.poll_time_histogram_enabled()
        } #[doc =
        " Returns the number of histogram buckets tracking the distribution of"] #[doc =
        " task poll times."] #[doc = ""] #[doc = " This value is configured by calling"]
        #[doc =
        " [`metrics_poll_time_histogram_configuration()`] when building the runtime."]
        #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc =
        " use tokio::runtime::{self, Handle};"] #[doc = ""] #[doc = " fn main() {"] #[doc
        = "     runtime::Builder::new_current_thread()"] #[doc =
        "         .enable_metrics_poll_time_histogram()"] #[doc = "         .build()"]
        #[doc = "         .unwrap()"] #[doc = "         .block_on(async {"] #[doc =
        "             let metrics = Handle::current().metrics();"] #[doc =
        "             let buckets = metrics.poll_time_histogram_num_buckets();"] #[doc =
        ""] #[doc = "             println!(\"Histogram buckets: {:?}\", buckets);"] #[doc
        = "         });"] #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc =
        " [`metrics_poll_time_histogram_configuration()`]:"] #[doc =
        "     crate::runtime::Builder::metrics_poll_time_histogram_configuration"] pub fn
        poll_time_histogram_num_buckets(& self) -> usize { self.handle.inner
        .worker_metrics(0).poll_count_histogram.as_ref().map(| histogram | histogram
        .num_buckets()).unwrap_or_default() } #[doc =
        " Deprecated. Use [`poll_time_histogram_num_buckets()`] instead."] #[doc = ""]
        #[doc =
        " [`poll_time_histogram_num_buckets()`]: Self::poll_time_histogram_num_buckets"]
        #[doc(hidden)] #[deprecated(note =
        "renamed to `poll_time_histogram_num_buckets`.")] pub fn
        poll_count_histogram_num_buckets(& self) -> usize { self
        .poll_time_histogram_num_buckets() } #[doc =
        " Returns the range of task poll times tracked by the given bucket."] #[doc = ""]
        #[doc = " This value is configured by calling"] #[doc =
        " [`metrics_poll_time_histogram_configuration()`] when building the runtime."]
        #[doc = ""] #[doc = " # Panics"] #[doc = ""] #[doc =
        " The method panics if `bucket` represents an invalid bucket index, i.e."] #[doc
        = " is greater than or equal to `poll_time_histogram_num_buckets()`."] #[doc =
        ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc =
        " use tokio::runtime::{self, Handle};"] #[doc = ""] #[doc = " fn main() {"] #[doc
        = "     runtime::Builder::new_current_thread()"] #[doc =
        "         .enable_metrics_poll_time_histogram()"] #[doc = "         .build()"]
        #[doc = "         .unwrap()"] #[doc = "         .block_on(async {"] #[doc =
        "             let metrics = Handle::current().metrics();"] #[doc =
        "             let buckets = metrics.poll_time_histogram_num_buckets();"] #[doc =
        ""] #[doc = "             for i in 0..buckets {"] #[doc =
        "                 let range = metrics.poll_time_histogram_bucket_range(i);"]
        #[doc =
        "                 println!(\"Histogram bucket {} range: {:?}\", i, range);"]
        #[doc = "             }"] #[doc = "         });"] #[doc = " }"] #[doc = " ```"]
        #[doc = ""] #[doc = " [`metrics_poll_time_histogram_configuration()`]:"] #[doc =
        "     crate::runtime::Builder::metrics_poll_time_histogram_configuration"]
        #[track_caller] pub fn poll_time_histogram_bucket_range(& self, bucket : usize)
        -> Range < Duration > { self.handle.inner.worker_metrics(0).poll_count_histogram
        .as_ref().map(| histogram | { let range = histogram.bucket_range(bucket);
        std::ops::Range { start : Duration::from_nanos(range.start), end :
        Duration::from_nanos(range.end), } }).unwrap_or_default() } #[doc =
        " Deprecated. Use [`poll_time_histogram_bucket_range()`] instead."] #[doc = ""]
        #[doc =
        " [`poll_time_histogram_bucket_range()`]: Self::poll_time_histogram_bucket_range"]
        #[track_caller] #[doc(hidden)] #[deprecated(note =
        "renamed to `poll_time_histogram_bucket_range`")] pub fn
        poll_count_histogram_bucket_range(& self, bucket : usize) -> Range < Duration > {
        self.poll_time_histogram_bucket_range(bucket) } #[doc =
        " Returns the number of tasks currently scheduled in the blocking"] #[doc =
        " thread pool, spawned using `spawn_blocking`."] #[doc = ""] #[doc =
        " This metric returns the **current** number of tasks pending in"] #[doc =
        " blocking thread pool. As such, the returned value may increase"] #[doc =
        " or decrease as new tasks are scheduled and processed."] #[doc = ""] #[doc =
        " # Examples"] #[doc = ""] #[doc = " ```"] #[doc =
        " use tokio::runtime::Handle;"] #[doc = ""] #[doc =
        " # #[tokio::main(flavor = \"current_thread\")]"] #[doc = " # async fn main() {"]
        #[doc = " let metrics = Handle::current().metrics();"] #[doc = ""] #[doc =
        " let n = metrics.blocking_queue_depth();"] #[doc =
        " println!(\"{} tasks currently pending in the blocking thread pool\", n);"]
        #[doc = " # }"] #[doc = " ```"] pub fn blocking_queue_depth(& self) -> usize {
        self.handle.inner.blocking_queue_depth() }
    }
    feature! {
        #![all(tokio_unstable, target_has_atomic = "64")] #[doc =
        " Returns the number of tasks spawned in this runtime since it was created."]
        #[doc = ""] #[doc =
        " This count starts at zero when the runtime is created and increases by one each time a task is spawned."]
        #[doc = ""] #[doc =
        " The counter is monotonically increasing. It is never decremented or"] #[doc =
        " reset to zero."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"]
        #[doc = " use tokio::runtime::Handle;"] #[doc = ""] #[doc =
        " # #[tokio::main(flavor = \"current_thread\")]"] #[doc = " # async fn main() {"]
        #[doc = " let metrics = Handle::current().metrics();"] #[doc = ""] #[doc =
        " let n = metrics.spawned_tasks_count();"] #[doc =
        " println!(\"Runtime has had {} tasks spawned\", n);"] #[doc = " # }"] #[doc =
        " ```"] pub fn spawned_tasks_count(& self) -> u64 { self.handle.inner
        .spawned_tasks_count() } #[doc =
        " Returns the number of tasks scheduled from **outside** of the runtime."] #[doc
        = ""] #[doc =
        " The remote schedule count starts at zero when the runtime is created and"]
        #[doc = " increases by one each time a task is woken from **outside** of the"]
        #[doc = " runtime. This usually means that a task is spawned or notified from a"]
        #[doc = " non-runtime thread and must be queued using the Runtime's injection"]
        #[doc = " queue, which tends to be slower."] #[doc = ""] #[doc =
        " The counter is monotonically increasing. It is never decremented or"] #[doc =
        " reset to zero."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"]
        #[doc = " use tokio::runtime::Handle;"] #[doc = ""] #[doc =
        " # #[tokio::main(flavor = \"current_thread\")]"] #[doc = " # async fn main() {"]
        #[doc = " let metrics = Handle::current().metrics();"] #[doc = ""] #[doc =
        " let n = metrics.remote_schedule_count();"] #[doc =
        " println!(\"{} tasks were scheduled from outside the runtime\", n);"] #[doc =
        " # }"] #[doc = " ```"] pub fn remote_schedule_count(& self) -> u64 { self.handle
        .inner.scheduler_metrics().remote_schedule_count.load(Relaxed) } #[doc =
        " Returns the number of times that tasks have been forced to yield back to the scheduler"]
        #[doc = " after exhausting their task budgets."] #[doc = ""] #[doc =
        " This count starts at zero when the runtime is created and increases by one each time a task yields due to exhausting its budget."]
        #[doc = ""] #[doc =
        " The counter is monotonically increasing. It is never decremented or"] #[doc =
        " reset to zero."] pub fn budget_forced_yield_count(& self) -> u64 { self.handle
        .inner.scheduler_metrics().budget_forced_yield_count.load(Relaxed) } #[doc =
        " Returns the number of times the given worker thread unparked but"] #[doc =
        " performed no work before parking again."] #[doc = ""] #[doc =
        " The worker no-op count starts at zero when the runtime is created and"] #[doc =
        " increases by one each time the worker unparks the thread but finds no"] #[doc =
        " new work and goes back to sleep. This indicates a false-positive wake up."]
        #[doc = ""] #[doc =
        " The counter is monotonically increasing. It is never decremented or"] #[doc =
        " reset to zero."] #[doc = ""] #[doc = " # Arguments"] #[doc = ""] #[doc =
        " `worker` is the index of the worker being queried. The given value must"] #[doc
        = " be between 0 and `num_workers()`. The index uniquely identifies a single"]
        #[doc =
        " worker and will continue to identify the worker throughout the lifetime"] #[doc
        = " of the runtime instance."] #[doc = ""] #[doc = " # Panics"] #[doc = ""] #[doc
        = " The method panics when `worker` represents an invalid worker, i.e. is"] #[doc
        = " greater than or equal to `num_workers()`."] #[doc = ""] #[doc =
        " # Examples"] #[doc = ""] #[doc = " ```"] #[doc =
        " use tokio::runtime::Handle;"] #[doc = ""] #[doc =
        " # #[tokio::main(flavor = \"current_thread\")]"] #[doc = " # async fn main() {"]
        #[doc = " let metrics = Handle::current().metrics();"] #[doc = ""] #[doc =
        " let n = metrics.worker_noop_count(0);"] #[doc =
        " println!(\"worker 0 had {} no-op unparks\", n);"] #[doc = " # }"] #[doc =
        " ```"] pub fn worker_noop_count(& self, worker : usize) -> u64 { self.handle
        .inner.worker_metrics(worker).noop_count.load(Relaxed) } #[doc =
        " Returns the number of tasks the given worker thread stole from"] #[doc =
        " another worker thread."] #[doc = ""] #[doc =
        " This metric only applies to the **multi-threaded** runtime and will"] #[doc =
        " always return `0` when using the current thread runtime."] #[doc = ""] #[doc =
        " The worker steal count starts at zero when the runtime is created and"] #[doc =
        " increases by `N` each time the worker has processed its scheduled queue"] #[doc
        = " and successfully steals `N` more pending tasks from another worker."] #[doc =
        ""] #[doc =
        " The counter is monotonically increasing. It is never decremented or"] #[doc =
        " reset to zero."] #[doc = ""] #[doc = " # Arguments"] #[doc = ""] #[doc =
        " `worker` is the index of the worker being queried. The given value must"] #[doc
        = " be between 0 and `num_workers()`. The index uniquely identifies a single"]
        #[doc =
        " worker and will continue to identify the worker throughout the lifetime"] #[doc
        = " of the runtime instance."] #[doc = ""] #[doc = " # Panics"] #[doc = ""] #[doc
        = " The method panics when `worker` represents an invalid worker, i.e. is"] #[doc
        = " greater than or equal to `num_workers()`."] #[doc = ""] #[doc =
        " # Examples"] #[doc = ""] #[doc = " ```"] #[doc =
        " use tokio::runtime::Handle;"] #[doc = ""] #[doc =
        " # #[tokio::main(flavor = \"current_thread\")]"] #[doc = " # async fn main() {"]
        #[doc = " let metrics = Handle::current().metrics();"] #[doc = ""] #[doc =
        " let n = metrics.worker_steal_count(0);"] #[doc =
        " println!(\"worker 0 has stolen {} tasks\", n);"] #[doc = " # }"] #[doc =
        " ```"] pub fn worker_steal_count(& self, worker : usize) -> u64 { self.handle
        .inner.worker_metrics(worker).steal_count.load(Relaxed) } #[doc =
        " Returns the number of times the given worker thread stole tasks from"] #[doc =
        " another worker thread."] #[doc = ""] #[doc =
        " This metric only applies to the **multi-threaded** runtime and will"] #[doc =
        " always return `0` when using the current thread runtime."] #[doc = ""] #[doc =
        " The worker steal count starts at zero when the runtime is created and"] #[doc =
        " increases by one each time the worker has processed its scheduled queue"] #[doc
        = " and successfully steals more pending tasks from another worker."] #[doc = ""]
        #[doc = " The counter is monotonically increasing. It is never decremented or"]
        #[doc = " reset to zero."] #[doc = ""] #[doc = " # Arguments"] #[doc = ""] #[doc
        = " `worker` is the index of the worker being queried. The given value must"]
        #[doc =
        " be between 0 and `num_workers()`. The index uniquely identifies a single"]
        #[doc =
        " worker and will continue to identify the worker throughout the lifetime"] #[doc
        = " of the runtime instance."] #[doc = ""] #[doc = " # Panics"] #[doc = ""] #[doc
        = " The method panics when `worker` represents an invalid worker, i.e. is"] #[doc
        = " greater than or equal to `num_workers()`."] #[doc = ""] #[doc =
        " # Examples"] #[doc = ""] #[doc = " ```"] #[doc =
        " use tokio::runtime::Handle;"] #[doc = ""] #[doc =
        " # #[tokio::main(flavor = \"current_thread\")]"] #[doc = " # async fn main() {"]
        #[doc = " let metrics = Handle::current().metrics();"] #[doc = ""] #[doc =
        " let n = metrics.worker_steal_operations(0);"] #[doc =
        " println!(\"worker 0 has stolen tasks {} times\", n);"] #[doc = " # }"] #[doc =
        " ```"] pub fn worker_steal_operations(& self, worker : usize) -> u64 { self
        .handle.inner.worker_metrics(worker).steal_operations.load(Relaxed) } #[doc =
        " Returns the number of tasks the given worker thread has polled."] #[doc = ""]
        #[doc = " The worker poll count starts at zero when the runtime is created and"]
        #[doc = " increases by one each time the worker polls a scheduled task."] #[doc =
        ""] #[doc =
        " The counter is monotonically increasing. It is never decremented or"] #[doc =
        " reset to zero."] #[doc = ""] #[doc = " # Arguments"] #[doc = ""] #[doc =
        " `worker` is the index of the worker being queried. The given value must"] #[doc
        = " be between 0 and `num_workers()`. The index uniquely identifies a single"]
        #[doc =
        " worker and will continue to identify the worker throughout the lifetime"] #[doc
        = " of the runtime instance."] #[doc = ""] #[doc = " # Panics"] #[doc = ""] #[doc
        = " The method panics when `worker` represents an invalid worker, i.e. is"] #[doc
        = " greater than or equal to `num_workers()`."] #[doc = ""] #[doc =
        " # Examples"] #[doc = ""] #[doc = " ```"] #[doc =
        " use tokio::runtime::Handle;"] #[doc = ""] #[doc =
        " # #[tokio::main(flavor = \"current_thread\")]"] #[doc = " # async fn main() {"]
        #[doc = " let metrics = Handle::current().metrics();"] #[doc = ""] #[doc =
        " let n = metrics.worker_poll_count(0);"] #[doc =
        " println!(\"worker 0 has polled {} tasks\", n);"] #[doc = " # }"] #[doc =
        " ```"] pub fn worker_poll_count(& self, worker : usize) -> u64 { self.handle
        .inner.worker_metrics(worker).poll_count.load(Relaxed) } #[doc =
        " Returns the number of tasks scheduled from **within** the runtime on the"]
        #[doc = " given worker's local queue."] #[doc = ""] #[doc =
        " The local schedule count starts at zero when the runtime is created and"] #[doc
        = " increases by one each time a task is woken from **inside** of the"] #[doc =
        " runtime on the given worker. This usually means that a task is spawned"] #[doc
        = " or notified from within a runtime thread and will be queued on the"] #[doc =
        " worker-local queue."] #[doc = ""] #[doc =
        " The counter is monotonically increasing. It is never decremented or"] #[doc =
        " reset to zero."] #[doc = ""] #[doc = " # Arguments"] #[doc = ""] #[doc =
        " `worker` is the index of the worker being queried. The given value must"] #[doc
        = " be between 0 and `num_workers()`. The index uniquely identifies a single"]
        #[doc =
        " worker and will continue to identify the worker throughout the lifetime"] #[doc
        = " of the runtime instance."] #[doc = ""] #[doc = " # Panics"] #[doc = ""] #[doc
        = " The method panics when `worker` represents an invalid worker, i.e. is"] #[doc
        = " greater than or equal to `num_workers()`."] #[doc = ""] #[doc =
        " # Examples"] #[doc = ""] #[doc = " ```"] #[doc =
        " use tokio::runtime::Handle;"] #[doc = ""] #[doc =
        " # #[tokio::main(flavor = \"current_thread\")]"] #[doc = " # async fn main() {"]
        #[doc = " let metrics = Handle::current().metrics();"] #[doc = ""] #[doc =
        " let n = metrics.worker_local_schedule_count(0);"] #[doc =
        " println!(\"{} tasks were scheduled on the worker's local queue\", n);"] #[doc =
        " # }"] #[doc = " ```"] pub fn worker_local_schedule_count(& self, worker :
        usize) -> u64 { self.handle.inner.worker_metrics(worker).local_schedule_count
        .load(Relaxed) } #[doc =
        " Returns the number of times the given worker thread saturated its local"] #[doc
        = " queue."] #[doc = ""] #[doc =
        " This metric only applies to the **multi-threaded** scheduler."] #[doc = ""]
        #[doc =
        " The worker overflow count starts at zero when the runtime is created and"]
        #[doc = " increases by one each time the worker attempts to schedule a task"]
        #[doc = " locally, but its local queue is full. When this happens, half of the"]
        #[doc = " local queue is moved to the injection queue."] #[doc = ""] #[doc =
        " The counter is monotonically increasing. It is never decremented or"] #[doc =
        " reset to zero."] #[doc = ""] #[doc = " # Arguments"] #[doc = ""] #[doc =
        " `worker` is the index of the worker being queried. The given value must"] #[doc
        = " be between 0 and `num_workers()`. The index uniquely identifies a single"]
        #[doc =
        " worker and will continue to identify the worker throughout the lifetime"] #[doc
        = " of the runtime instance."] #[doc = ""] #[doc = " # Panics"] #[doc = ""] #[doc
        = " The method panics when `worker` represents an invalid worker, i.e. is"] #[doc
        = " greater than or equal to `num_workers()`."] #[doc = ""] #[doc =
        " # Examples"] #[doc = ""] #[doc = " ```"] #[doc =
        " use tokio::runtime::Handle;"] #[doc = ""] #[doc =
        " # #[tokio::main(flavor = \"current_thread\")]"] #[doc = " # async fn main() {"]
        #[doc = " let metrics = Handle::current().metrics();"] #[doc = ""] #[doc =
        " let n = metrics.worker_overflow_count(0);"] #[doc =
        " println!(\"worker 0 has overflowed its queue {} times\", n);"] #[doc = " # }"]
        #[doc = " ```"] pub fn worker_overflow_count(& self, worker : usize) -> u64 {
        self.handle.inner.worker_metrics(worker).overflow_count.load(Relaxed) } #[doc =
        " Returns the number of times the given worker polled tasks with a poll"] #[doc =
        " duration within the given bucket's range."] #[doc = ""] #[doc =
        " Each worker maintains its own histogram and the counts for each bucket"] #[doc
        = " starts at zero when the runtime is created. Each time the worker polls a"]
        #[doc =
        " task, it tracks the duration the task poll time took and increments the"] #[doc
        = " associated bucket by 1."] #[doc = ""] #[doc =
        " Each bucket is a monotonically increasing counter. It is never"] #[doc =
        " decremented or reset to zero."] #[doc = ""] #[doc = " # Arguments"] #[doc = ""]
        #[doc =
        " `worker` is the index of the worker being queried. The given value must"] #[doc
        = " be between 0 and `num_workers()`. The index uniquely identifies a single"]
        #[doc =
        " worker and will continue to identify the worker throughout the lifetime"] #[doc
        = " of the runtime instance."] #[doc = ""] #[doc =
        " `bucket` is the index of the bucket being queried. The bucket is scoped"] #[doc
        = " to the worker. The range represented by the bucket can be queried by"] #[doc
        = " calling [`poll_time_histogram_bucket_range()`]. Each worker maintains"] #[doc
        = " identical bucket ranges."] #[doc = ""] #[doc = " # Panics"] #[doc = ""] #[doc
        = " The method panics when `worker` represents an invalid worker, i.e. is"] #[doc
        = " greater than or equal to `num_workers()` or if `bucket` represents an"] #[doc
        = " invalid bucket."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc =
        " ```"] #[doc = " use tokio::runtime::{self, Handle};"] #[doc = ""] #[doc =
        " fn main() {"] #[doc = "     runtime::Builder::new_current_thread()"] #[doc =
        "         .enable_metrics_poll_time_histogram()"] #[doc = "         .build()"]
        #[doc = "         .unwrap()"] #[doc = "         .block_on(async {"] #[doc =
        "             let metrics = Handle::current().metrics();"] #[doc =
        "             let buckets = metrics.poll_time_histogram_num_buckets();"] #[doc =
        ""] #[doc = "             for worker in 0..metrics.num_workers() {"] #[doc =
        "                 for i in 0..buckets {"] #[doc =
        "                     let count = metrics.poll_time_histogram_bucket_count(worker, i);"]
        #[doc = "                     println!(\"Poll count {}\", count);"] #[doc =
        "                 }"] #[doc = "             }"] #[doc = "         });"] #[doc =
        " }"] #[doc = " ```"] #[doc = ""] #[doc =
        " [`poll_time_histogram_bucket_range()`]: crate::runtime::RuntimeMetrics::poll_time_histogram_bucket_range"]
        #[track_caller] pub fn poll_time_histogram_bucket_count(& self, worker : usize,
        bucket : usize) -> u64 { self.handle.inner.worker_metrics(worker)
        .poll_count_histogram.as_ref().map(| histogram | histogram.get(bucket))
        .unwrap_or_default() } #[doc(hidden)] #[deprecated(note =
        "use `poll_time_histogram_bucket_count` instead")] pub fn
        poll_count_histogram_bucket_count(& self, worker : usize, bucket : usize) -> u64
        { self.poll_time_histogram_bucket_count(worker, bucket) } #[doc =
        " Returns the mean duration of task polls, in nanoseconds."] #[doc = ""] #[doc =
        " This is an exponentially weighted moving average. Currently, this metric"]
        #[doc = " is only provided by the multi-threaded runtime."] #[doc = ""] #[doc =
        " # Arguments"] #[doc = ""] #[doc =
        " `worker` is the index of the worker being queried. The given value must"] #[doc
        = " be between 0 and `num_workers()`. The index uniquely identifies a single"]
        #[doc =
        " worker and will continue to identify the worker throughout the lifetime"] #[doc
        = " of the runtime instance."] #[doc = ""] #[doc = " # Panics"] #[doc = ""] #[doc
        = " The method panics when `worker` represents an invalid worker, i.e. is"] #[doc
        = " greater than or equal to `num_workers()`."] #[doc = ""] #[doc =
        " # Examples"] #[doc = ""] #[doc = " ```"] #[doc =
        " use tokio::runtime::Handle;"] #[doc = ""] #[doc =
        " # #[tokio::main(flavor = \"current_thread\")]"] #[doc = " # async fn main() {"]
        #[doc = " let metrics = Handle::current().metrics();"] #[doc = ""] #[doc =
        " let n = metrics.worker_mean_poll_time(0);"] #[doc =
        " println!(\"worker 0 has a mean poll time of {:?}\", n);"] #[doc = " # }"] #[doc
        = " ```"] #[track_caller] pub fn worker_mean_poll_time(& self, worker : usize) ->
        Duration { let nanos = self.handle.inner.worker_metrics(worker).mean_poll_time
        .load(Relaxed); Duration::from_nanos(nanos) }
    }
    feature! {
        #![feature = "schedule-latency"] #[doc =
        " Returns `true` if the runtime is tracking the distribution of task"] #[doc =
        " schedule latencies."] #[doc = ""] #[doc =
        " Task schedule latencies are not instrumented by default as doing so"] #[doc =
        " requires calling [`Instant::now()`] when a task is scheduled and when"] #[doc =
        " it is polled. The feature is enabled by calling"] #[doc =
        " [`enable_metrics_schedule_latency_histogram()`] when building the runtime."]
        #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc =
        " use tokio::runtime::{self, Handle};"] #[doc = ""] #[doc = " fn main() {"] #[doc
        = "     runtime::Builder::new_current_thread()"] #[doc =
        "         .enable_metrics_schedule_latency_histogram()"] #[doc =
        "         .build()"] #[doc = "         .unwrap()"] #[doc =
        "         .block_on(async {"] #[doc =
        "             let metrics = Handle::current().metrics();"] #[doc =
        "             let enabled = metrics.schedule_latency_histogram_enabled();"] #[doc
        = ""] #[doc =
        "             println!(\"Tracking task schedule latency distribution: {:?}\", enabled);"]
        #[doc = "         });"] #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc =
        " [`enable_metrics_schedule_latency_histogram()`]: crate::runtime::Builder::enable_metrics_schedule_latency_histogram"]
        #[doc = " [`Instant::now()`]: std::time::Instant::now"] pub fn
        schedule_latency_histogram_enabled(& self) -> bool { self.handle.inner
        .worker_metrics(0).schedule_latency_histogram.is_some() } #[doc =
        " Returns the number of histogram buckets tracking the distribution of"] #[doc =
        " task schedule latencies."] #[doc = ""] #[doc =
        " This value is configured by calling"] #[doc =
        " [`metrics_schedule_latency_histogram_configuration()`] when building the runtime."]
        #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc =
        " use tokio::runtime::{self, Handle};"] #[doc = ""] #[doc = " fn main() {"] #[doc
        = "     runtime::Builder::new_current_thread()"] #[doc =
        "         .enable_metrics_schedule_latency_histogram()"] #[doc =
        "         .build()"] #[doc = "         .unwrap()"] #[doc =
        "         .block_on(async {"] #[doc =
        "             let metrics = Handle::current().metrics();"] #[doc =
        "             let buckets = metrics.schedule_latency_histogram_num_buckets();"]
        #[doc = ""] #[doc =
        "             println!(\"Histogram buckets: {:?}\", buckets);"] #[doc =
        "         });"] #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc =
        " [`metrics_schedule_latency_histogram_configuration()`]: crate::runtime::Builder::metrics_schedule_latency_histogram_configuration"]
        pub fn schedule_latency_histogram_num_buckets(& self) -> usize { self.handle
        .inner.worker_metrics(0).schedule_latency_histogram.as_ref().map(| histogram |
        histogram.num_buckets()).unwrap_or_default() } #[doc =
        " Returns the range of task schedule latencies tracked by the given bucket."]
        #[doc = ""] #[doc = " This value is configured by calling"] #[doc =
        " [`metrics_schedule_latency_histogram_configuration()`] when building the runtime."]
        #[doc = ""] #[doc = " # Panics"] #[doc = ""] #[doc =
        " The method panics if `bucket` represents an invalid bucket index, i.e."] #[doc
        = " is greater than or equal to `schedule_latency_histogram_num_buckets()`."]
        #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```"] #[doc =
        " use tokio::runtime::{self, Handle};"] #[doc = ""] #[doc = " fn main() {"] #[doc
        = "     runtime::Builder::new_current_thread()"] #[doc =
        "         .enable_metrics_schedule_latency_histogram()"] #[doc =
        "         .build()"] #[doc = "         .unwrap()"] #[doc =
        "         .block_on(async {"] #[doc =
        "             let metrics = Handle::current().metrics();"] #[doc =
        "             let buckets = metrics.schedule_latency_histogram_num_buckets();"]
        #[doc = ""] #[doc = "             for i in 0..buckets {"] #[doc =
        "                 let range = metrics.schedule_latency_histogram_bucket_range(i);"]
        #[doc =
        "                 println!(\"Histogram bucket {} range: {:?}\", i, range);"]
        #[doc = "             }"] #[doc = "         });"] #[doc = " }"] #[doc = " ```"]
        #[doc = ""] #[doc =
        " [`metrics_schedule_latency_histogram_configuration()`]: crate::runtime::Builder::metrics_schedule_latency_histogram_configuration"]
        #[track_caller] pub fn schedule_latency_histogram_bucket_range(& self, bucket :
        usize) -> Range < Duration > { self.handle.inner.worker_metrics(0)
        .schedule_latency_histogram.as_ref().map(| histogram | { let range = histogram
        .bucket_range(bucket); std::ops::Range { start : Duration::from_nanos(range
        .start), end : Duration::from_nanos(range.end), } }).unwrap_or_default() } #[doc
        = " Returns the number of times the given worker polled tasks with a schedule"]
        #[doc = " latency within the given bucket's range."] #[doc = ""] #[doc =
        " Each worker maintains its own histogram and the counts for each bucket"] #[doc
        = " starts at zero when the runtime is created. Each time the worker polls a"]
        #[doc =
        " task, it tracks the time elapsed between when the task was scheduled and"]
        #[doc = " when it was polled and increments the associated bucket by 1."] #[doc =
        ""] #[doc = " Each bucket is a monotonically increasing counter. It is never"]
        #[doc = " decremented or reset to zero."] #[doc = ""] #[doc = " # Arguments"]
        #[doc = ""] #[doc =
        " `worker` is the index of the worker being queried. The given value must"] #[doc
        = " be between 0 and `num_workers()`. The index uniquely identifies a single"]
        #[doc =
        " worker and will continue to identify the worker throughout the lifetime"] #[doc
        = " of the runtime instance."] #[doc = ""] #[doc =
        " `bucket` is the index of the bucket being queried. The bucket is scoped"] #[doc
        = " to the worker. The range represented by the bucket can be queried by"] #[doc
        =
        " calling [`schedule_latency_histogram_bucket_range()`]. Each worker maintains"]
        #[doc = " identical bucket ranges."] #[doc = ""] #[doc = " # Panics"] #[doc = ""]
        #[doc = " The method panics when `worker` represents an invalid worker, i.e. is"]
        #[doc = " greater than or equal to `num_workers()` or if `bucket` represents an"]
        #[doc = " invalid bucket."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc
        = " ```"] #[doc = " use tokio::runtime::{self, Handle};"] #[doc = ""] #[doc =
        " fn main() {"] #[doc = "     runtime::Builder::new_current_thread()"] #[doc =
        "         .enable_metrics_schedule_latency_histogram()"] #[doc =
        "         .build()"] #[doc = "         .unwrap()"] #[doc =
        "         .block_on(async {"] #[doc =
        "             let metrics = Handle::current().metrics();"] #[doc =
        "             let buckets = metrics.schedule_latency_histogram_num_buckets();"]
        #[doc = ""] #[doc = "             for worker in 0..metrics.num_workers() {"]
        #[doc = "                 for i in 0..buckets {"] #[doc =
        "                     let range = metrics.schedule_latency_histogram_bucket_range(i);"]
        #[doc =
        "                     let count = metrics.schedule_latency_histogram_bucket_count(worker, i);"]
        #[doc =
        "                     println!(\"{} tasks encountered a scheduling latency between {}us and {}us\", count, range.start.as_micros(), range.end.as_micros());"]
        #[doc = "                 }"] #[doc = "             }"] #[doc = "         });"]
        #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc =
        " [`schedule_latency_histogram_bucket_range()`]: crate::runtime::RuntimeMetrics::schedule_latency_histogram_bucket_range"]
        #[track_caller] pub fn schedule_latency_histogram_bucket_count(& self, worker :
        usize, bucket : usize) -> u64 { self.handle.inner.worker_metrics(worker)
        .schedule_latency_histogram.as_ref().map(| histogram | histogram.get(bucket))
        .unwrap_or_default() }
    }
    feature! {
        #![all(tokio_unstable, target_has_atomic = "64", feature = "net")] #[doc =
        " Returns the number of file descriptors that have been registered with the"]
        #[doc = " runtime's I/O driver."] #[doc = ""] #[doc = " # Examples"] #[doc = ""]
        #[doc = " ```"] #[doc = " use tokio::runtime::Handle;"] #[doc = ""] #[doc =
        " #[tokio::main]"] #[doc = " async fn main() {"] #[doc =
        "     let metrics = Handle::current().metrics();"] #[doc = ""] #[doc =
        "     let registered_fds = metrics.io_driver_fd_registered_count();"] #[doc =
        "     println!(\"{} fds have been registered with the runtime's I/O driver.\", registered_fds);"]
        #[doc = ""] #[doc =
        "     let deregistered_fds = metrics.io_driver_fd_deregistered_count();"] #[doc =
        ""] #[doc = "     let current_fd_count = registered_fds - deregistered_fds;"]
        #[doc =
        "     println!(\"{} fds are currently registered by the runtime's I/O driver.\", current_fd_count);"]
        #[doc = " }"] #[doc = " ```"] pub fn io_driver_fd_registered_count(& self) -> u64
        { self.with_io_driver_metrics(| m | { m.fd_registered_count.load(Relaxed) }) }
        #[doc =
        " Returns the number of file descriptors that have been deregistered by the"]
        #[doc = " runtime's I/O driver."] #[doc = ""] #[doc = " # Examples"] #[doc = ""]
        #[doc = " ```"] #[doc = " use tokio::runtime::Handle;"] #[doc = ""] #[doc =
        " #[tokio::main]"] #[doc = " async fn main() {"] #[doc =
        "     let metrics = Handle::current().metrics();"] #[doc = ""] #[doc =
        "     let n = metrics.io_driver_fd_deregistered_count();"] #[doc =
        "     println!(\"{} fds have been deregistered by the runtime's I/O driver.\", n);"]
        #[doc = " }"] #[doc = " ```"] pub fn io_driver_fd_deregistered_count(& self) ->
        u64 { self.with_io_driver_metrics(| m | { m.fd_deregistered_count.load(Relaxed)
        }) } #[doc = " Returns the number of ready events processed by the runtime's"]
        #[doc = " I/O driver."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc =
        " ```"] #[doc = " use tokio::runtime::Handle;"] #[doc = ""] #[doc =
        " #[tokio::main]"] #[doc = " async fn main() {"] #[doc =
        "     let metrics = Handle::current().metrics();"] #[doc = ""] #[doc =
        "     let n = metrics.io_driver_ready_count();"] #[doc =
        "     println!(\"{} ready events processed by the runtime's I/O driver.\", n);"]
        #[doc = " }"] #[doc = " ```"] pub fn io_driver_ready_count(& self) -> u64 { self
        .with_io_driver_metrics(| m | m.ready_count.load(Relaxed)) } fn
        with_io_driver_metrics < F > (& self, f : F) -> u64 where F : Fn(&
        super::IoDriverMetrics) -> u64, { self.handle.inner.driver().io.as_ref().map(| h
        | f(& h.metrics)).unwrap_or(0) }
    }
}
