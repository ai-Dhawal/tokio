use super::Notify;
use crate::loom::cell::UnsafeCell;
use crate::loom::sync::atomic::AtomicBool;
use std::error::Error;
use std::fmt;
use std::future::{poll_fn, Future};
use std::mem::MaybeUninit;
use std::ops::Drop;
use std::ptr;
use std::sync::atomic::Ordering;
use std::task::Poll;
/// A thread-safe cell that can be written to only once.
///
/// A `SetOnce` is inspired from python's [`asyncio.Event`] type. It can be
/// used to wait until the value of the `SetOnce` is set like a "Event" mechanism.
///
/// # Example
///
/// ```
/// use tokio::sync::{SetOnce, SetOnceError};
///
/// static ONCE: SetOnce<u32> = SetOnce::const_new();
///
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() -> Result<(), SetOnceError<u32>> {
///
/// // set the value inside a task somewhere...
/// tokio::spawn(async move { ONCE.set(20) });
///
/// // checking with .get doesn't block main thread
/// println!("{:?}", ONCE.get());
///
/// // wait until the value is set, blocks the thread
/// println!("{:?}", ONCE.wait().await);
///
/// Ok(())
/// # }
/// ```
///
/// A `SetOnce` is typically used for global variables that need to be
/// initialized once on first use, but need no further changes. The `SetOnce`
/// in Tokio allows the initialization procedure to be asynchronous.
///
/// # Example
///
/// ```
/// use tokio::sync::{SetOnce, SetOnceError};
/// use std::sync::Arc;
///
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() -> Result<(), SetOnceError<u32>> {
/// let once = SetOnce::new();
///
/// let arc = Arc::new(once);
/// let first_cl = Arc::clone(&arc);
/// let second_cl = Arc::clone(&arc);
///
/// // set the value inside a task
/// tokio::spawn(async move { first_cl.set(20) }).await.unwrap()?;
///
/// // wait inside task to not block the main thread
/// tokio::spawn(async move {
///     // wait inside async context for the value to be set
///     assert_eq!(*second_cl.wait().await, 20);
/// }).await.unwrap();
///
/// // subsequent set calls will fail
/// assert!(arc.set(30).is_err());
///
/// println!("{:?}", arc.get());
///
/// Ok(())
/// # }
/// ```
///
/// [`asyncio.Event`]: https://docs.python.org/3/library/asyncio-sync.html#asyncio.Event
pub struct SetOnce<T> {
    value_set: AtomicBool,
    value: UnsafeCell<MaybeUninit<T>>,
    notify: Notify,
}
impl<T> Default for SetOnce<T> {
    fn default() -> SetOnce<T> {
        panic!("STUB: not implemented");
    }
}
impl<T: fmt::Debug> fmt::Debug for SetOnce<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl<T: Clone> Clone for SetOnce<T> {
    fn clone(&self) -> SetOnce<T> {
        panic!("STUB: not implemented");
    }
}
impl<T: PartialEq> PartialEq for SetOnce<T> {
    fn eq(&self, other: &SetOnce<T>) -> bool {
        panic!("STUB: not implemented");
    }
}
impl<T: Eq> Eq for SetOnce<T> {}
impl<T> Drop for SetOnce<T> {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
impl<T> From<T> for SetOnce<T> {
    fn from(value: T) -> Self {
        panic!("STUB: not implemented");
    }
}
impl<T> SetOnce<T> {
    /// Creates a new empty `SetOnce` instance.
    pub fn new() -> Self {
        panic!("STUB: not implemented");
    }
    /// Creates a new empty `SetOnce` instance.
    ///
    /// Equivalent to `SetOnce::new`, except that it can be used in static
    /// variables.
    ///
    /// When using the `tracing` [unstable feature], a `SetOnce` created with
    /// `const_new` will not be instrumented. As such, it will not be visible
    /// in [`tokio-console`]. Instead, [`SetOnce::new`] should be used to
    /// create an instrumented object if that is needed.
    ///
    /// # Example
    ///
    /// ```
    /// use tokio::sync::{SetOnce, SetOnceError};
    ///
    /// static ONCE: SetOnce<u32> = SetOnce::const_new();
    ///
    /// fn get_global_integer() -> Result<Option<&'static u32>, SetOnceError<u32>> {
    ///     ONCE.set(2)?;
    ///     Ok(ONCE.get())
    /// }
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() -> Result<(), SetOnceError<u32>> {
    /// let result = get_global_integer()?;
    ///
    /// assert_eq!(result, Some(&2));
    /// Ok(())
    /// # }
    /// ```
    ///
    /// [`tokio-console`]: https://github.com/tokio-rs/console
    /// [unstable feature]: crate#unstable-features
    #[cfg(not(all(loom, test)))]
    pub const fn const_new() -> Self {
        Self {
            value_set: AtomicBool::new(false),
            value: UnsafeCell::new(MaybeUninit::uninit()),
            notify: Notify::const_new(),
        }
    }
    /// Creates a new `SetOnce` that contains the provided value, if any.
    ///
    /// If the `Option` is `None`, this is equivalent to `SetOnce::new`.
    ///
    /// [`SetOnce::new`]: crate::sync::SetOnce::new
    pub fn new_with(value: Option<T>) -> Self {
        panic!("STUB: not implemented");
    }
    /// Creates a new `SetOnce` that contains the provided value.
    ///
    /// # Example
    ///
    /// When using the `tracing` [unstable feature], a `SetOnce` created with
    /// `const_new_with` will not be instrumented. As such, it will not be
    /// visible in [`tokio-console`]. Instead, [`SetOnce::new_with`] should be
    /// used to create an instrumented object if that is needed.
    ///
    /// ```
    /// use tokio::sync::SetOnce;
    ///
    /// static ONCE: SetOnce<u32> = SetOnce::const_new_with(1);
    ///
    /// fn get_global_integer() -> Option<&'static u32> {
    ///     ONCE.get()
    /// }
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let result = get_global_integer();
    ///
    /// assert_eq!(result, Some(&1));
    /// # }
    /// ```
    ///
    /// [`tokio-console`]: https://github.com/tokio-rs/console
    /// [unstable feature]: crate#unstable-features
    #[cfg(not(all(loom, test)))]
    pub const fn const_new_with(value: T) -> Self {
        Self {
            value_set: AtomicBool::new(true),
            value: UnsafeCell::new(MaybeUninit::new(value)),
            notify: Notify::const_new(),
        }
    }
    /// Returns `true` if the `SetOnce` currently contains a value, and `false`
    /// otherwise.
    pub fn initialized(&self) -> bool {
        panic!("STUB: not implemented");
    }
    unsafe fn get_unchecked(&self) -> &T {
        panic!("STUB: not implemented");
    }
    /// Returns a reference to the value currently stored in the `SetOnce`, or
    /// `None` if the `SetOnce` is empty.
    pub fn get(&self) -> Option<&T> {
        panic!("STUB: not implemented");
    }
    /// Sets the value of the `SetOnce` to the given value if the `SetOnce` is
    /// empty.
    ///
    /// If the `SetOnce` already has a value, this call will fail with an
    /// [`SetOnceError`].
    ///
    /// [`SetOnceError`]: crate::sync::SetOnceError
    pub fn set(&self, value: T) -> Result<(), SetOnceError<T>> {
        panic!("STUB: not implemented");
    }
    /// Takes the value from the cell, destroying the cell in the process.
    /// Returns `None` if the cell is empty.
    pub fn into_inner(self) -> Option<T> {
        panic!("STUB: not implemented");
    }
    /// Waits until the value is set.
    ///
    /// If the `SetOnce` is already initialized, it will return the value
    /// immediately.
    ///
    /// # Cancel safety
    ///
    /// This method is cancel safe.
    pub async fn wait(&self) -> &T {
        panic!("STUB: not implemented");
    }
}
unsafe impl<T: Sync + Send> Sync for SetOnce<T> {}
unsafe impl<T: Send> Send for SetOnce<T> {}
/// Error that can be returned from [`SetOnce::set`].
///
/// This error means that the `SetOnce` was already initialized when
/// set was called
///
/// [`SetOnce::set`]: crate::sync::SetOnce::set
#[derive(Debug, PartialEq, Eq)]
pub struct SetOnceError<T>(pub T);
impl<T> fmt::Display for SetOnceError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl<T: fmt::Debug> Error for SetOnceError<T> {}
