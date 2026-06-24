use super::MAX_SAFE_MILLIS_DURATION;
use crate::time::{Clock, Duration, Instant};
/// A structure which handles conversion from Instants to `u64` timestamps.
#[derive(Debug)]
pub(crate) struct TimeSource {
    start_time: Instant,
}
impl TimeSource {
    pub(crate) fn new(clock: &Clock) -> Self {
        panic!("STUB: not implemented");
    }
    pub(crate) fn deadline_to_tick(&self, t: Instant) -> u64 {
        panic!("STUB: not implemented");
    }
    pub(crate) fn instant_to_tick(&self, t: Instant) -> u64 {
        panic!("STUB: not implemented");
    }
    pub(crate) fn tick_to_duration(&self, t: u64) -> Duration {
        panic!("STUB: not implemented");
    }
    pub(crate) fn now(&self, clock: &Clock) -> u64 {
        panic!("STUB: not implemented");
    }
    #[cfg(test)]
    #[allow(dead_code)]
    pub(super) fn start_time(&self) -> Instant {
        self.start_time
    }
}
