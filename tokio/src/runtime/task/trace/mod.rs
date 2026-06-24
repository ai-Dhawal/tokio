use crate::loom::sync::Arc;
use crate::runtime::context;
use crate::runtime::scheduler::{self, current_thread, Inject};
use crate::task::Id;
use backtrace::BacktraceFrame;
use std::cell::Cell;
use std::collections::VecDeque;
use std::ffi::c_void;
use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::ptr::NonNull;
use std::task::{self, Poll};
mod symbol;
mod trace_impl;
mod tree;
use symbol::Symbol;
use tree::Tree;
use super::{Notified, OwnedTasks, Schedule};
type Backtrace = Vec<BacktraceFrame>;
type SymbolTrace = Vec<Symbol>;
/// The ambient backtracing context.
pub(crate) struct Context {
    /// The address of [`Trace::root`] establishes an upper unwinding bound on
    /// the backtraces in `Trace`.
    active_frame: Cell<Option<NonNull<Frame>>>,
    /// The function that is invoked at each leaf future inside of Tokio
    ///
    /// For example, within tokio::time:sleep, sockets. etc.
    #[allow(clippy::type_complexity)]
    trace_leaf_fn: Cell<Option<NonNull<dyn FnMut(&TraceMeta)>>>,
}
/// A [`Frame`] in an intrusive, doubly-linked tree of [`Frame`]s.
struct Frame {
    /// The location associated with this frame.
    inner_addr: *const c_void,
    /// The parent frame, if any.
    ///
    /// Tracking parent allows nested `Root` futures to correctly manage their boundaries
    parent: Option<NonNull<Frame>>,
}
/// An tree execution trace.
///
/// Traces are captured with [`Trace::capture`], rooted with [`Trace::root`]
/// and leaved with [`trace_leaf`].
#[derive(Clone, Debug)]
pub(crate) struct Trace {
    backtraces: Vec<Backtrace>,
}
pin_project_lite::pin_project! {
    #[derive(Debug, Clone)] #[must_use =
    "futures do nothing unless you `.await` or poll them"] #[doc =
    " A future wrapper that roots traces (captured with [`Trace::capture`])."] pub struct
    Root < T > { #[pin] future : T, }
}
const FAIL_NO_THREAD_LOCAL: &str = "The Tokio thread-local has been destroyed \
                                    as part of shutting down the current \
                                    thread, so collecting a taskdump is not \
                                    possible.";
