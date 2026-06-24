use std::sync::{self, MutexGuard, TryLockError};
/// Adapter for `std::Mutex` that removes the poisoning aspects
/// from its API.
#[derive(Debug)]
pub(crate) struct Mutex<T: ?Sized>(sync::Mutex<T>);
#[allow(dead_code)]
impl<T> Mutex<T> {
    #[inline]
    pub(crate) fn new(t: T) -> Mutex<T> {
        panic!("STUB: not implemented");
    }
    #[inline]
    pub(crate) const fn const_new(t: T) -> Mutex<T> {
        Mutex(sync::Mutex::new(t))
    }
    #[inline]
    pub(crate) fn lock(&self) -> MutexGuard<'_, T> {
        panic!("STUB: not implemented");
    }
    #[inline]
    pub(crate) fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        panic!("STUB: not implemented");
    }
}
