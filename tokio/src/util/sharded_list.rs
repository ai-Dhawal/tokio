use std::ptr::NonNull;
use std::sync::atomic::Ordering;
use crate::loom::sync::{Mutex, MutexGuard};
use crate::util::metric_atomics::{MetricAtomicU64, MetricAtomicUsize};
use super::linked_list::{Link, LinkedList};
/// An intrusive linked list supporting highly concurrent updates.
///
/// It currently relies on `LinkedList`, so it is the caller's
/// responsibility to ensure the list is empty before dropping it.
///
/// Note: Due to its inner sharded design, the order of nodes cannot be guaranteed.
pub(crate) struct ShardedList<L: ShardedListItem> {
    lists: Box<[Mutex<LinkedList<L>>]>,
    added: MetricAtomicU64,
    count: MetricAtomicUsize,
    shard_mask: usize,
}
/// Determines which linked list an item should be stored in.
///
/// # Safety
///
/// Implementations must guarantee that the id of an item does not change from
/// call to call.
pub(crate) unsafe trait ShardedListItem: Link {
    /// # Safety
    ///
    /// The provided pointer must point at a valid list item.
    unsafe fn get_shard_id(target: NonNull<Self::Target>) -> usize;
}
/// Used to get the lock of shard.
pub(crate) struct ShardGuard<'a, L: Link> {
    lock: MutexGuard<'a, LinkedList<L>>,
    added: &'a MetricAtomicU64,
    count: &'a MetricAtomicUsize,
    id: usize,
}
impl<L: ShardedListItem> ShardedList<L> {
    /// Creates a new and empty sharded linked list with the specified size.
    pub(crate) fn new(sharded_size: usize) -> Self {
        panic!("STUB: not implemented");
    }
    /// Removes the last element from a list specified by `shard_id` and returns it, or None if it is
    /// empty.
    pub(crate) fn pop_back(&self, shard_id: usize) -> Option<L::Handle> {
        panic!("STUB: not implemented");
    }
    /// Removes the specified node from the list.
    ///
    /// # Safety
    ///
    /// The caller **must** ensure that exactly one of the following is true:
    /// - `node` is currently contained by `self`,
    /// - `node` is not contained by any list,
    /// - `node` is currently contained by some other `GuardedLinkedList`.
    pub(crate) unsafe fn remove(&self, node: NonNull<L::Target>) -> Option<L::Handle> {
        panic!("STUB: not implemented");
    }
    /// Gets the lock of `ShardedList`, makes us have the write permission.
    pub(crate) fn lock_shard(&self, val: &L::Handle) -> ShardGuard<'_, L> {
        panic!("STUB: not implemented");
    }
    /// Gets the count of elements in this list.
    pub(crate) fn len(&self) -> usize {
        panic!("STUB: not implemented");
    }
    cfg_unstable_metrics! {
        cfg_64bit_metrics! { #[doc =
        " Gets the total number of elements added to this list."] pub (crate) fn added(&
        self) -> u64 { self.added.load(Ordering::Relaxed) } }
    }
    /// Returns whether the linked list does not contain any node.
    pub(crate) fn is_empty(&self) -> bool {
        panic!("STUB: not implemented");
    }
    /// Gets the shard size of this `SharedList`.
    ///
    /// Used to help us to decide the parameter `shard_id` of the `pop_back` method.
    pub(crate) fn shard_size(&self) -> usize {
        panic!("STUB: not implemented");
    }
    #[inline]
    fn shard_inner(&self, id: usize) -> MutexGuard<'_, LinkedList<L>> {
        panic!("STUB: not implemented");
    }
}
impl<'a, L: ShardedListItem> ShardGuard<'a, L> {
    /// Push a value to this shard.
    pub(crate) fn push(mut self, val: L::Handle) {
        panic!("STUB: not implemented");
    }
}
cfg_taskdump! {
    impl < L : ShardedListItem > ShardedList < L > { pub (crate) fn for_each < F > (&
    self, mut f : F) where F : FnMut(& L::Handle), { let mut guards =
    Vec::with_capacity(self.lists.len()); for list in self.lists.iter() { guards
    .push(list.lock()); } for g in & mut guards { g.for_each(& mut f); } } }
}
