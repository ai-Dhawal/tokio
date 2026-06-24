use crate::loom::sync::atomic::AtomicBool;
use std::cell::UnsafeCell;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::Ordering::SeqCst;
pub(crate) struct TryLock<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}
pub(crate) struct LockGuard<'a, T> {
    lock: &'a TryLock<T>,
    _p: PhantomData<std::rc::Rc<()>>,
}
unsafe impl<T: Send> Send for TryLock<T> {}
unsafe impl<T: Send> Sync for TryLock<T> {}
unsafe impl<T: Sync> Sync for LockGuard<'_, T> {}
macro_rules! new {
    ($data:ident) => {
        TryLock { locked : AtomicBool::new(false), data : UnsafeCell::new($data), }
    };
}
impl<T> TryLock<T> {
    #[cfg(not(loom))]
    /// Create a new `TryLock`
    pub(crate) const fn new(data: T) -> TryLock<T> {
        new!(data)
    }
    #[cfg(loom)]
    /// Create a new `TryLock`
    pub(crate) fn new(data: T) -> TryLock<T> {
        panic!("STUB: not implemented");
    }
    /// Attempt to acquire lock
    pub(crate) fn try_lock(&self) -> Option<LockGuard<'_, T>> {
        panic!("STUB: not implemented");
    }
}
impl<T> Deref for LockGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        panic!("STUB: not implemented");
    }
}
impl<T> DerefMut for LockGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        panic!("STUB: not implemented");
    }
}
impl<T> Drop for LockGuard<'_, T> {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
