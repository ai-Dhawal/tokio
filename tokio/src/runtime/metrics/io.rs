#![cfg_attr(not(feature = "net"), allow(dead_code))]
use crate::util::metric_atomics::MetricAtomicU64;
use std::sync::atomic::Ordering::Relaxed;
#[derive(Default)]
pub(crate) struct IoDriverMetrics {
    pub(super) fd_registered_count: MetricAtomicU64,
    pub(super) fd_deregistered_count: MetricAtomicU64,
    pub(super) ready_count: MetricAtomicU64,
}
impl IoDriverMetrics {
    pub(crate) fn incr_fd_count(&self) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn dec_fd_count(&self) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn incr_ready_count_by(&self, amt: u64) {
        panic!("STUB: not implemented");
    }
}
