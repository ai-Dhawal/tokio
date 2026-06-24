//! Runs `!Send` futures on the current thread.
use crate::loom::cell::UnsafeCell;
use crate::loom::sync::{Arc, Mutex};
use crate::runtime;
use crate::runtime::task::{
    self, JoinHandle, LocalOwnedTasks, SpawnLocation, Task, TaskHarnessScheduleHooks,
};
use crate::runtime::{context, ThreadId, BOX_FUTURE_THRESHOLD};
use crate::sync::AtomicWaker;
use crate::util::trace::SpawnMeta;
use crate::util::RcCell;
use std::cell::Cell;
use std::collections::VecDeque;
use std::fmt;
use std::future::Future;
use std::marker::PhantomData;
use std::mem;
use std::pin::Pin;
use std::rc::Rc;
use std::task::Poll;
use pin_project_lite::pin_project;
cfg_rt! {
    #[doc = " A set of tasks which are executed on the same thread."] #[doc = ""] #[doc =
    " In some cases, it is necessary to run one or more futures that do not"] #[doc =
    " implement [`Send`] and thus are unsafe to send between threads. In these"] #[doc =
    " cases, a [local task set] may be used to schedule one or more `!Send`"] #[doc =
    " futures to run together on the same thread."] #[doc = ""] #[doc =
    " For example, the following code will not compile:"] #[doc = ""] #[doc =
    " ```rust,compile_fail"] #[doc = " use std::rc::Rc;"] #[doc = ""] #[doc =
    " #[tokio::main]"] #[doc = " async fn main() {"] #[doc =
    "     // `Rc` does not implement `Send`, and thus may not be sent between"] #[doc =
    "     // threads safely."] #[doc =
    "     let nonsend_data = Rc::new(\"my nonsend data...\");"] #[doc = ""] #[doc =
    "     let nonsend_data = nonsend_data.clone();"] #[doc =
    "     // Because the `async` block here moves `nonsend_data`, the future is `!Send`."]
    #[doc =
    "     // Since `tokio::spawn` requires the spawned future to implement `Send`, this"]
    #[doc = "     // will not compile."] #[doc = "     tokio::spawn(async move {"] #[doc
    = "         println!(\"{}\", nonsend_data);"] #[doc = "         // ..."] #[doc =
    "     }).await.unwrap();"] #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc =
    " # Use with `run_until`"] #[doc = ""] #[doc =
    " To spawn `!Send` futures, we can use a local task set to schedule them"] #[doc =
    " on the thread calling [`Runtime::block_on`]. When running inside of the"] #[doc =
    " local task set, we can use [`task::spawn_local`], which can spawn"] #[doc =
    " `!Send` futures. For example:"] #[doc = ""] #[doc = " ```rust"] #[doc =
    " use std::rc::Rc;"] #[doc = " use tokio::task;"] #[doc = ""] #[doc =
    " # #[tokio::main(flavor = \"current_thread\")]"] #[doc = " # async fn main() {"]
    #[doc = " let nonsend_data = Rc::new(\"my nonsend data...\");"] #[doc = ""] #[doc =
    " // Construct a local task set that can run `!Send` futures."] #[doc =
    " let local = task::LocalSet::new();"] #[doc = ""] #[doc =
    " // Run the local task set."] #[doc = " local.run_until(async move {"] #[doc =
    "     let nonsend_data = nonsend_data.clone();"] #[doc =
    "     // `spawn_local` ensures that the future is spawned on the local"] #[doc =
    "     // task set."] #[doc = "     task::spawn_local(async move {"] #[doc =
    "         println!(\"{}\", nonsend_data);"] #[doc = "         // ..."] #[doc =
    "     }).await.unwrap();"] #[doc = " }).await;"] #[doc = " # }"] #[doc = " ```"]
    #[doc = " **Note:** The `run_until` method can only be used in `#[tokio::main]`,"]
    #[doc = " `#[tokio::test]` or directly inside a call to [`Runtime::block_on`]. It"]
    #[doc = " cannot be used inside a task spawned with `tokio::spawn`."] #[doc = ""]
    #[doc = " ## Awaiting a `LocalSet`"] #[doc = ""] #[doc =
    " Additionally, a `LocalSet` itself implements `Future`, completing when"] #[doc =
    " *all* tasks spawned on the `LocalSet` complete. This can be used to run"] #[doc =
    " several futures on a `LocalSet` and drive the whole set until they"] #[doc =
    " complete. For example,"] #[doc = ""] #[doc = " ```rust"] #[doc =
    " use tokio::{task, time};"] #[doc = " use std::rc::Rc;"] #[doc = ""] #[doc =
    " # #[tokio::main(flavor = \"current_thread\")]"] #[doc = " # async fn main() {"]
    #[doc = " let nonsend_data = Rc::new(\"world\");"] #[doc =
    " let local = task::LocalSet::new();"] #[doc = ""] #[doc =
    " let nonsend_data2 = nonsend_data.clone();"] #[doc =
    " local.spawn_local(async move {"] #[doc = "     // ..."] #[doc =
    "     println!(\"hello {}\", nonsend_data2)"] #[doc = " });"] #[doc = ""] #[doc =
    " local.spawn_local(async move {"] #[doc =
    "     time::sleep(time::Duration::from_millis(100)).await;"] #[doc =
    "     println!(\"goodbye {}\", nonsend_data)"] #[doc = " });"] #[doc = ""] #[doc =
    " // ..."] #[doc = ""] #[doc = " local.await;"] #[doc = " # }"] #[doc = " ```"] #[doc
    = " **Note:** Awaiting a `LocalSet` can only be done inside"] #[doc =
    " `#[tokio::main]`, `#[tokio::test]` or directly inside a call to"] #[doc =
    " [`Runtime::block_on`]. It cannot be used inside a task spawned with"] #[doc =
    " `tokio::spawn`."] #[doc = ""] #[doc = " ## Use inside `tokio::spawn`"] #[doc = ""]
    #[doc = " The two methods mentioned above cannot be used inside `tokio::spawn`, so"]
    #[doc = " to spawn `!Send` futures from inside `tokio::spawn`, we need to do"] #[doc
    = " something else. The solution is to create the `LocalSet` somewhere else,"] #[doc
    = " and communicate with it using an [`mpsc`] channel."] #[doc = ""] #[doc =
    " The following example puts the `LocalSet` inside a new thread."] #[doc = " ```"]
    #[doc = " # #[cfg(not(target_family = \"wasm\"))]"] #[doc = " # {"] #[doc =
    " use tokio::runtime::Builder;"] #[doc = " use tokio::sync::{mpsc, oneshot};"] #[doc
    = " use tokio::task::LocalSet;"] #[doc = ""] #[doc =
    " // This struct describes the task you want to spawn. Here we include"] #[doc =
    " // some simple examples. The oneshot channel allows sending a response"] #[doc =
    " // to the spawner."] #[doc = " #[derive(Debug)]"] #[doc = " enum Task {"] #[doc =
    "     PrintNumber(u32),"] #[doc = "     AddOne(u32, oneshot::Sender<u32>),"] #[doc =
    " }"] #[doc = ""] #[doc = " #[derive(Clone)]"] #[doc = " struct LocalSpawner {"]
    #[doc = "    send: mpsc::UnboundedSender<Task>,"] #[doc = " }"] #[doc = ""] #[doc =
    " impl LocalSpawner {"] #[doc = "     pub fn new() -> Self {"] #[doc =
    "         let (send, mut recv) = mpsc::unbounded_channel();"] #[doc = ""] #[doc =
    "         let rt = Builder::new_current_thread()"] #[doc =
    "             .enable_all()"] #[doc = "             .build()"] #[doc =
    "             .unwrap();"] #[doc = ""] #[doc =
    "         std::thread::spawn(move || {"] #[doc =
    "             let local = LocalSet::new();"] #[doc = ""] #[doc =
    "             local.spawn_local(async move {"] #[doc =
    "                 while let Some(new_task) = recv.recv().await {"] #[doc =
    "                     tokio::task::spawn_local(run_task(new_task));"] #[doc =
    "                 }"] #[doc =
    "                 // If the while loop returns, then all the LocalSpawner"] #[doc =
    "                 // objects have been dropped."] #[doc = "             });"] #[doc =
    ""] #[doc = "             // This will return once all senders are dropped and all"]
    #[doc = "             // spawned tasks have returned."] #[doc =
    "             rt.block_on(local);"] #[doc = "         });"] #[doc = ""] #[doc =
    "         Self {"] #[doc = "             send,"] #[doc = "         }"] #[doc =
    "     }"] #[doc = ""] #[doc = "     pub fn spawn(&self, task: Task) {"] #[doc =
    "         self.send.send(task).expect(\"Thread with LocalSet has shut down.\");"]
    #[doc = "     }"] #[doc = " }"] #[doc = ""] #[doc =
    " // This task may do !Send stuff. We use printing a number as an example,"] #[doc =
    " // but it could be anything."] #[doc = " //"] #[doc =
    " // The Task struct is an enum to support spawning many different kinds"] #[doc =
    " // of operations."] #[doc = " async fn run_task(task: Task) {"] #[doc =
    "     match task {"] #[doc = "         Task::PrintNumber(n) => {"] #[doc =
    "             println!(\"{}\", n);"] #[doc = "         },"] #[doc =
    "         Task::AddOne(n, response) => {"] #[doc =
    "             // We ignore failures to send the response."] #[doc =
    "             let _ = response.send(n + 1);"] #[doc = "         },"] #[doc =
    "     }"] #[doc = " }"] #[doc = ""] #[doc = " #[tokio::main]"] #[doc =
    " async fn main() {"] #[doc = "     let spawner = LocalSpawner::new();"] #[doc = ""]
    #[doc = "     let (send, response) = oneshot::channel();"] #[doc =
    "     spawner.spawn(Task::AddOne(10, send));"] #[doc =
    "     let eleven = response.await.unwrap();"] #[doc = "     assert_eq!(eleven, 11);"]
    #[doc = " }"] #[doc = " # }"] #[doc = " ```"] #[doc = ""] #[doc =
    " [`Send`]: trait@std::marker::Send"] #[doc = " [local task set]: struct@LocalSet"]
    #[doc = " [`Runtime::block_on`]: method@crate::runtime::Runtime::block_on"] #[doc =
    " [`task::spawn_local`]: fn@spawn_local"] #[doc = " [`mpsc`]: mod@crate::sync::mpsc"]
    pub struct LocalSet { #[doc = " Current scheduler tick."] tick : Cell < u8 >, #[doc =
    " State available from thread-local."] context : Rc < Context >, #[doc =
    " This type should not be Send."] _not_send : PhantomData <* const () >, }
}
/// State available from the thread-local.
struct Context {
    /// State shared between threads.
    shared: Arc<Shared>,
    /// True if a task panicked without being handled and the local set is
    /// configured to shutdown on unhandled panic.
    unhandled_panic: Cell<bool>,
}
/// `LocalSet` state shared between threads.
struct Shared {
    /// # Safety
    ///
    /// This field must *only* be accessed from the thread that owns the
    /// `LocalSet` (i.e., `Thread::current().id() == owner`).
    local_state: LocalState,
    /// Remote run queue sender.
    queue: Mutex<Option<VecDeque<task::Notified<Arc<Shared>>>>>,
    /// Wake the `LocalSet` task.
    waker: AtomicWaker,
    /// How to respond to unhandled task panics.
    #[cfg(tokio_unstable)]
    pub(crate) unhandled_panic: crate::runtime::UnhandledPanic,
}
/// Tracks the `LocalSet` state that must only be accessed from the thread that
/// created the `LocalSet`.
struct LocalState {
    /// The `ThreadId` of the thread that owns the `LocalSet`.
    owner: ThreadId,
    /// Local run queue sender and receiver.
    local_queue: UnsafeCell<VecDeque<task::Notified<Arc<Shared>>>>,
    /// Collection of all active tasks spawned onto this executor.
    owned: LocalOwnedTasks<Arc<Shared>>,
}
pin_project! {
    #[derive(Debug)] struct RunUntil <'a, F > { local_set : &'a LocalSet, #[pin] future :
    F, }
}
tokio_thread_local!(
    static CURRENT : LocalData = const { LocalData { ctx : RcCell::new(),
    wake_on_schedule : Cell::new(false), } }
);
struct LocalData {
    ctx: RcCell<Context>,
    wake_on_schedule: Cell<bool>,
}
impl LocalData {
    /// Should be called except when we call `LocalSet::enter`.
    /// Especially when we poll a `LocalSet`.
    #[must_use = "dropping this guard will reset the entered state"]
    fn enter(&self, ctx: Rc<Context>) -> LocalDataEnterGuard<'_> {
        panic!("STUB: not implemented");
    }
}
/// A guard for `LocalData::enter()`
struct LocalDataEnterGuard<'a> {
    local_data_ref: &'a LocalData,
    ctx: Option<Rc<Context>>,
    wake_on_schedule: bool,
}
impl<'a> Drop for LocalDataEnterGuard<'a> {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
cfg_rt! {
    #[doc = " Spawns a `!Send` future on the current [`LocalSet`] or [`LocalRuntime`]."]
    #[doc = ""] #[doc =
    " This is possible when either using one of these types explicitly, or by"] #[doc =
    " opting to use the `\"local\"` runtime flavor in `tokio::main`:"] #[doc = ""] #[doc
    = " ```ignore"] #[doc = " #[tokio::main(flavor = \"local\")]"] #[doc = " ```"] #[doc
    = ""] #[doc =
    " The spawned future will run on the same thread that called `spawn_local`."] #[doc =
    ""] #[doc = " The provided future will start running in the background immediately"]
    #[doc = " when `spawn_local` is called, even if you don't await the returned"] #[doc
    = " `JoinHandle`."] #[doc = ""] #[doc = " # Panics"] #[doc = ""] #[doc =
    " This function panics if called outside of a [`LocalSet`] or [`LocalRuntime`]."]
    #[doc = ""] #[doc =
    " Note that if [`tokio::spawn`] is used from within a `LocalSet`, the"] #[doc =
    " resulting new task will _not_ be inside the `LocalSet`, so you must use"] #[doc =
    " `spawn_local` if you want to stay within the `LocalSet`."] #[doc = ""] #[doc =
    " # Examples"] #[doc = ""] #[doc = " With `LocalSet`:"] #[doc = ""] #[doc =
    " ```rust"] #[doc = " use std::rc::Rc;"] #[doc = " use tokio::task;"] #[doc = ""]
    #[doc = " # #[tokio::main(flavor = \"current_thread\")]"] #[doc =
    " # async fn main() {"] #[doc =
    " let nonsend_data = Rc::new(\"my nonsend data...\");"] #[doc = ""] #[doc =
    " let local = task::LocalSet::new();"] #[doc = ""] #[doc =
    " // Run the local task set."] #[doc = " local.run_until(async move {"] #[doc =
    "     let nonsend_data = nonsend_data.clone();"] #[doc =
    "     task::spawn_local(async move {"] #[doc =
    "         println!(\"{}\", nonsend_data);"] #[doc = "         // ..."] #[doc =
    "     }).await.unwrap();"] #[doc = " }).await;"] #[doc = " # }"] #[doc = " ```"]
    #[doc = " With local runtime flavor."] #[doc = ""] #[doc = " ```rust"] #[doc =
    " #[tokio::main(flavor = \"local\")]"] #[doc = " async fn main() {"] #[doc =
    "     let join = tokio::task::spawn_local(async {"] #[doc =
    "         println!(\"my nonsend data...\")"] #[doc = "     });"] #[doc = ""] #[doc =
    "    join.await.unwrap()"] #[doc = "  }"] #[doc = ""] #[doc = " ```"] #[doc = ""]
    #[doc = " [`LocalSet`]: struct@crate::task::LocalSet"] #[doc =
    " [`LocalRuntime`]: struct@crate::runtime::LocalRuntime"] #[doc =
    " [`tokio::spawn`]: fn@crate::task::spawn"] #[doc =
    " [unstable]: ../../tokio/index.html#unstable-features"] #[track_caller] pub fn
    spawn_local < F > (future : F) -> JoinHandle < F::Output > where F : Future +
    'static, F::Output : 'static, { let fut_size = std::mem::size_of::< F > (); if
    fut_size > BOX_FUTURE_THRESHOLD { spawn_local_inner(Box::pin(future),
    SpawnMeta::new_unnamed(fut_size)) } else { spawn_local_inner(future,
    SpawnMeta::new_unnamed(fut_size)) } } #[track_caller] pub (super) fn
    spawn_local_inner < F > (future : F, meta : SpawnMeta <'_ >) -> JoinHandle <
    F::Output > where F : Future + 'static, F::Output : 'static { use crate ::runtime:: {
    context, task }; let mut future = Some(future); let res = context::with_current(|
    handle | { Some(if handle.is_local() { if ! handle.can_spawn_local_on_local_runtime()
    { return None; } let future = future.take().unwrap(); #[cfg(all(tokio_unstable,
    feature = "taskdump", feature = "rt", target_os = "linux", any(target_arch =
    "aarch64", target_arch = "x86", target_arch = "x86_64", target_arch = "s390x")))] let
    future = task::trace::Trace::root(future); let id = task::Id::next(); let task =
    crate ::util::trace::task(future, "task", meta, id.as_u64()); unsafe { handle
    .spawn_local(task, id, meta.spawned_at) } } else { match CURRENT.with(| LocalData {
    ctx, .. } | ctx.get()) { None =>
    panic!("`spawn_local` called from outside of a `task::LocalSet` or `runtime::LocalRuntime`"),
    Some(cx) => cx.spawn(future.take().unwrap(), meta) } }) }); match res { Ok(None) =>
    panic!("Local tasks can only be spawned on a LocalRuntime from the thread the runtime was created on"),
    Ok(Some(join_handle)) => join_handle, Err(_) => match CURRENT.with(| LocalData { ctx,
    .. } | ctx.get()) { None =>
    panic!("`spawn_local` called from outside of a `task::LocalSet` or `runtime::LocalRuntime`"),
    Some(cx) => cx.spawn(future.unwrap(), meta) } } }
}
/// Initial queue capacity.
const INITIAL_CAPACITY: usize = 64;
/// Max number of tasks to poll per tick.
const MAX_TASKS_PER_TICK: usize = 61;
/// How often it check the remote queue first.
const REMOTE_FIRST_INTERVAL: u8 = 31;
/// Context guard for `LocalSet`
pub struct LocalEnterGuard {
    ctx: Option<Rc<Context>>,
    /// Distinguishes whether the context was entered or being polled.
    /// When we enter it, the value `wake_on_schedule` is set. In this case
    /// `spawn_local` refers the context, whereas it is not being polled now.
    wake_on_schedule: bool,
}
impl Drop for LocalEnterGuard {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
impl fmt::Debug for LocalEnterGuard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl LocalSet {
    /// Returns a new local task set.
    pub fn new() -> LocalSet {
        panic!("STUB: not implemented");
    }
    /// Enters the context of this `LocalSet`.
    ///
    /// The [`spawn_local`] method will spawn tasks on the `LocalSet` whose
    /// context you are inside.
    ///
    /// [`spawn_local`]: fn@crate::task::spawn_local
    pub fn enter(&self) -> LocalEnterGuard {
        panic!("STUB: not implemented");
    }
    /// Spawns a `!Send` task onto the local task set.
    ///
    /// This task is guaranteed to be run on the current thread.
    ///
    /// Unlike the free function [`spawn_local`], this method may be used to
    /// spawn local tasks when the `LocalSet` is _not_ running. The provided
    /// future will start running once the `LocalSet` is next started, even if
    /// you don't await the returned `JoinHandle`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tokio::task;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let local = task::LocalSet::new();
    ///
    /// // Spawn a future on the local set. This future will be run when
    /// // we call `run_until` to drive the task set.
    /// local.spawn_local(async {
    ///     // ...
    /// });
    ///
    /// // Run the local task set.
    /// local.run_until(async move {
    ///     // ...
    /// }).await;
    ///
    /// // When `run` finishes, we can spawn _more_ futures, which will
    /// // run in subsequent calls to `run_until`.
    /// local.spawn_local(async {
    ///     // ...
    /// });
    ///
    /// local.run_until(async move {
    ///     // ...
    /// }).await;
    /// # }
    /// ```
    /// [`spawn_local`]: fn@spawn_local
    #[track_caller]
    pub fn spawn_local<F>(&self, future: F) -> JoinHandle<F::Output>
    where
        F: Future + 'static,
        F::Output: 'static,
    {
        panic!("STUB: not implemented");
    }
    /// Runs a future to completion on the provided runtime, driving any local
    /// futures spawned on this task set on the current thread.
    ///
    /// This runs the given future on the runtime, blocking until it is
    /// complete, and yielding its resolved result. Any tasks or timers which
    /// the future spawns internally will be executed on the runtime. The future
    /// may also call [`spawn_local`] to `spawn_local` additional local futures on the
    /// current thread.
    ///
    /// This method should not be called from an asynchronous context.
    ///
    /// # Panics
    ///
    /// This function panics if the executor is at capacity, if the provided
    /// future panics, or if called within an asynchronous execution context.
    ///
    /// # Notes
    ///
    /// Since this function internally calls [`Runtime::block_on`], and drives
    /// futures in the local task set inside that call to `block_on`, the local
    /// futures may not use [in-place blocking]. If a blocking call needs to be
    /// issued from a local task, the [`spawn_blocking`] API may be used instead.
    ///
    /// For example, this will panic:
    /// ```should_panic,ignore-wasm
    /// use tokio::runtime::Runtime;
    /// use tokio::task;
    ///
    /// let rt  = Runtime::new().unwrap();
    /// let local = task::LocalSet::new();
    /// local.block_on(&rt, async {
    ///     let join = task::spawn_local(async {
    ///         let blocking_result = task::block_in_place(|| {
    ///             // ...
    ///         });
    ///         // ...
    ///     });
    ///     join.await.unwrap();
    /// })
    /// ```
    /// This, however, will not panic:
    /// ```
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// use tokio::runtime::Runtime;
    /// use tokio::task;
    ///
    /// let rt  = Runtime::new().unwrap();
    /// let local = task::LocalSet::new();
    /// local.block_on(&rt, async {
    ///     let join = task::spawn_local(async {
    ///         let blocking_result = task::spawn_blocking(|| {
    ///             // ...
    ///         }).await;
    ///         // ...
    ///     });
    ///     join.await.unwrap();
    /// })
    /// # }
    /// ```
    ///
    /// [`spawn_local`]: fn@spawn_local
    /// [`Runtime::block_on`]: method@crate::runtime::Runtime::block_on
    /// [in-place blocking]: fn@crate::task::block_in_place
    /// [`spawn_blocking`]: fn@crate::task::spawn_blocking
    #[track_caller]
    #[cfg(feature = "rt")]
    #[cfg_attr(docsrs, doc(cfg(feature = "rt")))]
    pub fn block_on<F>(&self, rt: &crate::runtime::Runtime, future: F) -> F::Output
    where
        F: Future,
    {
        panic!("STUB: not implemented");
    }
    /// Runs a future to completion on the local set, returning its output.
    ///
    /// This returns a future that runs the given future with a local set,
    /// allowing it to call [`spawn_local`] to spawn additional `!Send` futures.
    /// Any local futures spawned on the local set will be driven in the
    /// background until the future passed to `run_until` completes. When the future
    /// passed to `run_until` finishes, any local futures which have not completed
    /// will remain on the local set, and will be driven on subsequent calls to
    /// `run_until` or when [awaiting the local set] itself.
    ///
    /// # Cancel safety
    ///
    /// This method is cancel safe when `future` is cancel safe.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tokio::task;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// task::LocalSet::new().run_until(async {
    ///     task::spawn_local(async move {
    ///         // ...
    ///     }).await.unwrap();
    ///     // ...
    /// }).await;
    /// # }
    /// ```
    ///
    /// [`spawn_local`]: fn@spawn_local
    /// [awaiting the local set]: #awaiting-a-localset
    pub async fn run_until<F>(&self, future: F) -> F::Output
    where
        F: Future,
    {
        panic!("STUB: not implemented");
    }
    #[track_caller]
    pub(in crate::task) fn spawn_named<F>(
        &self,
        future: F,
        meta: SpawnMeta<'_>,
    ) -> JoinHandle<F::Output>
    where
        F: Future + 'static,
        F::Output: 'static,
    {
        panic!("STUB: not implemented");
    }
    #[track_caller]
    fn spawn_named_inner<F>(
        &self,
        future: F,
        meta: SpawnMeta<'_>,
    ) -> JoinHandle<F::Output>
    where
        F: Future + 'static,
        F::Output: 'static,
    {
        panic!("STUB: not implemented");
    }
    /// Ticks the scheduler, returning whether the local future needs to be
    /// notified again.
    fn tick(&self) -> bool {
        panic!("STUB: not implemented");
    }
    fn next_task(&self) -> Option<task::LocalNotified<Arc<Shared>>> {
        panic!("STUB: not implemented");
    }
    fn pop_local(&self) -> Option<task::Notified<Arc<Shared>>> {
        panic!("STUB: not implemented");
    }
    fn with<T>(&self, f: impl FnOnce() -> T) -> T {
        panic!("STUB: not implemented");
    }
    /// This method is like `with`, but it just calls `f` without setting the thread-local if that
    /// fails.
    fn with_if_possible<T>(&self, f: impl FnOnce() -> T) -> T {
        panic!("STUB: not implemented");
    }
    /// Returns the [`Id`] of the current [`LocalSet`] runtime.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use tokio::task;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let local_set = task::LocalSet::new();
    /// println!("Local set id: {}", local_set.id());
    /// # }
    /// ```
    ///
    /// [`Id`]: struct@crate::runtime::Id
    pub fn id(&self) -> runtime::Id {
        panic!("STUB: not implemented");
    }
}
cfg_unstable! {
    impl LocalSet { #[doc =
    " Configure how the `LocalSet` responds to an unhandled panic on a"] #[doc =
    " spawned task."] #[doc = ""] #[doc =
    " By default, an unhandled panic (i.e. a panic not caught by"] #[doc =
    " [`std::panic::catch_unwind`]) has no impact on the `LocalSet`'s"] #[doc =
    " execution. The panic is error value is forwarded to the task's"] #[doc =
    " [`JoinHandle`] and all other spawned tasks continue running."] #[doc = ""] #[doc =
    " The `unhandled_panic` option enables configuring this behavior."] #[doc = ""] #[doc
    = " * `UnhandledPanic::Ignore` is the default behavior. Panics on"] #[doc =
    "   spawned tasks have no impact on the `LocalSet`'s execution."] #[doc =
    " * `UnhandledPanic::ShutdownRuntime` will force the `LocalSet` to"] #[doc =
    "   shutdown immediately when a spawned task panics even if that"] #[doc =
    "   task's `JoinHandle` has not been dropped. All other spawned tasks"] #[doc =
    "   will immediately terminate and further calls to"] #[doc =
    "   [`LocalSet::block_on`] and [`LocalSet::run_until`] will panic."] #[doc = ""]
    #[doc = " # Panics"] #[doc = ""] #[doc =
    " This method panics if called after the `LocalSet` has started"] #[doc =
    " running."] #[doc = ""] #[doc = " # Unstable"] #[doc = ""] #[doc =
    " This option is currently unstable and its implementation is"] #[doc =
    " incomplete. The API may change or be removed in the future. See"] #[doc =
    " tokio-rs/tokio#4516 for more details."] #[doc = ""] #[doc = " # Examples"] #[doc =
    ""] #[doc = " The following demonstrates a `LocalSet` configured to shutdown on"]
    #[doc = " panic. The first spawned task panics and results in the `LocalSet`"] #[doc
    = " shutting down. The second spawned task never has a chance to"] #[doc =
    " execute. The call to `run_until` will panic due to the runtime being"] #[doc =
    " forcibly shutdown."] #[doc = ""] #[doc = " ```should_panic"] #[doc =
    " use tokio::runtime::UnhandledPanic;"] #[doc = ""] #[doc =
    " # #[tokio::main(flavor = \"current_thread\")]"] #[doc = " # async fn main() {"]
    #[doc = " tokio::task::LocalSet::new()"] #[doc =
    "     .unhandled_panic(UnhandledPanic::ShutdownRuntime)"] #[doc =
    "     .run_until(async {"] #[doc =
    "         tokio::task::spawn_local(async { panic!(\"boom\"); });"] #[doc =
    "         tokio::task::spawn_local(async {"] #[doc =
    "             // This task never completes"] #[doc = "         });"] #[doc = ""]
    #[doc = "         // Do some work, but `run_until` will panic before it completes"]
    #[doc = " # loop { tokio::task::yield_now().await; }"] #[doc = "     })"] #[doc =
    "     .await;"] #[doc = " # }"] #[doc = " ```"] #[doc = ""] #[doc =
    " [`JoinHandle`]: struct@crate::task::JoinHandle"] pub fn unhandled_panic(& mut self,
    behavior : crate ::runtime::UnhandledPanic) -> & mut Self { Rc::get_mut(& mut self
    .context).and_then(| ctx | Arc::get_mut(& mut ctx.shared))
    .expect("Unhandled Panic behavior modified after starting LocalSet").unhandled_panic
    = behavior; self } }
}
impl fmt::Debug for LocalSet {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl Future for LocalSet {
    type Output = ();
    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Self::Output> {
        panic!("STUB: not implemented");
    }
}
impl Default for LocalSet {
    fn default() -> LocalSet {
        panic!("STUB: not implemented");
    }
}
impl Drop for LocalSet {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
impl Context {
    #[track_caller]
    fn spawn<F>(&self, future: F, meta: SpawnMeta<'_>) -> JoinHandle<F::Output>
    where
        F: Future + 'static,
        F::Output: 'static,
    {
        panic!("STUB: not implemented");
    }
}
impl<T: Future> Future for RunUntil<'_, T> {
    type Output = T::Output;
    fn poll(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Self::Output> {
        panic!("STUB: not implemented");
    }
}
impl Shared {
    /// Schedule the provided task on the scheduler.
    fn schedule(&self, task: task::Notified<Arc<Self>>) {
        panic!("STUB: not implemented");
    }
    fn ptr_eq(&self, other: &Shared) -> bool {
        panic!("STUB: not implemented");
    }
}
unsafe impl Sync for Shared {}
impl task::Schedule for Arc<Shared> {
    fn release(&self, task: &Task<Self>) -> Option<Task<Self>> {
        panic!("STUB: not implemented");
    }
    fn schedule(&self, task: task::Notified<Self>) {
        panic!("STUB: not implemented");
    }
    fn hooks(&self) -> TaskHarnessScheduleHooks {
        panic!("STUB: not implemented");
    }
    cfg_unstable! {
        fn unhandled_panic(& self) { use crate ::runtime::UnhandledPanic; match self
        .unhandled_panic { UnhandledPanic::Ignore => {} UnhandledPanic::ShutdownRuntime
        => { CURRENT.with(| LocalData { ctx, .. } | match ctx.get() { Some(cx) if
        Arc::ptr_eq(self, & cx.shared) => { cx.unhandled_panic.set(true); unsafe { cx
        .shared.local_state.close_and_shutdown_all(); } } _ =>
        unreachable!("runtime core not set in CURRENT thread-local"), }) } } }
    }
}
impl LocalState {
    /// # Safety
    ///
    /// This method must only be called from the thread who
    /// has the same [`ThreadId`] as [`Self::owner`].
    unsafe fn task_pop_front(&self) -> Option<task::Notified<Arc<Shared>>> {
        panic!("STUB: not implemented");
    }
    /// # Safety
    ///
    /// This method must only be called from the thread who
    /// has the same [`ThreadId`] as [`Self::owner`].
    unsafe fn task_push_back(&self, task: task::Notified<Arc<Shared>>) {
        panic!("STUB: not implemented");
    }
    /// # Safety
    ///
    /// This method must only be called from the thread who
    /// has the same [`ThreadId`] as [`Self::owner`].
    unsafe fn take_local_queue(&self) -> VecDeque<task::Notified<Arc<Shared>>> {
        panic!("STUB: not implemented");
    }
    unsafe fn task_remove(&self, task: &Task<Arc<Shared>>) -> Option<Task<Arc<Shared>>> {
        panic!("STUB: not implemented");
    }
    /// Returns true if the `LocalSet` does not have any spawned tasks
    unsafe fn owned_is_empty(&self) -> bool {
        panic!("STUB: not implemented");
    }
    unsafe fn assert_owner(
        &self,
        task: task::Notified<Arc<Shared>>,
    ) -> task::LocalNotified<Arc<Shared>> {
        panic!("STUB: not implemented");
    }
    unsafe fn close_and_shutdown_all(&self) {
        panic!("STUB: not implemented");
    }
    #[track_caller]
    fn assert_called_from_owner_thread(&self) {
        panic!("STUB: not implemented");
    }
}
unsafe impl Send for LocalState {}
#[cfg(all(test, not(loom)))]
mod tests {
    use super::*;
    #[test]
    fn local_current_thread_scheduler() {
        let f = async {
            LocalSet::new()
                .run_until(async {
                    spawn_local(async {}).await.unwrap();
                })
                .await;
        };
        crate::runtime::Builder::new_current_thread().build().expect("rt").block_on(f)
    }
    #[test]
    fn wakes_to_local_queue() {
        use super::*;
        use crate::sync::Notify;
        let rt = crate::runtime::Builder::new_current_thread().build().expect("rt");
        rt.block_on(async {
            let local = LocalSet::new();
            let notify = Arc::new(Notify::new());
            let task = local
                .spawn_local({
                    let notify = notify.clone();
                    async move {
                        notify.notified().await;
                    }
                });
            let mut run_until = Box::pin(
                local
                    .run_until(async move {
                        task.await.unwrap();
                    }),
            );
            std::future::poll_fn(|cx| {
                    let _ = run_until.as_mut().poll(cx);
                    Poll::Ready(())
                })
                .await;
            notify.notify_one();
            let task = unsafe { local.context.shared.local_state.task_pop_front() };
            assert!(
                task.is_some(),
                "task should have been notified to the LocalSet's local queue"
            );
        })
    }
}
