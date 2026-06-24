//! A collection of tasks spawned on a Tokio runtime.
//!
//! This module provides the [`JoinSet`] type, a collection which stores a set
//! of spawned tasks and allows asynchronously awaiting the output of those
//! tasks as they complete. See the documentation for the [`JoinSet`] type for
//! details.
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::{fmt, panic};
use crate::runtime::Handle;
use crate::task::Id;
use crate::task::{unconstrained, AbortHandle, JoinError, JoinHandle, LocalSet};
use crate::util::IdleNotifiedSet;
/// A collection of tasks spawned on a Tokio runtime.
///
/// A `JoinSet` can be used to await the completion of some or all of the tasks
/// in the set. The set is not ordered, and the tasks will be returned in the
/// order they complete.
///
/// All of the tasks must have the same return type `T`.
///
/// When the `JoinSet` is dropped, all tasks in the `JoinSet` are immediately aborted.
///
/// # Examples
///
/// Spawn multiple tasks and wait for them.
///
/// ```
/// use tokio::task::JoinSet;
///
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() {
/// let mut set = JoinSet::new();
///
/// for i in 0..10 {
///     set.spawn(async move { i });
/// }
///
/// let mut seen = [false; 10];
/// while let Some(res) = set.join_next().await {
///     let idx = res.unwrap();
///     seen[idx] = true;
/// }
///
/// for i in 0..10 {
///     assert!(seen[i]);
/// }
/// # }
/// ```
///
/// # Task ID guarantees
///
/// While a task is tracked in a `JoinSet`, that task's ID is unique relative
/// to all other running tasks in Tokio. For this purpose, tracking a task in a
/// `JoinSet` is equivalent to holding a [`JoinHandle`] to it. See the [task ID]
/// documentation for more info.
///
/// [`JoinHandle`]: crate::task::JoinHandle
/// [task ID]: crate::task::Id
#[cfg_attr(docsrs, doc(cfg(feature = "rt")))]
pub struct JoinSet<T> {
    inner: IdleNotifiedSet<JoinHandle<T>>,
}
/// A variant of [`task::Builder`] that spawns tasks on a [`JoinSet`] rather
/// than on the current default runtime.
///
/// [`task::Builder`]: crate::task::Builder
#[cfg(all(tokio_unstable, feature = "tracing"))]
#[cfg_attr(docsrs, doc(cfg(all(tokio_unstable, feature = "tracing"))))]
#[must_use = "builders do nothing unless used to spawn a task"]
pub struct Builder<'a, T> {
    joinset: &'a mut JoinSet<T>,
    builder: super::Builder<'a>,
}
impl<T> JoinSet<T> {
    /// Create a new `JoinSet`.
    pub fn new() -> Self {
        panic!("STUB: not implemented");
    }
    /// Returns the number of tasks currently in the `JoinSet`.
    pub fn len(&self) -> usize {
        panic!("STUB: not implemented");
    }
    /// Returns whether the `JoinSet` is empty.
    pub fn is_empty(&self) -> bool {
        panic!("STUB: not implemented");
    }
}
impl<T: 'static> JoinSet<T> {
    /// Returns a [`Builder`] that can be used to configure a task prior to
    /// spawning it on this `JoinSet`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::task::JoinSet;
    ///
    /// #[tokio::main]
    /// async fn main() -> std::io::Result<()> {
    ///     let mut set = JoinSet::new();
    ///
    ///     // Use the builder to configure a task's name before spawning it.
    ///     set.build_task()
    ///         .name("my_task")
    ///         .spawn(async { /* ... */ })?;
    ///
    ///     Ok(())
    /// }
    /// ```
    #[cfg(all(tokio_unstable, feature = "tracing"))]
    #[cfg_attr(docsrs, doc(cfg(all(tokio_unstable, feature = "tracing"))))]
    pub fn build_task(&mut self) -> Builder<'_, T> {
        panic!("STUB: not implemented");
    }
    /// Spawn the provided task on the `JoinSet`, returning an [`AbortHandle`]
    /// that can be used to remotely cancel the task.
    ///
    /// The provided future will start running in the background immediately
    /// when this method is called, even if you don't await anything on this
    /// `JoinSet`.
    ///
    /// # Panics
    ///
    /// This method panics if called outside of a Tokio runtime.
    ///
    /// [`AbortHandle`]: crate::task::AbortHandle
    #[track_caller]
    pub fn spawn<F>(&mut self, task: F) -> AbortHandle
    where
        F: Future<Output = T>,
        F: Send + 'static,
        T: Send,
    {
        panic!("STUB: not implemented");
    }
    /// Spawn the provided task on the provided runtime and store it in this
    /// `JoinSet` returning an [`AbortHandle`] that can be used to remotely
    /// cancel the task.
    ///
    /// The provided future will start running in the background immediately
    /// when this method is called, even if you don't await anything on this
    /// `JoinSet`.
    ///
    /// [`AbortHandle`]: crate::task::AbortHandle
    #[track_caller]
    pub fn spawn_on<F>(&mut self, task: F, handle: &Handle) -> AbortHandle
    where
        F: Future<Output = T>,
        F: Send + 'static,
        T: Send,
    {
        panic!("STUB: not implemented");
    }
    /// Spawn the provided task on the current [`LocalSet`] or [`LocalRuntime`]
    /// and store it in this `JoinSet`, returning an [`AbortHandle`] that can
    /// be used to remotely cancel the task.
    ///
    /// The provided future will start running in the background immediately
    /// when this method is called, even if you don't await anything on this
    /// `JoinSet`.
    ///
    /// # Panics
    ///
    /// This method panics if it is called outside of a `LocalSet` or `LocalRuntime`.
    ///
    /// [`LocalSet`]: crate::task::LocalSet
    /// [`LocalRuntime`]: crate::runtime::LocalRuntime
    /// [`AbortHandle`]: crate::task::AbortHandle
    #[track_caller]
    pub fn spawn_local<F>(&mut self, task: F) -> AbortHandle
    where
        F: Future<Output = T>,
        F: 'static,
    {
        panic!("STUB: not implemented");
    }
    /// Spawn the provided task on the provided [`LocalSet`] and store it in
    /// this `JoinSet`, returning an [`AbortHandle`] that can be used to
    /// remotely cancel the task.
    ///
    /// Unlike the [`spawn_local`] method, this method may be used to spawn local
    /// tasks on a `LocalSet` that is _not_ currently running. The provided
    /// future will start running whenever the `LocalSet` is next started.
    ///
    /// [`LocalSet`]: crate::task::LocalSet
    /// [`AbortHandle`]: crate::task::AbortHandle
    /// [`spawn_local`]: Self::spawn_local
    #[track_caller]
    pub fn spawn_local_on<F>(&mut self, task: F, local_set: &LocalSet) -> AbortHandle
    where
        F: Future<Output = T>,
        F: 'static,
    {
        panic!("STUB: not implemented");
    }
    /// Spawn the blocking code on the blocking threadpool and store
    /// it in this `JoinSet`, returning an [`AbortHandle`] that can be
    /// used to remotely cancel the task.
    ///
    /// # Examples
    ///
    /// Spawn multiple blocking tasks and wait for them.
    ///
    /// ```
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// use tokio::task::JoinSet;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let mut set = JoinSet::new();
    ///
    ///     for i in 0..10 {
    ///         set.spawn_blocking(move || { i });
    ///     }
    ///
    ///     let mut seen = [false; 10];
    ///     while let Some(res) = set.join_next().await {
    ///         let idx = res.unwrap();
    ///         seen[idx] = true;
    ///     }
    ///
    ///     for i in 0..10 {
    ///         assert!(seen[i]);
    ///     }
    /// }
    /// # }
    /// ```
    ///
    /// # Panics
    ///
    /// This method panics if called outside of a Tokio runtime.
    ///
    /// [`AbortHandle`]: crate::task::AbortHandle
    #[track_caller]
    pub fn spawn_blocking<F>(&mut self, f: F) -> AbortHandle
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send,
    {
        panic!("STUB: not implemented");
    }
    /// Spawn the blocking code on the blocking threadpool of the
    /// provided runtime and store it in this `JoinSet`, returning an
    /// [`AbortHandle`] that can be used to remotely cancel the task.
    ///
    /// [`AbortHandle`]: crate::task::AbortHandle
    #[track_caller]
    pub fn spawn_blocking_on<F>(&mut self, f: F, handle: &Handle) -> AbortHandle
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send,
    {
        panic!("STUB: not implemented");
    }
    fn insert(&mut self, jh: JoinHandle<T>) -> AbortHandle {
        panic!("STUB: not implemented");
    }
    /// Waits until one of the tasks in the set completes and returns its output.
    ///
    /// Returns `None` if the set is empty.
    ///
    /// # Cancel safety
    ///
    /// This method is cancel safe. If `join_next` is used as a branch in
    /// `tokio::select!` and another branch completes first, it is guaranteed
    /// that no tasks were removed from this `JoinSet`.
    pub async fn join_next(&mut self) -> Option<Result<T, JoinError>> {
        panic!("STUB: not implemented");
    }
    /// Waits until one of the tasks in the set completes and returns its
    /// output, along with the [task ID] of the completed task.
    ///
    /// Returns `None` if the set is empty.
    ///
    /// When this method returns an error, then the id of the task that failed can be accessed
    /// using the [`JoinError::id`] method.
    ///
    /// # Cancel safety
    ///
    /// This method is cancel safe. If `join_next_with_id` is used as a branch
    /// in `tokio::select!` and another branch completes first, it is
    /// guaranteed that no tasks were removed from this `JoinSet`.
    ///
    /// [task ID]: crate::task::Id
    /// [`JoinError::id`]: fn@crate::task::JoinError::id
    pub async fn join_next_with_id(&mut self) -> Option<Result<(Id, T), JoinError>> {
        panic!("STUB: not implemented");
    }
    /// Tries to join one of the tasks in the set that has completed and return its output.
    ///
    /// Returns `None` if there are no completed tasks, or if the set is empty.
    pub fn try_join_next(&mut self) -> Option<Result<T, JoinError>> {
        panic!("STUB: not implemented");
    }
    /// Tries to join one of the tasks in the set that has completed and return its output,
    /// along with the [task ID] of the completed task.
    ///
    /// Returns `None` if there are no completed tasks, or if the set is empty.
    ///
    /// When this method returns an error, then the id of the task that failed can be accessed
    /// using the [`JoinError::id`] method.
    ///
    /// [task ID]: crate::task::Id
    /// [`JoinError::id`]: fn@crate::task::JoinError::id
    pub fn try_join_next_with_id(&mut self) -> Option<Result<(Id, T), JoinError>> {
        panic!("STUB: not implemented");
    }
    /// Aborts all tasks and waits for them to finish shutting down.
    ///
    /// Calling this method is equivalent to calling [`abort_all`] and then calling [`join_next`] in
    /// a loop until it returns `None`.
    ///
    /// This method ignores any panics in the tasks shutting down. When this call returns, the
    /// `JoinSet` will be empty.
    ///
    /// [`abort_all`]: fn@Self::abort_all
    /// [`join_next`]: fn@Self::join_next
    pub async fn shutdown(&mut self) {
        panic!("STUB: not implemented");
    }
    /// Awaits the completion of all tasks in this `JoinSet`, returning a vector of their results.
    ///
    /// The results will be stored in the order they completed not the order they were spawned.
    /// This is a convenience method that is equivalent to calling [`join_next`] in
    /// a loop. If any tasks on the `JoinSet` fail with an [`JoinError`], then this call
    /// to `join_all` will panic and all remaining tasks on the `JoinSet` are
    /// cancelled. To handle errors in any other way, manually call [`join_next`]
    /// in a loop.
    ///
    /// # Examples
    ///
    /// Spawn multiple tasks and `join_all` them.
    ///
    /// ```
    /// use tokio::task::JoinSet;
    /// use std::time::Duration;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let mut set = JoinSet::new();
    ///
    /// for i in 0..3 {
    ///     set.spawn(async move {
    ///         tokio::time::sleep(Duration::from_secs(3 - i)).await;
    ///         i
    ///     });
    /// }
    ///
    /// let output = set.join_all().await;
    /// assert_eq!(output, vec![2, 1, 0]);
    /// # }
    /// ```
    ///
    /// Equivalent implementation of `join_all`, using [`join_next`] and loop.
    ///
    /// ```
    /// use tokio::task::JoinSet;
    /// use std::panic;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let mut set = JoinSet::new();
    ///
    /// for i in 0..3 {
    ///     set.spawn(async move {i});
    /// }
    ///
    /// let mut output = Vec::new();
    /// while let Some(res) = set.join_next().await{
    ///     match res {
    ///         Ok(t) => output.push(t),
    ///         Err(err) if err.is_panic() => panic::resume_unwind(err.into_panic()),
    ///         Err(err) => panic!("{err}"),
    ///     }
    /// }
    /// assert_eq!(output.len(),3);
    /// # }
    /// ```
    /// [`join_next`]: fn@Self::join_next
    /// [`JoinError::id`]: fn@crate::task::JoinError::id
    pub async fn join_all(mut self) -> Vec<T> {
        panic!("STUB: not implemented");
    }
    /// Aborts all tasks on this `JoinSet`.
    ///
    /// This does not remove the tasks from the `JoinSet`. To wait for the tasks to complete
    /// cancellation, you should call `join_next` in a loop until the `JoinSet` is empty.
    pub fn abort_all(&mut self) {
        panic!("STUB: not implemented");
    }
    /// Removes all tasks from this `JoinSet` without aborting them.
    ///
    /// The tasks removed by this call will continue to run in the background even if the `JoinSet`
    /// is dropped.
    pub fn detach_all(&mut self) {
        panic!("STUB: not implemented");
    }
    /// Polls for one of the tasks in the set to complete.
    ///
    /// If this returns `Poll::Ready(Some(_))`, then the task that completed is removed from the set.
    ///
    /// When the method returns `Poll::Pending`, the `Waker` in the provided `Context` is scheduled
    /// to receive a wakeup when a task in the `JoinSet` completes. Note that on multiple calls to
    /// `poll_join_next`, only the `Waker` from the `Context` passed to the most recent call is
    /// scheduled to receive a wakeup.
    ///
    /// # Returns
    ///
    /// This function returns:
    ///
    ///  * `Poll::Pending` if the `JoinSet` is not empty but there is no task whose output is
    ///    available right now.
    ///  * `Poll::Ready(Some(Ok(value)))` if one of the tasks in this `JoinSet` has completed.
    ///    The `value` is the return value of one of the tasks that completed.
    ///  * `Poll::Ready(Some(Err(err)))` if one of the tasks in this `JoinSet` has panicked or been
    ///    aborted. The `err` is the `JoinError` from the panicked/aborted task.
    ///  * `Poll::Ready(None)` if the `JoinSet` is empty.
    ///
    /// Note that this method may return `Poll::Pending` even if one of the tasks has completed.
    /// This can happen if the [coop budget] is reached.
    ///
    /// [coop budget]: crate::task::coop#cooperative-scheduling
    pub fn poll_join_next(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<T, JoinError>>> {
        panic!("STUB: not implemented");
    }
    /// Polls for one of the tasks in the set to complete.
    ///
    /// If this returns `Poll::Ready(Some(_))`, then the task that completed is removed from the set.
    ///
    /// When the method returns `Poll::Pending`, the `Waker` in the provided `Context` is scheduled
    /// to receive a wakeup when a task in the `JoinSet` completes. Note that on multiple calls to
    /// `poll_join_next`, only the `Waker` from the `Context` passed to the most recent call is
    /// scheduled to receive a wakeup.
    ///
    /// # Returns
    ///
    /// This function returns:
    ///
    ///  * `Poll::Pending` if the `JoinSet` is not empty but there is no task whose output is
    ///    available right now.
    ///  * `Poll::Ready(Some(Ok((id, value))))` if one of the tasks in this `JoinSet` has completed.
    ///    The `value` is the return value of one of the tasks that completed, and
    ///    `id` is the [task ID] of that task.
    ///  * `Poll::Ready(Some(Err(err)))` if one of the tasks in this `JoinSet` has panicked or been
    ///    aborted. The `err` is the `JoinError` from the panicked/aborted task.
    ///  * `Poll::Ready(None)` if the `JoinSet` is empty.
    ///
    /// Note that this method may return `Poll::Pending` even if one of the tasks has completed.
    /// This can happen if the [coop budget] is reached.
    ///
    /// [coop budget]: crate::task::coop#cooperative-scheduling
    /// [task ID]: crate::task::Id
    pub fn poll_join_next_with_id(
        &mut self,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<(Id, T), JoinError>>> {
        panic!("STUB: not implemented");
    }
}
impl<T> Drop for JoinSet<T> {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
impl<T> fmt::Debug for JoinSet<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl<T> Default for JoinSet<T> {
    fn default() -> Self {
        panic!("STUB: not implemented");
    }
}
/// Collect an iterator of futures into a [`JoinSet`].
///
/// This is equivalent to calling [`JoinSet::spawn`] on each element of the iterator.
///
/// # Examples
///
/// The main example from [`JoinSet`]'s documentation can also be written using [`collect`]:
///
/// ```
/// use tokio::task::JoinSet;
///
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() {
/// let mut set: JoinSet<_> = (0..10).map(|i| async move { i }).collect();
///
/// let mut seen = [false; 10];
/// while let Some(res) = set.join_next().await {
///     let idx = res.unwrap();
///     seen[idx] = true;
/// }
///
/// for i in 0..10 {
///      assert!(seen[i]);
/// }
/// # }
/// ```
///
/// [`collect`]: std::iter::Iterator::collect
impl<T, F> std::iter::FromIterator<F> for JoinSet<T>
where
    F: Future<Output = T>,
    F: Send + 'static,
    T: Send + 'static,
{
    fn from_iter<I: IntoIterator<Item = F>>(iter: I) -> Self {
        panic!("STUB: not implemented");
    }
}
/// Extend a [`JoinSet`] with futures from an iterator.
///
/// This is equivalent to calling [`JoinSet::spawn`] on each element of the iterator.
///
/// # Examples
///
/// ```
/// # #[cfg(not(target_family = "wasm"))]
/// # {
/// use tokio::task::JoinSet;
///
/// #[tokio::main]
/// async fn main() {
///     let mut set: JoinSet<_> = (0..5).map(|i| async move { i }).collect();
///
///     set.extend((5..10).map(|i| async move { i }));
///
///     let mut seen = [false; 10];
///     while let Some(res) = set.join_next().await {
///         let idx = res.unwrap();
///         seen[idx] = true;
///     }
///
///     for i in 0..10 {
///         assert!(seen[i]);
///     }
/// }
/// # }
/// ```
impl<T, F> std::iter::Extend<F> for JoinSet<T>
where
    F: Future<Output = T>,
    F: Send + 'static,
    T: Send + 'static,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = F>,
    {
        panic!("STUB: not implemented");
    }
}
#[cfg(all(tokio_unstable, feature = "tracing"))]
#[cfg_attr(docsrs, doc(cfg(all(tokio_unstable, feature = "tracing"))))]
impl<'a, T: 'static> Builder<'a, T> {
    /// Assigns a name to the task which will be spawned.
    pub fn name(self, name: &'a str) -> Self {
        panic!("STUB: not implemented");
    }
    /// Spawn the provided task with this builder's settings and store it in the
    /// [`JoinSet`], returning an [`AbortHandle`] that can be used to remotely
    /// cancel the task.
    ///
    /// # Returns
    ///
    /// An [`AbortHandle`] that can be used to remotely cancel the task.
    ///
    /// # Panics
    ///
    /// This method panics if called outside of a Tokio runtime.
    ///
    /// [`AbortHandle`]: crate::task::AbortHandle
    #[track_caller]
    pub fn spawn<F>(self, future: F) -> std::io::Result<AbortHandle>
    where
        F: Future<Output = T>,
        F: Send + 'static,
        T: Send,
    {
        panic!("STUB: not implemented");
    }
    /// Spawn the provided task on the provided [runtime handle] with this
    /// builder's settings, and store it in the [`JoinSet`].
    ///
    /// # Returns
    ///
    /// An [`AbortHandle`] that can be used to remotely cancel the task.
    ///
    ///
    /// [`AbortHandle`]: crate::task::AbortHandle
    /// [runtime handle]: crate::runtime::Handle
    #[track_caller]
    pub fn spawn_on<F>(self, future: F, handle: &Handle) -> std::io::Result<AbortHandle>
    where
        F: Future<Output = T>,
        F: Send + 'static,
        T: Send,
    {
        panic!("STUB: not implemented");
    }
    /// Spawn the blocking code on the blocking threadpool with this builder's
    /// settings, and store it in the [`JoinSet`].
    ///
    /// # Returns
    ///
    /// An [`AbortHandle`] that can be used to remotely cancel the task.
    ///
    /// # Panics
    ///
    /// This method panics if called outside of a Tokio runtime.
    ///
    /// [`JoinSet`]: crate::task::JoinSet
    /// [`AbortHandle`]: crate::task::AbortHandle
    #[track_caller]
    pub fn spawn_blocking<F>(self, f: F) -> std::io::Result<AbortHandle>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send,
    {
        panic!("STUB: not implemented");
    }
    /// Spawn the blocking code on the blocking threadpool of the provided
    /// runtime handle with this builder's settings, and store it in the
    /// [`JoinSet`].
    ///
    /// # Returns
    ///
    /// An [`AbortHandle`] that can be used to remotely cancel the task.
    ///
    /// [`JoinSet`]: crate::task::JoinSet
    /// [`AbortHandle`]: crate::task::AbortHandle
    #[track_caller]
    pub fn spawn_blocking_on<F>(
        self,
        f: F,
        handle: &Handle,
    ) -> std::io::Result<AbortHandle>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send,
    {
        panic!("STUB: not implemented");
    }
    /// Spawn the provided task on the current [`LocalSet`] or [`LocalRuntime`]
    /// with this builder's settings, and store it in the [`JoinSet`].
    ///
    /// # Returns
    ///
    /// An [`AbortHandle`] that can be used to remotely cancel the task.
    ///
    /// # Panics
    ///
    /// This method panics if it is called outside of a `LocalSet` or `LocalRuntime`.
    ///
    /// [`LocalSet`]: crate::task::LocalSet
    /// [`LocalRuntime`]: crate::runtime::LocalRuntime
    /// [`AbortHandle`]: crate::task::AbortHandle
    #[track_caller]
    pub fn spawn_local<F>(self, future: F) -> std::io::Result<AbortHandle>
    where
        F: Future<Output = T>,
        F: 'static,
    {
        panic!("STUB: not implemented");
    }
    /// Spawn the provided task on the provided [`LocalSet`] with this builder's
    /// settings, and store it in the [`JoinSet`].
    ///
    /// # Returns
    ///
    /// An [`AbortHandle`] that can be used to remotely cancel the task.
    ///
    /// [`LocalSet`]: crate::task::LocalSet
    /// [`AbortHandle`]: crate::task::AbortHandle
    #[track_caller]
    pub fn spawn_local_on<F>(
        self,
        future: F,
        local_set: &LocalSet,
    ) -> std::io::Result<AbortHandle>
    where
        F: Future<Output = T>,
        F: 'static,
    {
        panic!("STUB: not implemented");
    }
}
#[cfg(all(tokio_unstable, feature = "tracing"))]
#[cfg_attr(docsrs, doc(cfg(all(tokio_unstable, feature = "tracing"))))]
impl<'a, T> fmt::Debug for Builder<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
