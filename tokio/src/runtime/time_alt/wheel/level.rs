use crate::util::linked_list::LinkedList;
use super::{Entry, EntryHandle};
use std::ptr::NonNull;
use std::{array, fmt};
/// Wheel for a single level in the timer. This wheel contains 64 slots.
pub(crate) struct Level {
    level: usize,
    /// Bit field tracking which slots currently contain entries.
    ///
    /// Using a bit field to track slots that contain entries allows avoiding a
    /// scan to find entries. This field is updated when entries are added or
    /// removed from a slot.
    ///
    /// The least-significant bit represents slot zero.
    occupied: u64,
    /// Slots. We access these via the EntryInner `current_list` as well, so this needs to be an `UnsafeCell`.
    slot: [LinkedList<Entry>; LEVEL_MULT],
}
/// Indicates when a slot must be processed next.
#[derive(Debug)]
pub(crate) struct Expiration {
    /// The level containing the slot.
    pub(crate) level: usize,
    /// The slot index.
    pub(crate) slot: usize,
    /// The instant at which the slot needs to be processed.
    pub(crate) deadline: u64,
}
/// Level multiplier.
///
/// Being a power of 2 is very important.
const LEVEL_MULT: usize = 64;
impl Level {
    pub(crate) fn new(level: usize) -> Level {
        panic!("STUB: not implemented");
    }
    /// Finds the slot that needs to be processed next and returns the slot and
    /// `Instant` at which this slot must be processed.
    pub(crate) fn next_expiration(&self, now: u64) -> Option<Expiration> {
        panic!("STUB: not implemented");
    }
    fn next_occupied_slot(&self, now: u64) -> Option<usize> {
        panic!("STUB: not implemented");
    }
    pub(crate) unsafe fn add_entry(&mut self, hdl: EntryHandle) {
        panic!("STUB: not implemented");
    }
    pub(crate) unsafe fn remove_entry(&mut self, hdl: EntryHandle) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn take_slot(&mut self, slot: usize) -> LinkedList<Entry> {
        panic!("STUB: not implemented");
    }
}
impl fmt::Debug for Level {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
fn occupied_bit(slot: usize) -> u64 {
    panic!("STUB: not implemented");
}
fn slot_range(level: usize) -> u64 {
    panic!("STUB: not implemented");
}
fn level_range(level: usize) -> u64 {
    panic!("STUB: not implemented");
}
/// Converts a duration (milliseconds) and a level to a slot position.
fn slot_for(duration: u64, level: usize) -> usize {
    panic!("STUB: not implemented");
}
#[cfg(all(test, not(loom)))]
mod test {
    use super::*;
    #[test]
    fn test_slot_for() {
        for pos in 0..64 {
            assert_eq!(pos as usize, slot_for(pos, 0));
        }
        for level in 1..5 {
            for pos in level..64 {
                let a = pos * 64_usize.pow(level as u32);
                assert_eq!(pos, slot_for(a as u64, level));
            }
        }
    }
}
