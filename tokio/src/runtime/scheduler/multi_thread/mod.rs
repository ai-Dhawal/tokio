//! Multi-threaded runtime
mod counters;
use counters::Counters;
mod handle;
pub(crate) use handle::Handle;
mod overflow;
pub(crate) use overflow::Overflow;
mod idle;
use self::idle::Idle;
mod stats;
pub(crate) use stats::Stats;
mod park;
pub(crate) use park::{Parker, Unparker};
pub(crate) mod queue;
mod worker;
pub(crate) use worker::{Context, Launch, Shared};
cfg_taskdump! {
    mod trace; use trace::TraceStatus; pub (crate) use worker::Synced;
}
cfg_not_taskdump! {
    mod trace_mock; use trace_mock::TraceStatus;
}
pub(crate) use worker::block_in_place;
use crate::loom::sync::Arc;
use crate::runtime::{
    blocking, driver::{self, Driver},
    scheduler, Config, TimerFlavor,
};
use crate::util::RngSeedGenerator;
use std::fmt;
use std::future::Future;
/// Work-stealing based thread pool for executing futures.
pub(crate) struct MultiThread;
impl MultiThread {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        size: usize,
        driver: Driver,
        driver_handle: driver::Handle,
        blocking_spawner: blocking::Spawner,
        seed_generator: RngSeedGenerator,
        config: Config,
        timer_flavor: TimerFlavor,
        name: Option<String>,
    ) -> (MultiThread, Arc<Handle>, Launch) {
        panic!("STUB: not implemented");
    }
    /// Blocks the current thread waiting for the future to complete.
    ///
    /// The future will execute on the current thread, but all spawned tasks
    /// will be executed on the thread pool.
    pub(crate) fn block_on<F>(&self, handle: &scheduler::Handle, future: F) -> F::Output
    where
        F: Future,
    {
        panic!("STUB: not implemented");
    }
    pub(crate) fn shutdown(&mut self, handle: &scheduler::Handle) {
        panic!("STUB: not implemented");
    }
}
impl fmt::Debug for MultiThread {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
