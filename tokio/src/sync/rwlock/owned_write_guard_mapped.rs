use crate::sync::rwlock::RwLock;
use std::marker::PhantomData;
use std::sync::Arc;
use std::{fmt, mem, ops, ptr};
/// Owned RAII structure used to release the exclusive write access of a lock when
/// dropped.
///
/// This structure is created by [mapping] an [`OwnedRwLockWriteGuard`]. It is a
/// separate type from `OwnedRwLockWriteGuard` to disallow downgrading a mapped
/// guard, since doing so can cause undefined behavior.
///
/// [mapping]: method@crate::sync::OwnedRwLockWriteGuard::map
/// [`OwnedRwLockWriteGuard`]: struct@crate::sync::OwnedRwLockWriteGuard
#[clippy::has_significant_drop]
pub struct OwnedRwLockMappedWriteGuard<T: ?Sized, U: ?Sized = T> {
    #[cfg(all(tokio_unstable, feature = "tracing"))]
    pub(super) resource_span: tracing::Span,
    pub(super) permits_acquired: u32,
    pub(super) lock: Arc<RwLock<T>>,
    pub(super) data: *mut U,
    pub(super) _p: PhantomData<T>,
}
#[allow(dead_code)]
struct Inner<T: ?Sized, U: ?Sized> {
    #[cfg(all(tokio_unstable, feature = "tracing"))]
    resource_span: tracing::Span,
    permits_acquired: u32,
    lock: Arc<RwLock<T>>,
    data: *const U,
}
impl<T: ?Sized, U: ?Sized> OwnedRwLockMappedWriteGuard<T, U> {
    fn skip_drop(self) -> Inner<T, U> {
        panic!("STUB: not implemented");
    }
    /// Makes a new `OwnedRwLockMappedWriteGuard` for a component of the locked
    /// data.
    ///
    /// This operation cannot fail as the `OwnedRwLockMappedWriteGuard` passed
    /// in already locked the data.
    ///
    /// This is an associated function that needs to be used as
    /// `OwnedRwLockWriteGuard::map(..)`. A method would interfere with methods
    /// of the same name on the contents of the locked data.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    /// use tokio::sync::{RwLock, OwnedRwLockWriteGuard};
    ///
    /// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    /// struct Foo(u32);
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let lock = Arc::new(RwLock::new(Foo(1)));
    ///
    /// {
    ///     let lock = Arc::clone(&lock);
    ///     let mut mapped = OwnedRwLockWriteGuard::map(lock.write_owned().await, |f| &mut f.0);
    ///     *mapped = 2;
    /// }
    ///
    /// assert_eq!(Foo(2), *lock.read().await);
    /// # }
    /// ```
    #[inline]
    pub fn map<F, V: ?Sized>(mut this: Self, f: F) -> OwnedRwLockMappedWriteGuard<T, V>
    where
        F: FnOnce(&mut U) -> &mut V,
    {
        panic!("STUB: not implemented");
    }
    /// Attempts to make a new `OwnedRwLockMappedWriteGuard` for a component
    /// of the locked data. The original guard is returned if the closure
    /// returns `None`.
    ///
    /// This operation cannot fail as the `OwnedRwLockMappedWriteGuard` passed
    /// in already locked the data.
    ///
    /// This is an associated function that needs to be
    /// used as `OwnedRwLockMappedWriteGuard::try_map(...)`. A method would interfere with
    /// methods of the same name on the contents of the locked data.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    /// use tokio::sync::{RwLock, OwnedRwLockWriteGuard};
    ///
    /// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    /// struct Foo(u32);
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let lock = Arc::new(RwLock::new(Foo(1)));
    ///
    /// {
    ///     let guard = Arc::clone(&lock).write_owned().await;
    ///     let mut guard = OwnedRwLockWriteGuard::try_map(guard, |f| Some(&mut f.0)).expect("should not fail");
    ///     *guard = 2;
    /// }
    ///
    /// assert_eq!(Foo(2), *lock.read().await);
    /// # }
    /// ```
    #[inline]
    pub fn try_map<F, V: ?Sized>(
        mut this: Self,
        f: F,
    ) -> Result<OwnedRwLockMappedWriteGuard<T, V>, Self>
    where
        F: FnOnce(&mut U) -> Option<&mut V>,
    {
        panic!("STUB: not implemented");
    }
    /// Returns a reference to the original `Arc<RwLock>`.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    /// use tokio::sync::{
    ///     RwLock,
    ///     OwnedRwLockWriteGuard,
    ///     OwnedRwLockMappedWriteGuard,
    /// };
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let lock = Arc::new(RwLock::new(1));
    ///
    /// let guard = lock.clone().write_owned().await;
    /// let guard = OwnedRwLockWriteGuard::map(guard, |x| x);
    /// assert!(Arc::ptr_eq(&lock, OwnedRwLockMappedWriteGuard::rwlock(&guard)));
    /// # }
    /// ```
    pub fn rwlock(this: &Self) -> &Arc<RwLock<T>> {
        panic!("STUB: not implemented");
    }
}
impl<T: ?Sized, U: ?Sized> ops::Deref for OwnedRwLockMappedWriteGuard<T, U> {
    type Target = U;
    fn deref(&self) -> &U {
        panic!("STUB: not implemented");
    }
}
impl<T: ?Sized, U: ?Sized> ops::DerefMut for OwnedRwLockMappedWriteGuard<T, U> {
    fn deref_mut(&mut self) -> &mut U {
        panic!("STUB: not implemented");
    }
}
impl<T: ?Sized, U: ?Sized> fmt::Debug for OwnedRwLockMappedWriteGuard<T, U>
where
    U: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl<T: ?Sized, U: ?Sized> fmt::Display for OwnedRwLockMappedWriteGuard<T, U>
where
    U: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl<T: ?Sized, U: ?Sized> Drop for OwnedRwLockMappedWriteGuard<T, U> {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
