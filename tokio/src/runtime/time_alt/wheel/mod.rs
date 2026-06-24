mod level;
pub(crate) use self::level::Expiration;
use self::level::Level;
use super::cancellation_queue::Sender;
use super::{Entry, EntryHandle, WakeQueue};
use crate::util::linked_list::LinkedList;
/// Hashed timing wheel implementation.
///
/// See [`Driver`] documentation for some implementation notes.
///
/// [`Driver`]: crate::runtime::time::Driver
#[derive(Debug)]
pub(crate) struct Wheel {
    /// The number of milliseconds elapsed since the wheel started.
    elapsed: u64,
    /// Timer wheel.
    ///
    /// Levels:
    ///
    /// * 1 ms slots / 64 ms range
    /// * 64 ms slots / ~ 4 sec range
    /// * ~ 4 sec slots / ~ 4 min range
    /// * ~ 4 min slots / ~ 4 hr range
    /// * ~ 4 hr slots / ~ 12 day range
    /// * ~ 12 day slots / ~ 2 yr range
    levels: Box<[Level; NUM_LEVELS]>,
}
/// Number of levels. Each level has 64 slots. By using 6 levels with 64 slots
/// each, the timer is able to track time up to 2 years into the future with a
/// precision of 1 millisecond.
const NUM_LEVELS: usize = 6;
/// The maximum duration of a `Sleep`.
pub(super) const MAX_DURATION: u64 = (1 << (6 * NUM_LEVELS)) - 1;
impl Wheel {
    /// Creates a new timing wheel.
    pub(crate) fn new() -> Wheel {
        panic!("STUB: not implemented");
    }
    /// Returns the number of milliseconds that have elapsed since the timing
    /// wheel's creation.
    pub(crate) fn elapsed(&self) -> u64 {
        panic!("STUB: not implemented");
    }
    /// Inserts an entry into the timing wheel.
    ///
    /// # Arguments
    ///
    /// * `hdl`: The entry handle to insert into the wheel.
    ///
    /// # Safety
    ///
    /// The caller must ensure:
    ///
    /// * The entry is not already registered in ANY wheel.
    pub(crate) unsafe fn insert(&mut self, hdl: EntryHandle, cancel_tx: Sender) {
        panic!("STUB: not implemented");
    }
    /// Removes `item` from the timing wheel.
    ///
    /// # Safety
    ///
    /// The caller must ensure:
    ///
    /// * The entry is already registered in THIS wheel.
    pub(crate) unsafe fn remove(&mut self, hdl: EntryHandle) {
        panic!("STUB: not implemented");
    }
    /// Advances the timer up to the instant represented by `now`.
    pub(crate) fn take_expired(&mut self, now: u64, wake_queue: &mut WakeQueue) {
        panic!("STUB: not implemented");
    }
    /// Returns the instant at which the next timeout expires.
    fn next_expiration(&self) -> Option<Expiration> {
        panic!("STUB: not implemented");
    }
    /// Returns the tick at which this timer wheel next needs to perform some
    /// processing, or None if there are no timers registered.
    pub(crate) fn next_expiration_time(&self) -> Option<u64> {
        panic!("STUB: not implemented");
    }
    /// Used for debug assertions
    fn no_expirations_before(&self, start_level: usize, before: u64) -> bool {
        panic!("STUB: not implemented");
    }
    /// iteratively find entries that are between the wheel's current
    /// time and the expiration time.  for each in that population either
    /// queue it for notification (in the case of the last level) or tier
    /// it down to the next level (in all other cases).
    pub(crate) fn process_expiration(
        &mut self,
        expiration: &Expiration,
        wake_queue: &mut WakeQueue,
    ) {
        panic!("STUB: not implemented");
    }
    fn set_elapsed(&mut self, when: u64) {
        panic!("STUB: not implemented");
    }
    /// Obtains the list of entries that need processing for the given expiration.
    fn take_entries(&mut self, expiration: &Expiration) -> LinkedList<Entry> {
        panic!("STUB: not implemented");
    }
    fn level_for(&self, when: u64) -> usize {
        panic!("STUB: not implemented");
    }
}
fn level_for(elapsed: u64, when: u64) -> usize {
    panic!("STUB: not implemented");
}
#[cfg(all(test, not(loom)))]
mod test {
    use super::*;
    #[test]
    fn test_level_for() {
        for pos in 0..64 {
            assert_eq!(0, level_for(0, pos), "level_for({pos}) -- binary = {pos:b}");
        }
        for level in 1..5 {
            for pos in level..64 {
                let a = pos * 64_usize.pow(level as u32);
                assert_eq!(
                    level, level_for(0, a as u64), "level_for({a}) -- binary = {a:b}"
                );
                if pos > level {
                    let a = a - 1;
                    assert_eq!(
                        level, level_for(0, a as u64), "level_for({a}) -- binary = {a:b}"
                    );
                }
                if pos < 64 {
                    let a = a + 1;
                    assert_eq!(
                        level, level_for(0, a as u64), "level_for({a}) -- binary = {a:b}"
                    );
                }
            }
        }
    }
}
