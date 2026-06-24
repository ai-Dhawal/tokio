//! Snapshots of runtime state.
//!
//! See [`Handle::dump`][crate::runtime::Handle::dump].
use crate::task::Id;
use std::{fmt, future::Future, path::Path};
pub use crate::runtime::task::trace::{trace_with, Root, TraceMeta};
/// A snapshot of a runtime's state.
///
/// See [`Handle::dump`][crate::runtime::Handle::dump].
#[derive(Debug)]
pub struct Dump {
    tasks: Tasks,
}
/// Snapshots of tasks.
///
/// See [`Handle::dump`][crate::runtime::Handle::dump].
#[derive(Debug)]
pub struct Tasks {
    tasks: Vec<Task>,
}
/// A snapshot of a task.
///
/// See [`Handle::dump`][crate::runtime::Handle::dump].
#[derive(Debug)]
pub struct Task {
    id: Id,
    trace: Trace,
}
/// Represents an address that should not be dereferenced.
///
/// This type exists to get the auto traits correct, the public API
/// uses raw pointers to make life easier for users.
#[derive(Copy, Clone, Debug)]
struct Address(*mut std::ffi::c_void);
unsafe impl Send for Address {}
unsafe impl Sync for Address {}
/// A backtrace symbol.
///
/// This struct provides accessors for backtrace symbols, similar to [`backtrace::BacktraceSymbol`].
#[derive(Clone, Debug)]
pub struct BacktraceSymbol {
    name: Option<Box<[u8]>>,
    name_demangled: Option<Box<str>>,
    addr: Option<Address>,
    filename: Option<std::path::PathBuf>,
    lineno: Option<u32>,
    colno: Option<u32>,
}
impl BacktraceSymbol {
    pub(crate) fn from_backtrace_symbol(sym: &backtrace::BacktraceSymbol) -> Self {
        panic!("STUB: not implemented");
    }
    /// Return the raw name of the symbol.
    pub fn name_raw(&self) -> Option<&[u8]> {
        panic!("STUB: not implemented");
    }
    /// Return the demangled name of the symbol.
    pub fn name_demangled(&self) -> Option<&str> {
        panic!("STUB: not implemented");
    }
    /// Returns the starting address of this symbol.
    pub fn addr(&self) -> Option<*mut std::ffi::c_void> {
        panic!("STUB: not implemented");
    }
    /// Returns the file name where this function was defined. If debuginfo
    /// is missing, this is likely to return None.
    pub fn filename(&self) -> Option<&Path> {
        panic!("STUB: not implemented");
    }
    /// Returns the line number for where this symbol is currently executing.
    ///
    /// If debuginfo is missing, this is likely to return `None`.
    pub fn lineno(&self) -> Option<u32> {
        panic!("STUB: not implemented");
    }
    /// Returns the column number for where this symbol is currently executing.
    ///
    /// If debuginfo is missing, this is likely to return `None`.
    pub fn colno(&self) -> Option<u32> {
        panic!("STUB: not implemented");
    }
}
/// A backtrace frame.
///
/// This struct represents one stack frame in a captured backtrace, similar to [`backtrace::BacktraceFrame`].
#[derive(Clone, Debug)]
pub struct BacktraceFrame {
    ip: Address,
    symbol_address: Address,
    symbols: Box<[BacktraceSymbol]>,
}
impl BacktraceFrame {
    pub(crate) fn from_resolved_backtrace_frame(
        frame: &backtrace::BacktraceFrame,
    ) -> Self {
        panic!("STUB: not implemented");
    }
    /// Return the instruction pointer of this frame.
    ///
    /// See the ABI docs for your platform for the exact meaning.
    pub fn ip(&self) -> *mut std::ffi::c_void {
        panic!("STUB: not implemented");
    }
    /// Returns the starting symbol address of the frame of this function.
    pub fn symbol_address(&self) -> *mut std::ffi::c_void {
        panic!("STUB: not implemented");
    }
    /// Return an iterator over the symbols of this backtrace frame.
    ///
    /// Due to inlining, it is possible for there to be multiple [`BacktraceSymbol`] items relating
    /// to a single frame. The first symbol listed is the "innermost function",
    /// whereas the last symbol is the outermost (last caller).
    pub fn symbols(&self) -> impl Iterator<Item = &BacktraceSymbol> {
        panic!("STUB: not implemented");
        #[allow(unreachable_code)] std::iter::empty::<&BacktraceSymbol>()
    }
}
/// A captured backtrace.
///
/// This struct provides access to each backtrace frame, similar to [`backtrace::Backtrace`].
#[derive(Clone, Debug)]
pub struct Backtrace {
    frames: Box<[BacktraceFrame]>,
}
impl Backtrace {
    /// Return the frames in this backtrace, innermost (in a task dump,
    /// likely to be a leaf future's poll function) first.
    pub fn frames(&self) -> impl Iterator<Item = &BacktraceFrame> {
        panic!("STUB: not implemented");
        #[allow(unreachable_code)] std::iter::empty::<&BacktraceFrame>()
    }
}
/// An execution trace of a task's last poll.
///
/// <div class="warning">
///
/// Resolving a backtrace, either via the [`Display`][std::fmt::Display] impl or via
/// [`resolve_backtraces`][Trace::resolve_backtraces], parses debuginfo, which is
/// possibly a CPU-expensive operation that can take a platform-specific but
/// long time to run - often over 100 milliseconds, especially if the current
/// process's binary is big. In some cases, the platform might internally cache some of the
/// debuginfo, so successive calls to `resolve_backtraces` might be faster than
/// the first call, but all guarantees are platform-dependent.
///
/// To avoid blocking the runtime, it is recommended
/// that you resolve backtraces inside of a [`spawn_blocking()`][crate::task::spawn_blocking]
/// and to have some concurrency-limiting mechanism to avoid unexpected performance impact.
/// </div>
///
/// See [`Handle::dump`][crate::runtime::Handle::dump].
#[derive(Debug)]
pub struct Trace {
    inner: super::task::trace::Trace,
}
impl Trace {
    /// Resolve and return a list of backtraces that are involved in polls in this trace.
    ///
    /// The exact backtraces included here are unstable and might change in the future,
    /// but you can expect one [`Backtrace`] for every call to
    /// [`poll`] to a bottom-level Tokio future - so if something like [`join!`] is
    /// used, there will be a backtrace for each future in the join.
    ///
    /// [`poll`]: std::future::Future::poll
    /// [`join!`]: macro@join
    pub fn resolve_backtraces(&self) -> Vec<Backtrace> {
        panic!("STUB: not implemented");
    }
    /// Runs the function `f` in tracing mode, and returns its result along with the resulting [`Trace`].
    ///
    /// This is normally called with `f` being the poll function of a future, and will give you a backtrace
    /// that tells you what that one future is doing.
    ///
    /// Use [`Handle::dump`] instead if you want to know what *all the tasks* in your program are doing.
    /// Also see [`Handle::dump`] for more documentation about dumps, but unlike [`Handle::dump`], this function
    /// should not be much slower than calling `f` directly.
    ///
    /// Due to the way tracing is implemented, Tokio leaf futures will usually, instead of doing their
    /// actual work, return `Poll::Pending` without registering the task's waker with any driver.
    /// This means forward progress will probably not happen unless you eventually call your future
    /// outside of `capture`, or explicitly re-schedule the task (e.g. by calling
    /// [`cx.waker().wake_by_ref()`][std::task::Waker::wake_by_ref]) after `capture` returns.
    ///
    /// [`Handle::dump`]: crate::runtime::Handle::dump
    ///
    /// Example usage:
    /// ```
    /// use std::future::Future;
    /// use std::task::Poll;
    /// use tokio::runtime::dump::Trace;
    ///
    /// # async fn test_fn() {
    /// // some future
    /// let mut test_future = std::pin::pin!(async move { tokio::task::yield_now().await; 0 });
    ///
    /// // trace it once, see what it's doing
    /// let (trace, res) = Trace::root(std::future::poll_fn(|cx| {
    ///     let (res, trace) = Trace::capture(|| test_future.as_mut().poll(cx));
    ///     Poll::Ready((trace, res))
    /// })).await;
    ///
    /// // await it to let it finish, outside of a `capture`
    /// let output = match res {
    ///    Poll::Ready(output) => output,
    ///    Poll::Pending => test_future.await,
    /// };
    ///
    /// println!("{trace}");
    /// # }
    /// ```
    ///
    /// ### Nested calls
    ///
    /// Nested calls to `capture` might return partial traces, but will not do any other undesirable behavior (for
    /// example, they will not panic).
    pub fn capture<F, R>(f: F) -> (R, Trace)
    where
        F: FnOnce() -> R,
    {
        panic!("STUB: not implemented");
    }
    /// Create a root for stack traces captured using [`Trace::capture`]. Stack frames above
    /// the root will not be captured.
    ///
    /// Nesting multiple [`Root`] futures is fine. Captures will stop at the first root. Not having
    /// a [`Root`] is fine as well, but there is no guarantee on where the capture will stop.
    pub fn root<F>(f: F) -> Root<F>
    where
        F: Future,
    {
        panic!("STUB: not implemented");
    }
}
impl Dump {
    pub(crate) fn new(tasks: Vec<Task>) -> Self {
        panic!("STUB: not implemented");
    }
    /// Tasks in this snapshot.
    pub fn tasks(&self) -> &Tasks {
        panic!("STUB: not implemented");
    }
}
impl Tasks {
    /// Iterate over tasks.
    pub fn iter(&self) -> impl Iterator<Item = &Task> {
        panic!("STUB: not implemented");
        #[allow(unreachable_code)] std::iter::empty::<&Task>()
    }
}
impl Task {
    pub(crate) fn new(id: Id, trace: super::task::trace::Trace) -> Self {
        panic!("STUB: not implemented");
    }
    /// Returns a [task ID] that uniquely identifies this task relative to other
    /// tasks spawned at the time of the dump.
    ///
    /// **Note**: This is an [unstable API][unstable]. The public API of this type
    /// may break in 1.x releases. See [the documentation on unstable
    /// features][unstable] for details.
    ///
    /// [task ID]: crate::task::Id
    /// [unstable]: crate#unstable-features
    #[cfg(tokio_unstable)]
    #[cfg_attr(docsrs, doc(cfg(tokio_unstable)))]
    pub fn id(&self) -> Id {
        panic!("STUB: not implemented");
    }
    /// A trace of this task's state.
    pub fn trace(&self) -> &Trace {
        panic!("STUB: not implemented");
    }
}
impl fmt::Display for Trace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
