#[cfg(feature = "test-util")]
use crate::runtime::scheduler;
use crate::runtime::task::{self, Task, TaskHarnessScheduleHooks};
use crate::runtime::Handle;
/// `task::Schedule` implementation that does nothing (except some bookkeeping
/// in test-util builds). This is unique to the blocking scheduler as tasks
/// scheduled are not really futures but blocking operations.
///
/// We avoid storing the task by forgetting it in `bind` and re-materializing it
/// in `release`.
pub(crate) struct BlockingSchedule {
    #[cfg(feature = "test-util")]
    handle: Handle,
    hooks: TaskHarnessScheduleHooks,
}
impl BlockingSchedule {
    #[cfg_attr(not(feature = "test-util"), allow(unused_variables))]
    pub(crate) fn new(handle: &Handle) -> Self {
        panic!("STUB: not implemented");
    }
}
impl task::Schedule for BlockingSchedule {
    fn release(&self, _task: &Task<Self>) -> Option<Task<Self>> {
        panic!("STUB: not implemented");
    }
    fn schedule(&self, _task: task::Notified<Self>) {
        panic!("STUB: not implemented");
    }
    fn hooks(&self) -> TaskHarnessScheduleHooks {
        panic!("STUB: not implemented");
    }
}
