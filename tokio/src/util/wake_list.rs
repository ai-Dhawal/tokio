use core::mem::MaybeUninit;
use core::ptr;
use std::task::Waker;
const NUM_WAKERS: usize = 32;
/// A list of wakers to be woken.
///
/// # Invariants
///
/// The first `curr` elements of `inner` are initialized.
pub(crate) struct WakeList {
    inner: [MaybeUninit<Waker>; NUM_WAKERS],
    curr: usize,
}
impl WakeList {
    pub(crate) fn new() -> Self {
        panic!("STUB: not implemented");
    }
    #[inline]
    pub(crate) fn can_push(&self) -> bool {
        panic!("STUB: not implemented");
    }
    pub(crate) fn push(&mut self, val: Waker) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn wake_all(&mut self) {
        panic!("STUB: not implemented");
    }
}
impl Drop for WakeList {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