impl Context {
    pub(crate) const fn new() -> Self {
        Context {
            active_frame: Cell::new(None),
            trace_leaf_fn: Cell::new(None),
        }
    }
    /// SAFETY: Callers of this function must ensure that trace frames always
    /// form a valid linked list.
    unsafe fn try_with_current<F, R>(f: F) -> Option<R>
    where
        F: FnOnce(&Self) -> R,
    {
        panic!("STUB: not implemented");
    }
    /// SAFETY: Callers of this function must ensure that trace frames always
    /// form a valid linked list.
    unsafe fn with_current_frame<F, R>(f: F) -> R
    where
        F: FnOnce(&Cell<Option<NonNull<Frame>>>) -> R,
    {
        panic!("STUB: not implemented");
    }
    fn current_frame_addr() -> Option<*const c_void> {
        panic!("STUB: not implemented");
    }
    /// Calls the provided closure if we are being traced.
    fn try_with_current_trace_leaf_fn<F, R>(f: F) -> Option<R>
    where
        F: for<'a> FnOnce(&'a mut dyn FnMut(&TraceMeta)) -> R,
    {
        panic!("STUB: not implemented");
    }
    /// Produces `true` if the current task is being traced; otherwise false.
    pub(crate) fn is_tracing() -> bool {
        panic!("STUB: not implemented");
    }
}
/// Metadata passed into the `trace_leaf` callback for [`trace_with`]
#[non_exhaustive]
#[derive(Debug)]
pub struct TraceMeta {
    /// The root boundary address set by [`Root::poll`] if any.
    ///
    /// When using unwinding the stack, this is the address at which
    /// stack walking should stop. It corresponds to the `Root::poll` function pointer.
    pub root_addr: Option<*const c_void>,
    /// The address of the internal `trace_leaf` function that triggered this callback.
    ///
    /// When capturing a backtrace, use this as the lower bound — frames at or below
    /// this address are internal implementation details and should be excluded.
    pub trace_leaf_addr: *const c_void,
}
/// Runs `f`. If `f` hits a Tokio yield point `trace_leaf` will be invoked.
///
/// This allows taking a task dump with caller-provided task dump machinery. If `f` is the poll
/// function of a future and that future returns `Poll::Pending`, then `trace_leaf` will be
/// invoked. `trace_leaf` can then take a backtrace to determine exactly where the yield occurred.
///
/// # Example
///
/// ```
/// use std::future::Future;
/// use std::task::Poll;
/// use tokio::runtime::dump::{trace_with, Trace, TraceMeta};
///
/// fn my_trace_leaf(_meta: &TraceMeta, count: &mut u32) {
///     *count += 1;
/// }
///
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() {
/// let mut fut = std::pin::pin!(async {
///     tokio::task::yield_now().await;
/// });
///
/// let mut leaf_count = 0;
///
/// Trace::root(std::future::poll_fn(|cx| {
///     trace_with(
///         || { let _ = fut.as_mut().poll(cx); },
///         |meta| my_trace_leaf(meta, &mut leaf_count),
///     );
///     Poll::Ready(())
/// })).await;
///
/// assert!(leaf_count > 0);
/// # }
/// ```
pub fn trace_with<FN, FT, R>(f: FN, mut trace_leaf: FT) -> R
where
    FN: FnOnce() -> R,
    FT: FnMut(&TraceMeta),
{
    panic!("STUB: not implemented");
}
impl Trace {
    /// Invokes `f`, returning both its result and the collection of backtraces
    /// captured at each sub-invocation of [`trace_leaf`].
    #[inline(never)]
    pub(crate) fn capture<F, R>(f: F) -> (R, Trace)
    where
        F: FnOnce() -> R,
    {
        panic!("STUB: not implemented");
    }
    pub(crate) fn empty() -> Self {
        panic!("STUB: not implemented");
    }
    fn push_backtrace(&mut self, bt: Vec<BacktraceFrame>) {
        panic!("STUB: not implemented");
    }
    /// The root of a trace.
    #[inline(never)]
    pub(crate) fn root<F>(future: F) -> Root<F> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn backtraces(&self) -> &[Backtrace] {
        panic!("STUB: not implemented");
    }
}
/// If this is a sub-invocation of [`trace_with`], capture a backtrace.
///
/// The captured backtrace will be returned by [`trace_with`].
///
/// Invoking this function does nothing when it is not a sub-invocation
/// [`trace_with`].
#[inline(never)]
pub(crate) fn trace_leaf() -> Poll<()> {
    panic!("STUB: not implemented");
}
impl fmt::Display for Trace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
fn defer<F: FnOnce() -> R, R>(f: F) -> impl Drop {
    panic!("STUB: not implemented");
    #[allow(unreachable_code)] loop {}
}
impl<T: Future> Future for Root<T> {
    type Output = T::Output;
    #[inline(never)]
    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        panic!("STUB: not implemented");
    }
}
/// Trace and poll all tasks of the `current_thread` runtime.
pub(in crate::runtime) fn trace_current_thread(
    owned: &OwnedTasks<Arc<current_thread::Handle>>,
    local: &mut VecDeque<Notified<Arc<current_thread::Handle>>>,
    injection: &Inject<Arc<current_thread::Handle>>,
) -> Vec<(Id, Trace)> {
    panic!("STUB: not implemented");
}
cfg_rt_multi_thread! {
    use crate ::loom::sync::Mutex; use crate ::runtime::scheduler::multi_thread; use
    crate ::runtime::scheduler::multi_thread::Synced; use crate
    ::runtime::scheduler::inject::Shared; #[doc =
    " Trace and poll all tasks of the `current_thread` runtime."] #[doc = ""] #[doc =
    " ## Safety"] #[doc = ""] #[doc =
    " Must be called with the same `synced` that `injection` was created with."] pub (in
    crate ::runtime) unsafe fn trace_multi_thread(owned : & OwnedTasks < Arc <
    multi_thread::Handle >>, local : & mut multi_thread::queue::Local < Arc <
    multi_thread::Handle >>, synced : & Mutex < Synced >, injection : & Shared < Arc <
    multi_thread::Handle >>,) -> Vec < (Id, Trace) > { let mut dequeued = Vec::new();
    while let Some(notified) = local.pop() { dequeued.push(notified); } let mut synced =
    synced.lock(); while let Some(notified) = unsafe { injection.pop(& mut synced.inject)
    } { dequeued.push(notified); } drop(synced); trace_owned(owned, dequeued) }
}
/// Trace the `OwnedTasks`.
///
/// # Preconditions
///
/// This helper presumes exclusive access to each task. The tasks must not exist
/// in any other queue.
fn trace_owned<S: Schedule>(
    owned: &OwnedTasks<S>,
    dequeued: Vec<Notified<S>>,
) -> Vec<(Id, Trace)> {
    panic!("STUB: not implemented");
}
