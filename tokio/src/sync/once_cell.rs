use super::{Semaphore, SemaphorePermit, TryAcquireError};
use crate::loom::cell::UnsafeCell;
use std::error::Error;
use std::fmt;
use std::future::Future;
use std::mem::MaybeUninit;
use std::ops::Drop;
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
/// A thread-safe cell that can be written to only once.
///
/// A `OnceCell` is typically used for global variables that need to be
/// initialized once on first use, but need no further changes. The `OnceCell`
/// in Tokio allows the initialization procedure to be asynchronous.
///
/// # Examples
///
/// ```
/// use tokio::sync::OnceCell;
///
/// async fn some_computation() -> u32 {
///     1 + 1
/// }
///
/// static ONCE: OnceCell<u32> = OnceCell::const_new();
///
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() {
/// let result = ONCE.get_or_init(some_computation).await;
/// assert_eq!(*result, 2);
/// # }
/// ```
///
/// It is often useful to write a wrapper method for accessing the value.
///
/// ```
/// use tokio::sync::OnceCell;
///
/// static ONCE: OnceCell<u32> = OnceCell::const_new();
///
/// async fn get_global_integer() -> &'static u32 {
///     ONCE.get_or_init(|| async {
///         1 + 1
///     }).await
/// }
///
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() {
/// let result = get_global_integer().await;
/// assert_eq!(*result, 2);
/// # }
/// ```
pub struct OnceCell<T> {
    value_set: AtomicBool,
    value: UnsafeCell<MaybeUninit<T>>,
    semaphore: Semaphore,
}
impl<T> Default for OnceCell<T> {
    fn default() -> OnceCell<T> {
        panic!("STUB: not implemented");
    }
}
impl<T: fmt::Debug> fmt::Debug for OnceCell<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl<T: Clone> Clone for OnceCell<T> {
    fn clone(&self) -> OnceCell<T> {
        panic!("STUB: not implemented");
    }
}
impl<T: PartialEq> PartialEq for OnceCell<T> {
    fn eq(&self, other: &OnceCell<T>) -> bool {
        panic!("STUB: not implemented");
    }
}
impl<T: Eq> Eq for OnceCell<T> {}
impl<T> Drop for OnceCell<T> {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
impl<T> From<T> for OnceCell<T> {
    fn from(value: T) -> Self {
        panic!("STUB: not implemented");
    }
}
impl<T> OnceCell<T> {
    /// Creates a new empty `OnceCell` instance.
    pub fn new() -> Self {
        panic!("STUB: not implemented");
    }
    /// Creates a new empty `OnceCell` instance.
    ///
    /// Equivalent to `OnceCell::new`, except that it can be used in static
    /// variables.
    ///
    /// When using the `tracing` [unstable feature], a `OnceCell` created with
    /// `const_new` will not be instrumented. As such, it will not be visible
    /// in [`tokio-console`]. Instead, [`OnceCell::new`] should be used to
    /// create an instrumented object if that is needed.
    ///
    /// # Example
    ///
    /// ```
    /// use tokio::sync::OnceCell;
    ///
    /// static ONCE: OnceCell<u32> = OnceCell::const_new();
    ///
    /// async fn get_global_integer() -> &'static u32 {
    ///     ONCE.get_or_init(|| async {
    ///         1 + 1
    ///     }).await
    /// }
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let result = get_global_integer().await;
    /// assert_eq!(*result, 2);
    /// # }
    /// ```
    ///
    /// [`tokio-console`]: https://github.com/tokio-rs/console
    /// [unstable feature]: crate#unstable-features
    #[cfg(not(all(loom, test)))]
    pub const fn const_new() -> Self {
        OnceCell {
            value_set: AtomicBool::new(false),
            value: UnsafeCell::new(MaybeUninit::uninit()),
            semaphore: Semaphore::const_new(1),
        }
    }
    /// Creates a new `OnceCell` that contains the provided value, if any.
    ///
    /// If the `Option` is `None`, this is equivalent to `OnceCell::new`.
    ///
    /// [`OnceCell::new`]: crate::sync::OnceCell::new
    pub fn new_with(value: Option<T>) -> Self {
        panic!("STUB: not implemented");
    }
    /// Creates a new `OnceCell` that contains the provided value.
    ///
    /// # Example
    ///
    /// When using the `tracing` [unstable feature], a `OnceCell` created with
    /// `const_new_with` will not be instrumented. As such, it will not be
    /// visible in [`tokio-console`]. Instead, [`OnceCell::new_with`] should be
    /// used to create an instrumented object if that is needed.
    ///
    /// ```
    /// use tokio::sync::OnceCell;
    ///
    /// static ONCE: OnceCell<u32> = OnceCell::const_new_with(1);
    ///
    /// async fn get_global_integer() -> &'static u32 {
    ///     ONCE.get_or_init(|| async {
    ///         1 + 1
    ///     }).await
    /// }
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let result = get_global_integer().await;
    /// assert_eq!(*result, 1);
    /// # }
    /// ```
    ///
    /// [`tokio-console`]: https://github.com/tokio-rs/console
    /// [unstable feature]: crate#unstable-features
    #[cfg(not(all(loom, test)))]
    pub const fn const_new_with(value: T) -> Self {
        OnceCell {
            value_set: AtomicBool::new(true),
            value: UnsafeCell::new(MaybeUninit::new(value)),
            semaphore: Semaphore::const_new_closed(),
        }
    }
    /// Returns `true` if the `OnceCell` currently contains a value, and `false`
    /// otherwise.
    pub fn initialized(&self) -> bool {
        panic!("STUB: not implemented");
    }
    /// Returns `true` if the `OnceCell` currently contains a value, and `false`
    /// otherwise.
    fn initialized_mut(&mut self) -> bool {
        panic!("STUB: not implemented");
    }
    unsafe fn get_unchecked(&self) -> &T {
        panic!("STUB: not implemented");
    }
    unsafe fn get_unchecked_mut(&mut self) -> &mut T {
        panic!("STUB: not implemented");
    }
    fn set_value(&self, value: T, permit: SemaphorePermit<'_>) -> &T {
        panic!("STUB: not implemented");
    }
    /// Returns a reference to the value currently stored in the `OnceCell`, or
    /// `None` if the `OnceCell` is empty.
    pub fn get(&self) -> Option<&T> {
        panic!("STUB: not implemented");
    }
    /// Returns a mutable reference to the value currently stored in the
    /// `OnceCell`, or `None` if the `OnceCell` is empty.
    ///
    /// Since this call borrows the `OnceCell` mutably, it is safe to mutate the
    /// value inside the `OnceCell` — the mutable borrow statically guarantees
    /// no other references exist.
    pub fn get_mut(&mut self) -> Option<&mut T> {
        panic!("STUB: not implemented");
    }
    /// Sets the value of the `OnceCell` to the given value if the `OnceCell` is
    /// empty.
    ///
    /// If the `OnceCell` already has a value, this call will fail with an
    /// [`SetError::AlreadyInitializedError`].
    ///
    /// If the `OnceCell` is empty, but some other task is currently trying to
    /// set the value, this call will fail with [`SetError::InitializingError`].
    ///
    /// [`SetError::AlreadyInitializedError`]: crate::sync::SetError::AlreadyInitializedError
    /// [`SetError::InitializingError`]: crate::sync::SetError::InitializingError
    pub fn set(&self, value: T) -> Result<(), SetError<T>> {
        panic!("STUB: not implemented");
    }
    /// Gets the value currently in the `OnceCell`, or initialize it with the
    /// given asynchronous operation.
    ///
    /// If some other task is currently working on initializing the `OnceCell`,
    /// this call will wait for that other task to finish, then return the value
    /// that the other task produced.
    ///
    /// If the provided operation is cancelled or panics, the initialization
    /// attempt is cancelled. If there are other tasks waiting for the value to
    /// be initialized, one of them will start another attempt at initializing
    /// the value.
    ///
    /// This will deadlock if `f` tries to initialize the cell recursively.
    pub async fn get_or_init<F, Fut>(&self, f: F) -> &T
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = T>,
    {
        panic!("STUB: not implemented");
    }
    /// Gets the value currently in the `OnceCell`, or initialize it with the
    /// given asynchronous operation.
    ///
    /// If some other task is currently working on initializing the `OnceCell`,
    /// this call will wait for that other task to finish, then return the value
    /// that the other task produced.
    ///
    /// If the provided operation returns an error, is cancelled or panics, the
    /// initialization attempt is cancelled. If there are other tasks waiting
    /// for the value to be initialized, one of them will start another attempt
    /// at initializing the value.
    ///
    /// This will deadlock if `f` tries to initialize the cell recursively.
    pub async fn get_or_try_init<E, F, Fut>(&self, f: F) -> Result<&T, E>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<T, E>>,
    {
        panic!("STUB: not implemented");
    }
    /// Takes the value from the cell, destroying the cell in the process.
    /// Returns `None` if the cell is empty.
    pub fn into_inner(mut self) -> Option<T> {
        panic!("STUB: not implemented");
    }
    /// Takes ownership of the current value, leaving the cell empty.  Returns
    /// `None` if the cell is empty.
    pub fn take(&mut self) -> Option<T> {
        panic!("STUB: not implemented");
    }
}
unsafe impl<T: Sync + Send> Sync for OnceCell<T> {}
unsafe impl<T: Send> Send for OnceCell<T> {}
/// Errors that can be returned from [`OnceCell::set`].
///
/// [`OnceCell::set`]: crate::sync::OnceCell::set
#[derive(Debug, PartialEq, Eq)]
pub enum SetError<T> {
    /// The cell was already initialized when [`OnceCell::set`] was called.
    ///
    /// [`OnceCell::set`]: crate::sync::OnceCell::set
    AlreadyInitializedError(T),
    /// The cell is currently being initialized.
    InitializingError(T),
}
impl<T> fmt::Display for SetError<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl<T: fmt::Debug> Error for SetError<T> {}
impl<T> SetError<T> {
    /// Whether `SetError` is `SetError::AlreadyInitializedError`.
    pub fn is_already_init_err(&self) -> bool {
        panic!("STUB: not implemented");
    }
    /// Whether `SetError` is `SetError::InitializingError`
    pub fn is_initializing_err(&self) -> bool {
        panic!("STUB: not implemented");
    }
}
