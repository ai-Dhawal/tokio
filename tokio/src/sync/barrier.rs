use crate::loom::sync::Mutex;
use crate::sync::watch;
#[cfg(all(tokio_unstable, feature = "tracing"))]
use crate::util::trace;
/// A barrier enables multiple tasks to synchronize the beginning of some computation.
///
/// ```
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() {
/// use tokio::sync::Barrier;
/// use std::sync::Arc;
///
/// let mut handles = Vec::with_capacity(10);
/// let barrier = Arc::new(Barrier::new(10));
/// for _ in 0..10 {
///     let c = barrier.clone();
///     // The same messages will be printed together.
///     // You will NOT see any interleaving.
///     handles.push(tokio::spawn(async move {
///         println!("before wait");
///         let wait_result = c.wait().await;
///         println!("after wait");
///         wait_result
///     }));
/// }
///
/// // Will not resolve until all "after wait" messages have been printed
/// let mut num_leaders = 0;
/// for handle in handles {
///     let wait_result = handle.await.unwrap();
///     if wait_result.is_leader() {
///         num_leaders += 1;
///     }
/// }
///
/// // Exactly one barrier will resolve as the "leader"
/// assert_eq!(num_leaders, 1);
/// # }
/// ```
#[derive(Debug)]
pub struct Barrier {
    state: Mutex<BarrierState>,
    wait: watch::Receiver<usize>,
    n: usize,
    #[cfg(all(tokio_unstable, feature = "tracing"))]
    resource_span: tracing::Span,
}
#[derive(Debug)]
struct BarrierState {
    waker: watch::Sender<usize>,
    arrived: usize,
    generation: usize,
}
impl Barrier {
    /// Creates a new barrier that can block a given number of tasks.
    ///
    /// A barrier will block `n`-1 tasks which call [`Barrier::wait`] and then wake up all
    /// tasks at once when the `n`th task calls `wait`.
    #[track_caller]
    pub fn new(mut n: usize) -> Barrier {
        panic!("STUB: not implemented");
    }
    /// Does not resolve until all tasks have rendezvoused here.
    ///
    /// Barriers are re-usable after all tasks have rendezvoused once, and can
    /// be used continuously.
    ///
    /// A single (arbitrary) future will receive a [`BarrierWaitResult`] that returns `true` from
    /// [`BarrierWaitResult::is_leader`] when returning from this function, and all other tasks
    /// will receive a result that will return `false` from `is_leader`.
    ///
    /// # Cancel safety
    ///
    /// This method is not cancel safe.
    pub async fn wait(&self) -> BarrierWaitResult {
        panic!("STUB: not implemented");
    }
    async fn wait_internal(&self) -> BarrierWaitResult {
        panic!("STUB: not implemented");
    }
}
/// A `BarrierWaitResult` is returned by `wait` when all tasks in the `Barrier` have rendezvoused.
#[derive(Debug, Clone)]
pub struct BarrierWaitResult(bool);
impl BarrierWaitResult {
    /// Returns `true` if this task from wait is the "leader task".
    ///
    /// Only one task will have `true` returned from their result, all other tasks will have
    /// `false` returned.
    pub fn is_leader(&self) -> bool {
        panic!("STUB: not implemented");
    }
}
