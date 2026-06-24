//! A concurrent, lock-free, FIFO list.
use crate::loom::sync::atomic::{AtomicPtr, AtomicUsize};
use crate::loom::thread;
use crate::sync::mpsc::block::{self, Block};
use std::fmt;
use std::ptr::NonNull;
use std::sync::atomic::Ordering::{AcqRel, Acquire, Relaxed, Release};
/// List queue transmit handle.
pub(crate) struct Tx<T> {
    /// Tail in the `Block` mpmc list.
    block_tail: AtomicPtr<Block<T>>,
    /// Position to push the next message. This references a block and offset
    /// into the block.
    tail_position: AtomicUsize,
}
/// List queue receive handle
pub(crate) struct Rx<T> {
    /// Pointer to the block being processed.
    head: NonNull<Block<T>>,
    /// Next slot index to process.
    index: usize,
    /// Pointer to the next block pending release.
    free_head: NonNull<Block<T>>,
}
/// Return value of `Rx::try_pop`.
pub(crate) enum TryPopResult<T> {
    /// Successfully popped a value.
    Ok(T),
    /// The channel is empty.
    ///
    /// Note that `list.rs` only tracks the close state set by senders. If the
    /// channel is closed by `Rx::close()`, then `TryPopResult::Empty` is still
    /// returned, and the close state needs to be handled by `chan.rs`.
    Empty,
    /// The channel is empty and closed.
    ///
    /// Returned when the send half is closed (all senders dropped).
    Closed,
    /// The channel is not empty, but the first value is being written.
    Busy,
}
pub(crate) fn channel<T>() -> (Tx<T>, Rx<T>) {
    panic!("STUB: not implemented");
}
impl<T> Tx<T> {
    /// Pushes a value into the list.
    pub(crate) fn push(&self, value: T) {
        panic!("STUB: not implemented");
    }
    /// Closes the send half of the list.
    ///
    /// Similar process as pushing a value, but instead of writing the value &
    /// setting the ready flag, the `TX_CLOSED` flag is set on the block.
    pub(crate) fn close(&self) {
        panic!("STUB: not implemented");
    }
    fn find_block(&self, slot_index: usize) -> NonNull<Block<T>> {
        panic!("STUB: not implemented");
    }
    /// # Safety
    ///
    /// Behavior is undefined if any of the following conditions are violated:
    ///
    /// - The `block` was created by [`Box::into_raw`].
    /// - The `block` is not currently part of any linked list.
    /// - The `block` is a valid pointer to a [`Block<T>`].
    pub(crate) unsafe fn reclaim_block(&self, mut block: NonNull<Block<T>>) {
        panic!("STUB: not implemented");
    }
}
impl<T> fmt::Debug for Tx<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl<T> Rx<T> {
    pub(crate) fn is_empty(&self, tx: &Tx<T>) -> bool {
        panic!("STUB: not implemented");
    }
    fn is_maybe_closed(&self, tx: &Tx<T>, slot_index: usize) -> bool {
        panic!("STUB: not implemented");
    }
    pub(crate) fn len(&self, tx: &Tx<T>) -> usize {
        panic!("STUB: not implemented");
    }
    /// Pops the next value off the queue.
    pub(crate) fn pop(&mut self, tx: &Tx<T>) -> Option<block::Read<T>> {
        panic!("STUB: not implemented");
    }
    /// Pops the next value off the queue, detecting whether the block
    /// is busy or empty on failure.
    ///
    /// This function exists because `Rx::pop` can return `None` even if the
    /// channel's queue contains a message that has been completely written.
    /// This can happen if the fully delivered message is behind another message
    /// that is in the middle of being written to the block, since the channel
    /// can't return the messages out of order.
    pub(crate) fn try_pop(&mut self, tx: &Tx<T>) -> TryPopResult<T> {
        panic!("STUB: not implemented");
    }
    /// Tries advancing the block pointer to the block referenced by `self.index`.
    ///
    /// Returns `true` if successful, `false` if there is no next block to load.
    fn try_advancing_head(&mut self) -> bool {
        panic!("STUB: not implemented");
    }
    fn reclaim_blocks(&mut self, tx: &Tx<T>) {
        panic!("STUB: not implemented");
    }
    /// Effectively `Drop` all the blocks. Should only be called once, when
    /// the list is dropping.
    pub(super) unsafe fn free_blocks(&mut self) {
        panic!("STUB: not implemented");
    }
}
impl<T> fmt::Debug for Rx<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
