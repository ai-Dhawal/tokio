use crate::runtime::metrics::ScheduleLatencyContext;
use crate::runtime::{Config, MetricsBatch, WorkerMetrics};
use std::time::{Duration, Instant};
/// Per-worker statistics. This is used for both tuning the scheduler and
/// reporting runtime-level metrics/stats.
pub(crate) struct Stats {
    /// The metrics batch used to report runtime-level metrics/stats to the
    /// user.
    batch: MetricsBatch,
    /// Instant at which work last resumed (continued after park).
    ///
    /// This duplicates the value stored in `MetricsBatch`. We will unify
    /// `Stats` and `MetricsBatch` when we stabilize metrics.
    processing_scheduled_tasks_started_at: Instant,
    /// Number of tasks polled in the batch of scheduled tasks
    tasks_polled_in_batch: usize,
    /// Exponentially-weighted moving average of time spent polling scheduled a
    /// task.
    ///
    /// Tracked in nanoseconds, stored as a `f64` since that is what we use with
    /// the EWMA calculations
    task_poll_time_ewma: f64,
}
/// How to weigh each individual poll time, value is plucked from thin air.
const TASK_POLL_TIME_EWMA_ALPHA: f64 = 0.1;
/// Ideally, we wouldn't go above this, value is plucked from thin air.
const TARGET_GLOBAL_QUEUE_INTERVAL: f64 = Duration::from_micros(200).as_nanos() as f64;
/// Max value for the global queue interval. This is 2x the previous default
const MAX_TASKS_POLLED_PER_GLOBAL_QUEUE_INTERVAL: u32 = 127;
/// This is the previous default
const TARGET_TASKS_POLLED_PER_GLOBAL_QUEUE_INTERVAL: u32 = 61;
impl Stats {
    pub(crate) fn new(worker_metrics: &WorkerMetrics) -> Stats {
        panic!("STUB: not implemented");
    }
    pub(crate) fn tuned_global_queue_interval(&self, config: &Config) -> u32 {
        panic!("STUB: not implemented");
    }
    pub(crate) fn submit(&mut self, to: &WorkerMetrics) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn about_to_park(&mut self) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn unparked(&mut self) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn inc_local_schedule_count(&mut self) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn start_processing_scheduled_tasks(&mut self) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn end_processing_scheduled_tasks(&mut self) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn start_poll(
        &mut self,
        task_scheduled_at: Option<ScheduleLatencyContext>,
    ) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn end_poll(&mut self) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn incr_steal_count(&mut self, by: u16) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn incr_steal_operations(&mut self) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn incr_overflow_count(&mut self) {
        panic!("STUB: not implemented");
    }
}
