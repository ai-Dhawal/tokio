use super::Config;
use std::marker::PhantomData;
impl TaskHooks {
    pub(crate) fn spawn(&self, meta: &TaskMeta<'_>) {
        panic!("STUB: not implemented");
    }
    #[allow(dead_code)]
    pub(crate) fn from_config(config: &Config) -> Self {
        panic!("STUB: not implemented");
    }
    #[cfg(tokio_unstable)]
    #[inline]
    pub(crate) fn poll_start_callback(&self, meta: &TaskMeta<'_>) {
        panic!("STUB: not implemented");
    }
    #[cfg(tokio_unstable)]
    #[inline]
    pub(crate) fn poll_stop_callback(&self, meta: &TaskMeta<'_>) {
        panic!("STUB: not implemented");
    }
}
#[derive(Clone)]
pub(crate) struct TaskHooks {
    pub(crate) task_spawn_callback: Option<TaskCallback>,
    pub(crate) task_terminate_callback: Option<TaskCallback>,
    #[cfg(tokio_unstable)]
    pub(crate) before_poll_callback: Option<TaskCallback>,
    #[cfg(tokio_unstable)]
    pub(crate) after_poll_callback: Option<TaskCallback>,
}
/// Task metadata supplied to user-provided hooks for task events.
///
/// **Note**: This is an [unstable API][unstable]. The public API of this type
/// may break in 1.x releases. See [the documentation on unstable
/// features][unstable] for details.
///
/// [unstable]: crate#unstable-features
#[allow(missing_debug_implementations)]
#[cfg_attr(not(tokio_unstable), allow(unreachable_pub))]
pub struct TaskMeta<'a> {
    /// The opaque ID of the task.
    pub(crate) id: super::task::Id,
    /// The location where the task was spawned.
    #[cfg_attr(not(tokio_unstable), allow(unreachable_pub, dead_code))]
    pub(crate) spawned_at: crate::runtime::task::SpawnLocation,
    pub(crate) _phantom: PhantomData<&'a ()>,
}
impl<'a> TaskMeta<'a> {
    /// Return the opaque ID of the task.
    #[cfg_attr(not(tokio_unstable), allow(unreachable_pub, dead_code))]
    pub fn id(&self) -> super::task::Id {
        panic!("STUB: not implemented");
    }
    /// Return the source code location where the task was spawned.
    #[cfg(tokio_unstable)]
    pub fn spawned_at(&self) -> &'static std::panic::Location<'static> {
        panic!("STUB: not implemented");
    }
}
/// Runs on specific task-related events
pub(crate) type TaskCallback = std::sync::Arc<dyn Fn(&TaskMeta<'_>) + Send + Sync>;
