//! This module has containers for storing the tasks spawned on a scheduler. The
//! `OwnedTasks` container is thread-safe but can only store tasks that
//! implement Send. The `LocalOwnedTasks` container is not thread safe, but can
//! store non-Send tasks.
//!
//! The collections can be closed to prevent adding new tasks during shutdown of
//! the scheduler with the collection.
use crate::future::Future;
use crate::loom::cell::UnsafeCell;
use crate::runtime::task::{
    JoinHandle, LocalNotified, Notified, Schedule, SpawnLocation, Task,
};
use crate::util::linked_list::LinkedList;
use crate::util::sharded_list::ShardedList;
use crate::loom::sync::atomic::{AtomicBool, Ordering};
use std::marker::PhantomData;
use std::num::NonZeroU64;
cfg_has_atomic_u64! {
    use std::sync::atomic::AtomicU64; static NEXT_OWNED_TASKS_ID : AtomicU64 =
    AtomicU64::new(1); fn get_next_id() -> NonZeroU64 { loop { let id =
    NEXT_OWNED_TASKS_ID.fetch_add(1, Ordering::Relaxed); if let Some(id) =
    NonZeroU64::new(id) { return id; } } }
}
cfg_not_has_atomic_u64! {
    use std::sync::atomic::AtomicU32; static NEXT_OWNED_TASKS_ID : AtomicU32 =
    AtomicU32::new(1); fn get_next_id() -> NonZeroU64 { loop { let id =
    NEXT_OWNED_TASKS_ID.fetch_add(1, Ordering::Relaxed); if let Some(id) =
    NonZeroU64::new(u64::from(id)) { return id; } } }
}
pub(crate) struct OwnedTasks<S: 'static> {
    list: ShardedList<Task<S>>,
    pub(crate) id: NonZeroU64,
    closed: AtomicBool,
}
pub(crate) struct LocalOwnedTasks<S: 'static> {
    inner: UnsafeCell<OwnedTasksInner<S>>,
    pub(crate) id: NonZeroU64,
    _not_send_or_sync: PhantomData<*const ()>,
}
struct OwnedTasksInner<S: 'static> {
    list: LinkedList<Task<S>>,
    closed: bool,
}
impl<S: 'static> OwnedTasks<S> {
    pub(crate) fn new(num_cores: usize) -> Self {
        panic!("STUB: not implemented");
    }
    /// Binds the provided task to this `OwnedTasks` instance. This fails if the
    /// `OwnedTasks` has been closed.
    pub(crate) fn bind<T>(
        &self,
        task: T,
        scheduler: S,
        id: super::Id,
        spawned_at: SpawnLocation,
    ) -> (JoinHandle<T::Output>, Option<Notified<S>>)
    where
        S: Schedule,
        T: Future + Send + 'static,
        T::Output: Send + 'static,
    {
        panic!("STUB: not implemented");
    }
    /// Bind a task that isn't safe to transfer across thread boundaries.
    ///
    /// # Safety
    ///
    /// Only use this in `LocalRuntime` where the task cannot move
    pub(crate) unsafe fn bind_local<T>(
        &self,
        task: T,
        scheduler: S,
        id: super::Id,
        spawned_at: SpawnLocation,
    ) -> (JoinHandle<T::Output>, Option<Notified<S>>)
    where
        S: Schedule,
        T: Future + 'static,
        T::Output: 'static,
    {
        panic!("STUB: not implemented");
    }
    /// The part of `bind` that's the same for every type of future.
    unsafe fn bind_inner(
        &self,
        task: Task<S>,
        notified: Notified<S>,
    ) -> Option<Notified<S>>
    where
        S: Schedule,
    {
        panic!("STUB: not implemented");
    }
    /// Asserts that the given task is owned by this `OwnedTasks` and convert it to
    /// a `LocalNotified`, giving the thread permission to poll this task.
    #[inline]
    pub(crate) fn assert_owner(&self, task: Notified<S>) -> LocalNotified<S> {
        panic!("STUB: not implemented");
    }
    /// Shuts down all tasks in the collection. This call also closes the
    /// collection, preventing new items from being added.
    ///
    /// The parameter start determines which shard this method will start at.
    /// Using different values for each worker thread reduces contention.
    pub(crate) fn close_and_shutdown_all(&self, start: usize)
    where
        S: Schedule,
    {
        panic!("STUB: not implemented");
    }
    #[inline]
    pub(crate) fn get_shard_size(&self) -> usize {
        panic!("STUB: not implemented");
    }
    pub(crate) fn num_alive_tasks(&self) -> usize {
        panic!("STUB: not implemented");
    }
    cfg_unstable_metrics! {
        cfg_64bit_metrics! { pub (crate) fn spawned_tasks_count(& self) -> u64 { self
        .list.added() } }
    }
    pub(crate) fn remove(&self, task: &Task<S>) -> Option<Task<S>> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn is_empty(&self) -> bool {
        panic!("STUB: not implemented");
    }
    /// Generates the size of the sharded list based on the number of worker threads.
    ///
    /// The sharded lock design can effectively alleviate
    /// lock contention performance problems caused by high concurrency.
    ///
    /// However, as the number of shards increases, the memory continuity between
    /// nodes in the intrusive linked list will diminish. Furthermore,
    /// the construction time of the sharded list will also increase with a higher number of shards.
    ///
    /// Due to the above reasons, we set a maximum value for the shared list size,
    /// denoted as `MAX_SHARED_LIST_SIZE`.
    fn gen_shared_list_size(num_cores: usize) -> usize {
        panic!("STUB: not implemented");
    }
}
cfg_taskdump! {
    impl < S : 'static > OwnedTasks < S > { #[doc =
    " Locks the tasks, and calls `f` on an iterator over them."] pub (crate) fn for_each
    < F > (& self, f : F) where F : FnMut(& Task < S >), { self.list.for_each(f); } }
}
impl<S: 'static> LocalOwnedTasks<S> {
    pub(crate) fn new() -> Self {
        panic!("STUB: not implemented");
    }
    pub(crate) fn bind<T>(
        &self,
        task: T,
        scheduler: S,
        id: super::Id,
        spawned_at: SpawnLocation,
    ) -> (JoinHandle<T::Output>, Option<Notified<S>>)
    where
        S: Schedule,
        T: Future + 'static,
        T::Output: 'static,
    {
        panic!("STUB: not implemented");
    }
    /// Shuts down all tasks in the collection. This call also closes the
    /// collection, preventing new items from being added.
    pub(crate) fn close_and_shutdown_all(&self)
    where
        S: Schedule,
    {
        panic!("STUB: not implemented");
    }
    pub(crate) fn remove(&self, task: &Task<S>) -> Option<Task<S>> {
        panic!("STUB: not implemented");
    }
    /// Asserts that the given task is owned by this `LocalOwnedTasks` and convert
    /// it to a `LocalNotified`, giving the thread permission to poll this task.
    #[inline]
    pub(crate) fn assert_owner(&self, task: Notified<S>) -> LocalNotified<S> {
        panic!("STUB: not implemented");
    }
    #[inline]
    fn with_inner<F, T>(&self, f: F) -> T
    where
        F: FnOnce(&mut OwnedTasksInner<S>) -> T,
    {
        panic!("STUB: not implemented");
    }
    pub(crate) fn is_closed(&self) -> bool {
        panic!("STUB: not implemented");
    }
    pub(crate) fn is_empty(&self) -> bool {
        panic!("STUB: not implemented");
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_id_not_broken() {
        let mut last_id = get_next_id();
        for _ in 0..1000 {
            let next_id = get_next_id();
            assert!(last_id < next_id);
            last_id = next_id;
        }
    }
}
