//! A minimal adaption of the `parking_lot` synchronization primitives to the
//! equivalent `std::sync` types.
//!
//! This can be extended to additional types/methods as required.
use std::fmt;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::sync::{LockResult, TryLockError};
use std::time::Duration;
pub(crate) use parking_lot::WaitTimeoutResult;
#[derive(Debug)]
pub(crate) struct Mutex<T: ?Sized>(
    PhantomData<std::sync::Mutex<T>>,
    parking_lot::Mutex<T>,
);
#[derive(Debug)]
pub(crate) struct RwLock<T>(PhantomData<std::sync::RwLock<T>>, parking_lot::RwLock<T>);
#[derive(Debug)]
pub(crate) struct Condvar(PhantomData<std::sync::Condvar>, parking_lot::Condvar);
#[derive(Debug)]
pub(crate) struct MutexGuard<'a, T: ?Sized>(
    PhantomData<std::sync::MutexGuard<'a, T>>,
    parking_lot::MutexGuard<'a, T>,
);
#[derive(Debug)]
pub(crate) struct RwLockReadGuard<'a, T: ?Sized>(
    PhantomData<std::sync::RwLockReadGuard<'a, T>>,
    parking_lot::RwLockReadGuard<'a, T>,
);
#[derive(Debug)]
pub(crate) struct RwLockWriteGuard<'a, T: ?Sized>(
    PhantomData<std::sync::RwLockWriteGuard<'a, T>>,
    parking_lot::RwLockWriteGuard<'a, T>,
);
impl<T> Mutex<T> {
    #[inline]
    pub(crate) fn new(t: T) -> Mutex<T> {
        panic!("STUB: not implemented");
    }
    #[inline]
    #[cfg(not(all(loom, test)))]
    pub(crate) const fn const_new(t: T) -> Mutex<T> {
        Mutex(PhantomData, parking_lot::const_mutex(t))
    }
    #[inline]
    pub(crate) fn lock(&self) -> MutexGuard<'_, T> {
        panic!("STUB: not implemented");
    }
    #[inline]
    pub(crate) fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        panic!("STUB: not implemented");
    }
    #[inline]
    pub(crate) fn get_mut(&mut self) -> &mut T {
        panic!("STUB: not implemented");
    }
}
impl<'a, T: ?Sized> Deref for MutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        panic!("STUB: not implemented");
    }
}
impl<'a, T: ?Sized> DerefMut for MutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        panic!("STUB: not implemented");
    }
}
impl<T> RwLock<T> {
    pub(crate) fn new(t: T) -> RwLock<T> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn read(&self) -> RwLockReadGuard<'_, T> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn try_read(&self) -> Option<RwLockReadGuard<'_, T>> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn write(&self) -> RwLockWriteGuard<'_, T> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn try_write(&self) -> Option<RwLockWriteGuard<'_, T>> {
        panic!("STUB: not implemented");
    }
}
impl<'a, T: ?Sized> Deref for RwLockReadGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        panic!("STUB: not implemented");
    }
}
impl<'a, T: ?Sized> Deref for RwLockWriteGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        panic!("STUB: not implemented");
    }
}
impl<'a, T: ?Sized> DerefMut for RwLockWriteGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        panic!("STUB: not implemented");
    }
}
impl Condvar {
    #[inline]
    pub(crate) fn new() -> Condvar {
        panic!("STUB: not implemented");
    }
    #[inline]
    pub(crate) fn notify_one(&self) {
        panic!("STUB: not implemented");
    }
    #[inline]
    pub(crate) fn notify_all(&self) {
        panic!("STUB: not implemented");
    }
    #[inline]
    pub(crate) fn wait<'a, T>(
        &self,
        mut guard: MutexGuard<'a, T>,
    ) -> LockResult<MutexGuard<'a, T>> {
        panic!("STUB: not implemented");
    }
    #[inline]
    pub(crate) fn wait_timeout<'a, T>(
        &self,
        mut guard: MutexGuard<'a, T>,
        timeout: Duration,
    ) -> LockResult<(MutexGuard<'a, T>, WaitTimeoutResult)> {
        panic!("STUB: not implemented");
    }
}
impl<'a, T: ?Sized + fmt::Display> fmt::Display for MutexGuard<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl<'a, T: ?Sized + fmt::Display> fmt::Display for RwLockReadGuard<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl<'a, T: ?Sized + fmt::Display> fmt::Display for RwLockWriteGuard<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
