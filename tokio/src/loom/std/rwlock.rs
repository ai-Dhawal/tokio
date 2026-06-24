use std::sync::{self, RwLockReadGuard, RwLockWriteGuard, TryLockError};
/// Adapter for `std::sync::RwLock` that removes the poisoning aspects
/// from its api.
#[derive(Debug)]
pub(crate) struct RwLock<T: ?Sized>(sync::RwLock<T>);
#[allow(dead_code)]
impl<T> RwLock<T> {
    #[inline]
    pub(crate) fn new(t: T) -> Self {
        panic!("STUB: not implemented");
    }
    #[inline]
    pub(crate) fn read(&self) -> RwLockReadGuard<'_, T> {
        panic!("STUB: not implemented");
    }
    #[inline]
    pub(crate) fn try_read(&self) -> Option<RwLockReadGuard<'_, T>> {
        panic!("STUB: not implemented");
    }
    #[inline]
    pub(crate) fn write(&self) -> RwLockWriteGuard<'_, T> {
        panic!("STUB: not implemented");
    }
    #[inline]
    pub(crate) fn try_write(&self) -> Option<RwLockWriteGuard<'_, T>> {
        panic!("STUB: not implemented");
    }
}
