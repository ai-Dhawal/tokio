use crate::loom::cell::UnsafeCell;
use crate::loom::sync::atomic::{AtomicPtr, AtomicUsize};
use std::alloc::Layout;
use std::mem::MaybeUninit;
use std::ops;
use std::ptr::{self, NonNull};
use std::sync::atomic::Ordering::{self, AcqRel, Acquire, Release};
/// A block in a linked list.
///
/// Each block in the list can hold up to `BLOCK_CAP` messages.
pub(crate) struct Block<T> {
    /// The header fields.
    header: BlockHeader<T>,
    /// Array containing values pushed into the block. Values are stored in a
    /// continuous array in order to improve cache line behavior when reading.
    /// The values must be manually dropped.
    values: Values<T>,
}
/// Extra fields for a `Block<T>`.
struct BlockHeader<T> {
    /// The start index of this block.
    ///
    /// Slots in this block have indices in `start_index .. start_index + BLOCK_CAP`.
    start_index: usize,
    /// The next block in the linked list.
    next: AtomicPtr<Block<T>>,
    /// Bitfield tracking slots that are ready to have their values consumed.
    ready_slots: AtomicUsize,
    /// The observed `tail_position` value *after* the block has been passed by
    /// `block_tail`.
    observed_tail_position: UnsafeCell<usize>,
}
pub(crate) enum Read<T> {
    Value(T),
    Closed,
}
#[repr(transparent)]
struct Values<T>([UnsafeCell<MaybeUninit<T>>; BLOCK_CAP]);
use super::BLOCK_CAP;
/// Masks an index to get the block identifier.
const BLOCK_MASK: usize = !(BLOCK_CAP - 1);
/// Masks an index to get the value offset in a block.
const SLOT_MASK: usize = BLOCK_CAP - 1;
/// Flag tracking that a block has gone through the sender's release routine.
///
/// When this is set, the receiver may consider freeing the block.
const RELEASED: usize = 1 << BLOCK_CAP;
/// Flag tracking all senders dropped.
///
/// When this flag is set, the send half of the channel has closed.
const TX_CLOSED: usize = RELEASED << 1;
/// Mask covering all bits used to track slot readiness.
const READY_MASK: usize = RELEASED - 1;
/// Returns the index of the first slot in the block referenced by `slot_index`.
#[inline(always)]
pub(crate) fn start_index(slot_index: usize) -> usize {
    panic!("STUB: not implemented");
}
/// Returns the offset into the block referenced by `slot_index`.
#[inline(always)]
pub(crate) fn offset(slot_index: usize) -> usize {
    panic!("STUB: not implemented");
}
generate_addr_of_methods! {
    impl < T > Block < T > { unsafe fn addr_of_header(self : NonNull < Self >) -> NonNull
    < BlockHeader < T >> { & self.header } unsafe fn addr_of_values(self : NonNull < Self
    >) -> NonNull < Values < T >> { & self.values } }
}
impl<T> Block<T> {
    pub(crate) fn new(start_index: usize) -> Box<Block<T>> {
        panic!("STUB: not implemented");
    }
    /// Returns `true` if the block matches the given index.
    pub(crate) fn is_at_index(&self, index: usize) -> bool {
        panic!("STUB: not implemented");
    }
    /// Returns the number of blocks between `self` and the block at the
    /// specified index.
    ///
    /// `start_index` must represent a block *after* `self`.
    pub(crate) fn distance(&self, other_index: usize) -> usize {
        panic!("STUB: not implemented");
    }
    /// Reads the value at the given offset.
    ///
    /// Returns `None` if the slot is empty.
    ///
    /// # Safety
    ///
    /// To maintain safety, the caller must ensure:
    ///
    /// * No concurrent access to the slot.
    pub(crate) unsafe fn read(&self, slot_index: usize) -> Option<Read<T>> {
        panic!("STUB: not implemented");
    }
    /// Returns true if *this* block has a value in the given slot.
    ///
    /// Always returns false when given an index from a different block.
    pub(crate) fn has_value(&self, slot_index: usize) -> bool {
        panic!("STUB: not implemented");
    }
    /// Writes a value to the block at the given offset.
    ///
    /// # Safety
    ///
    /// To maintain safety, the caller must ensure:
    ///
    /// * The slot is empty.
    /// * No concurrent access to the slot.
    pub(crate) unsafe fn write(&self, slot_index: usize, value: T) {
        panic!("STUB: not implemented");
    }
    /// Signal to the receiver that the sender half of the list is closed.
    pub(crate) unsafe fn tx_close(&self) {
        panic!("STUB: not implemented");
    }
    /// Resets the block to a blank state. This enables reusing blocks in the
    /// channel.
    ///
    /// # Safety
    ///
    /// To maintain safety, the caller must ensure:
    ///
    /// * All slots are empty.
    /// * The caller holds a unique pointer to the block.
    pub(crate) unsafe fn reclaim(&mut self) {
        panic!("STUB: not implemented");
    }
    /// Releases the block to the rx half for freeing.
    ///
    /// This function is called by the tx half once it can be guaranteed that no
    /// more senders will attempt to access the block.
    ///
    /// # Safety
    ///
    /// To maintain safety, the caller must ensure:
    ///
    /// * The block will no longer be accessed by any sender.
    pub(crate) unsafe fn tx_release(&self, tail_position: usize) {
        panic!("STUB: not implemented");
    }
    /// Mark a slot as ready
    fn set_ready(&self, slot: usize) {
        panic!("STUB: not implemented");
    }
    /// Returns `true` when all slots have their `ready` bits set.
    ///
    /// This indicates that the block is in its final state and will no longer
    /// be mutated.
    pub(crate) fn is_final(&self) -> bool {
        panic!("STUB: not implemented");
    }
    /// Returns the `observed_tail_position` value, if set
    pub(crate) fn observed_tail_position(&self) -> Option<usize> {
        panic!("STUB: not implemented");
    }
    /// Loads the next block
    pub(crate) fn load_next(&self, ordering: Ordering) -> Option<NonNull<Block<T>>> {
        panic!("STUB: not implemented");
    }
    /// Pushes `block` as the next block in the link.
    ///
    /// Returns Ok if successful, otherwise, a pointer to the next block in
    /// the list is returned.
    ///
    /// This requires that the next pointer is null.
    ///
    /// # Ordering
    ///
    /// This performs a compare-and-swap on `next` using `AcqRel` ordering.
    ///
    /// # Safety
    ///
    /// To maintain safety, the caller must ensure:
    ///
    /// * `block` is not freed until it has been removed from the list.
    pub(crate) unsafe fn try_push(
        &self,
        block: &mut NonNull<Block<T>>,
        success: Ordering,
        failure: Ordering,
    ) -> Result<(), NonNull<Block<T>>> {
        panic!("STUB: not implemented");
    }
    /// Grows the `Block` linked list by allocating and appending a new block.
    ///
    /// The next block in the linked list is returned. This may or may not be
    /// the one allocated by the function call.
    ///
    /// # Implementation
    ///
    /// It is assumed that `self.next` is null. A new block is allocated with
    /// `start_index` set to be the next block. A compare-and-swap is performed
    /// with `AcqRel` memory ordering. If the compare-and-swap is successful, the
    /// newly allocated block is released to other threads walking the block
    /// linked list. If the compare-and-swap fails, the current thread acquires
    /// the next block in the linked list, allowing the current thread to access
    /// the slots.
    pub(crate) fn grow(&self) -> NonNull<Block<T>> {
        panic!("STUB: not implemented");
    }
}
/// Returns `true` if the specified slot has a value ready to be consumed.
fn is_ready(bits: usize, slot: usize) -> bool {
    panic!("STUB: not implemented");
}
/// Returns `true` if the closed flag has been set.
fn is_tx_closed(bits: usize) -> bool {
    panic!("STUB: not implemented");
}
impl<T> Values<T> {
    /// Initialize a `Values` struct from a pointer.
    ///
    /// # Safety
    ///
    /// The raw pointer must be valid for writing a `Values<T>`.
    unsafe fn initialize(_value: NonNull<Values<T>>) {
        panic!("STUB: not implemented");
    }
}
impl<T> ops::Index<usize> for Values<T> {
    type Output = UnsafeCell<MaybeUninit<T>>;
    fn index(&self, index: usize) -> &Self::Output {
        panic!("STUB: not implemented");
    }
}
#[cfg(all(test, not(loom)))]
#[test]
fn assert_no_stack_overflow() {
    struct Foo {
        _a: [u8; 2_000_000],
    }
    assert_eq!(
        Layout::new::< MaybeUninit < Block < Foo >>> (), Layout::new::< Block < Foo >> ()
    );
    let _block = Block::<Foo>::new(0);
}
