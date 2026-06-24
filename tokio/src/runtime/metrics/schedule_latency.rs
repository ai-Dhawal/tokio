use std::num::NonZeroU64;
use std::time::Instant;
/// `ScheduleLatencyInstant` tracks the time a task was scheduled.
///
/// The time a task was scheduled is stored as the number of nanoseconds
/// since startup of the task's scheduler.
///
/// **Note**: This is an [unstable API][unstable]. The public API of this type
/// may break in 1.x releases. See [the documentation on unstable
/// features][unstable] for details.
///
/// [unstable]: crate#unstable-features
#[derive(Copy, Clone)]
pub(crate) struct ScheduleLatencyInstant(Option<NonZeroU64>);
impl ScheduleLatencyInstant {
    /// Create a new `ScheduleLatencyInstant` using the provided scheduler startup Instant.
    pub(crate) fn new(scheduler_start: Option<Instant>) -> Self {
        panic!("STUB: not implemented");
    }
    /// Prepare a context that can calculate the number of nanoseconds elapsed
    /// since this task was scheduled.
    pub(crate) fn prepare(
        self,
        scheduler_start: Option<Instant>,
    ) -> Option<ScheduleLatencyContext> {
        panic!("STUB: not implemented");
    }
}
/// `ScheduleLatencyContext` contains all the data required to calculate the time elapsed
/// since a task was scheduled.
///
/// `ScheduleLatencyInstant` on its own in insufficient because it only contains a delta.
/// The scheduler startup time is required to convert the delta back into an actual time
/// but is omitted from `ScheduleLatencyInstant` to keep its memory size minimal.
pub(crate) struct ScheduleLatencyContext {
    scheduler_start: Instant,
    scheduled_at_delta: NonZeroU64,
}
impl ScheduleLatencyContext {
    /// Calculate how many nanoseconds have elapsed between `now` and when this task
    /// was last scheduled.
    pub(crate) fn elapsed_nanos(&self, now: Instant) -> u64 {
        panic!("STUB: not implemented");
    }
}
