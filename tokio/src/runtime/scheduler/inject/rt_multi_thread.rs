use super::{Shared, Synced};
use crate::runtime::scheduler::Lock;
use crate::runtime::task;
use std::sync::atomic::Ordering::Release;
impl<'a> Lock<Synced> for &'a mut Synced {
    type Handle = &'a mut Synced;
    fn lock(self) -> Self::Handle {
        panic!("STUB: not implemented");
    }
}
impl AsMut<Synced> for Synced {
    fn as_mut(&mut self) -> &mut Synced {
        panic!("STUB: not implemented");
    }
}
impl<T: 'static> Shared<T> {
    /// Pushes several values into the queue.
    ///
    /// # Safety
    ///
    /// Must be called with the same `Synced` instance returned by `Inject::new`
    #[inline]
    pub(crate) unsafe fn push_batch<L, I>(&self, shared: L, mut iter: I)
    where
        L: Lock<Synced>,
        I: Iterator<Item = task::Notified<T>>,
    {
        panic!("STUB: not implemented");
    }
    /// Inserts several tasks that have been linked together into the queue.
    ///
    /// The provided head and tail may be the same task. In this case, a
    /// single task is inserted.
    ///
    /// # Safety
    ///
    /// Must be called with the same `Synced` instance returned by `Inject::new`
    #[inline]
    unsafe fn push_batch_inner<L>(
        &self,
        shared: L,
        batch_head: task::RawTask,
        batch_tail: task::RawTask,
        num: usize,
    )
    where
        L: Lock<Synced>,
    {
        panic!("STUB: not implemented");
    }
}
