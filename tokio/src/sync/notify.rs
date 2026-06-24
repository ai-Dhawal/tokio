#![cfg_attr(not(feature = "sync"), allow(unreachable_pub, dead_code))]
use crate::loom::cell::UnsafeCell;
use crate::loom::sync::atomic::AtomicUsize;
use crate::loom::sync::Mutex;
use crate::util::linked_list::{self, GuardedLinkedList, LinkedList};
use crate::util::WakeList;
use std::future::Future;
use std::marker::PhantomPinned;
use std::panic::{RefUnwindSafe, UnwindSafe};
use std::pin::Pin;
use std::ptr::NonNull;
use std::sync::atomic::Ordering::{self, Acquire, Relaxed, Release, SeqCst};
use std::sync::Arc;
use std::task::{Context, Poll, Waker};
/// Notifies a single task to wake up.
///
/// `Notify` provides a basic mechanism to notify a single task of an event.
/// `Notify` itself does not carry any data. Instead, it is to be used to signal
/// another task to perform an operation.
///
/// A `Notify` can be thought of as a [`Semaphore`] starting with 0 permits. The
/// [`notified().await`] method waits for a permit to become available, and
/// [`notify_one()`] sets a permit **if there currently are no available
/// permits**.
///
/// The synchronization details of `Notify` are similar to
/// [`thread::park`][park] and [`Thread::unpark`][unpark] from std. A [`Notify`]
/// value contains a single permit. [`notified().await`] waits for the permit to
/// be made available, consumes the permit, and resumes.  [`notify_one()`] sets
/// the permit, waking a pending task if there is one.
///
/// If `notify_one()` is called **before** `notified().await`, then the next
/// call to `notified().await` will complete immediately, consuming the permit.
/// Any subsequent calls to `notified().await` will wait for a new permit.
///
/// If `notify_one()` is called **multiple** times before `notified().await`,
/// only a **single** permit is stored. The next call to `notified().await` will
/// complete immediately, but the one after will wait for a new permit.
///
/// # Examples
///
/// Basic usage.
///
/// ```
/// use tokio::sync::Notify;
/// use std::sync::Arc;
///
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() {
/// let notify = Arc::new(Notify::new());
/// let notify2 = notify.clone();
///
/// let handle = tokio::spawn(async move {
///     notify2.notified().await;
///     println!("received notification");
/// });
///
/// println!("sending notification");
/// notify.notify_one();
///
/// // Wait for task to receive notification.
/// handle.await.unwrap();
/// # }
/// ```
///
/// Unbound multi-producer single-consumer (mpsc) channel.
///
/// No wakeups can be lost when using this channel because the call to
/// `notify_one()` will store a permit in the `Notify`, which the following call
/// to `notified()` will consume.
///
/// ```
/// use tokio::sync::Notify;
///
/// use std::collections::VecDeque;
/// use std::sync::Mutex;
///
/// struct Channel<T> {
///     values: Mutex<VecDeque<T>>,
///     notify: Notify,
/// }
///
/// impl<T> Channel<T> {
///     pub fn send(&self, value: T) {
///         self.values.lock().unwrap()
///             .push_back(value);
///
///         // Notify the consumer a value is available
///         self.notify.notify_one();
///     }
///
///     // This is a single-consumer channel, so several concurrent calls to
///     // `recv` are not allowed.
///     pub async fn recv(&self) -> T {
///         loop {
///             // Drain values
///             if let Some(value) = self.values.lock().unwrap().pop_front() {
///                 return value;
///             }
///
///             // Wait for values to be available
///             self.notify.notified().await;
///         }
///     }
/// }
/// ```
///
/// Unbound multi-producer multi-consumer (mpmc) channel.
///
/// The call to [`enable`] is important because otherwise if you have two
/// calls to `recv` and two calls to `send` in parallel, the following could
/// happen:
///
///  1. Both calls to `try_recv` return `None`.
///  2. Both new elements are added to the vector.
///  3. The `notify_one` method is called twice, adding only a single
///     permit to the `Notify`.
///  4. Both calls to `recv` reach the `Notified` future. One of them
///     consumes the permit, and the other sleeps forever.
///
/// By adding the `Notified` futures to the list by calling `enable` before
/// `try_recv`, the `notify_one` calls in step three would remove the
/// futures from the list and mark them notified instead of adding a permit
/// to the `Notify`. This ensures that both futures are woken.
///
/// Notice that this failure can only happen if there are two concurrent calls
/// to `recv`. This is why the mpsc example above does not require a call to
/// `enable`.
///
/// ```
/// use tokio::sync::Notify;
///
/// use std::collections::VecDeque;
/// use std::sync::Mutex;
///
/// struct Channel<T> {
///     messages: Mutex<VecDeque<T>>,
///     notify_on_sent: Notify,
/// }
///
/// impl<T> Channel<T> {
///     pub fn send(&self, msg: T) {
///         let mut locked_queue = self.messages.lock().unwrap();
///         locked_queue.push_back(msg);
///         drop(locked_queue);
///
///         // Send a notification to one of the calls currently
///         // waiting in a call to `recv`.
///         self.notify_on_sent.notify_one();
///     }
///
///     pub fn try_recv(&self) -> Option<T> {
///         let mut locked_queue = self.messages.lock().unwrap();
///         locked_queue.pop_front()
///     }
///
///     pub async fn recv(&self) -> T {
///         let future = self.notify_on_sent.notified();
///         tokio::pin!(future);
///
///         loop {
///             // Make sure that no wakeup is lost if we get
///             // `None` from `try_recv`.
///             future.as_mut().enable();
///
///             if let Some(msg) = self.try_recv() {
///                 return msg;
///             }
///
///             // Wait for a call to `notify_one`.
///             //
///             // This uses `.as_mut()` to avoid consuming the future,
///             // which lets us call `Pin::set` below.
///             future.as_mut().await;
///
///             // Reset the future in case another call to
///             // `try_recv` got the message before us.
///             future.set(self.notify_on_sent.notified());
///         }
///     }
/// }
/// ```
///
/// [park]: std::thread::park
/// [unpark]: std::thread::Thread::unpark
/// [`notified().await`]: Notify::notified()
/// [`notify_one()`]: Notify::notify_one()
/// [`enable`]: Notified::enable()
/// [`Semaphore`]: crate::sync::Semaphore
#[derive(Debug)]
pub struct Notify {
    state: AtomicUsize,
    waiters: Mutex<LinkedList<Waiter>>,
}
#[derive(Debug)]
struct Waiter {
    /// Intrusive linked-list pointers.
    pointers: linked_list::Pointers<Waiter>,
    /// Waiting task's waker. Depending on the value of `notification`,
    /// this field is either protected by the `waiters` lock in
    /// `Notify`, or it is exclusively owned by the enclosing `Waiter`.
    waker: UnsafeCell<Option<Waker>>,
    /// Notification for this waiter. Uses 2 bits to store if and how was
    /// notified, 1 bit for storing if it was woken up using FIFO or LIFO, and
    /// the rest of it is unused.
    /// * if it's `None`, then `waker` is protected by the `waiters` lock.
    /// * if it's `Some`, then `waker` is exclusively owned by the
    ///   enclosing `Waiter` and can be accessed without locking.
    notification: AtomicNotification,
    /// Should not be `Unpin`.
    _p: PhantomPinned,
}
impl Waiter {
    fn new() -> Waiter {
        panic!("STUB: not implemented");
    }
}
generate_addr_of_methods! {
    impl <> Waiter { unsafe fn addr_of_pointers(self : NonNull < Self >) -> NonNull <
    linked_list::Pointers < Waiter >> { & self.pointers } }
}
const NOTIFICATION_NONE: usize = 0b000;
const NOTIFICATION_ONE: usize = 0b001;
const NOTIFICATION_LAST: usize = 0b101;
const NOTIFICATION_ALL: usize = 0b010;
/// Notification for a `Waiter`.
/// This struct is equivalent to `Option<Notification>`, but uses
/// `AtomicUsize` inside for atomic operations.
#[derive(Debug)]
struct AtomicNotification(AtomicUsize);
impl AtomicNotification {
    fn none() -> Self {
        panic!("STUB: not implemented");
    }
    /// Store-release a notification.
    /// This method should be called exactly once.
    fn store_release(&self, notification: Notification) {
        panic!("STUB: not implemented");
    }
    fn load(&self, ordering: Ordering) -> Option<Notification> {
        panic!("STUB: not implemented");
    }
    /// Clears the notification.
    /// This method is used by a `Notified` future to consume the
    /// notification. It uses relaxed ordering and should be only
    /// used once the atomic notification is no longer shared.
    fn clear(&self) {
        panic!("STUB: not implemented");
    }
}
#[derive(Debug, PartialEq, Eq)]
#[repr(usize)]
enum NotifyOneStrategy {
    Fifo,
    Lifo,
}
#[derive(Debug, PartialEq, Eq)]
#[repr(usize)]
enum Notification {
    One(NotifyOneStrategy),
    All,
}
/// List used in `Notify::notify_waiters`. It wraps a guarded linked list
/// and gates the access to it on `notify.waiters` mutex. It also empties
/// the list on drop.
struct NotifyWaitersList<'a> {
    list: GuardedLinkedList<Waiter>,
    is_empty: bool,
    notify: &'a Notify,
}
impl<'a> NotifyWaitersList<'a> {
    fn new(
        unguarded_list: LinkedList<Waiter>,
        guard: Pin<&'a Waiter>,
        notify: &'a Notify,
    ) -> NotifyWaitersList<'a> {
        panic!("STUB: not implemented");
    }
    /// Removes the last element from the guarded list. Modifying this list
    /// requires an exclusive access to the main list in `Notify`.
    fn pop_back_locked(
        &mut self,
        _waiters: &mut LinkedList<Waiter>,
    ) -> Option<NonNull<Waiter>> {
        panic!("STUB: not implemented");
    }
}
impl Drop for NotifyWaitersList<'_> {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
/// Future returned from [`Notify::notified()`].
///
/// This future is fused, so once it has completed, any future calls to poll
/// will immediately return `Poll::Ready`.
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Notified<'a> {
    /// The `Notify` being received on.
    notify: &'a Notify,
    /// The current state of the receiving process.
    state: State,
    /// Number of calls to `notify_waiters` at the time of creation.
    notify_waiters_calls: usize,
    /// Entry in the waiter `LinkedList`.
    waiter: Waiter,
}
unsafe impl<'a> Send for Notified<'a> {}
unsafe impl<'a> Sync for Notified<'a> {}
/// Future returned from [`Notify::notified_owned()`].
///
/// This future is fused, so once it has completed, any future calls to poll
/// will immediately return `Poll::Ready`.
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct OwnedNotified {
    /// The `Notify` being received on.
    notify: Arc<Notify>,
    /// The current state of the receiving process.
    state: State,
    /// Number of calls to `notify_waiters` at the time of creation.
    notify_waiters_calls: usize,
    /// Entry in the waiter `LinkedList`.
    waiter: Waiter,
}
unsafe impl Sync for OwnedNotified {}
/// A custom `project` implementation is used in place of `pin-project-lite`
/// as a custom drop for [`Notified`] and [`OwnedNotified`] implementation
/// is needed.
struct NotifiedProject<'a> {
    notify: &'a Notify,
    state: &'a mut State,
    notify_waiters_calls: &'a usize,
    waiter: &'a Waiter,
}
#[derive(Debug)]
enum State {
    Init,
    Waiting,
    Done,
}
const NOTIFY_WAITERS_SHIFT: usize = 2;
const STATE_MASK: usize = (1 << NOTIFY_WAITERS_SHIFT) - 1;
const NOTIFY_WAITERS_CALLS_MASK: usize = !STATE_MASK;
/// Initial "idle" state.
const EMPTY: usize = 0;
/// One or more threads are currently waiting to be notified.
const WAITING: usize = 1;
/// Pending notification.
const NOTIFIED: usize = 2;
fn set_state(data: usize, state: usize) -> usize {
    panic!("STUB: not implemented");
}
fn get_state(data: usize) -> usize {
    panic!("STUB: not implemented");
}
fn get_num_notify_waiters_calls(data: usize) -> usize {
    panic!("STUB: not implemented");
}
fn inc_num_notify_waiters_calls(data: usize) -> usize {
    panic!("STUB: not implemented");
}
fn atomic_inc_num_notify_waiters_calls(data: &AtomicUsize) {
    panic!("STUB: not implemented");
}
impl Notify {
    /// Create a new `Notify`, initialized without a permit.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::Notify;
    ///
    /// let notify = Notify::new();
    /// ```
    pub fn new() -> Notify {
        panic!("STUB: not implemented");
    }
    /// Create a new `Notify`, initialized without a permit.
    ///
    /// When using the `tracing` [unstable feature], a `Notify` created with
    /// `const_new` will not be instrumented. As such, it will not be visible
    /// in [`tokio-console`]. Instead, [`Notify::new`] should be used to create
    /// an instrumented object if that is needed.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::Notify;
    ///
    /// static NOTIFY: Notify = Notify::const_new();
    /// ```
    ///
    /// [`tokio-console`]: https://github.com/tokio-rs/console
    /// [unstable feature]: crate#unstable-features
    #[cfg(not(all(loom, test)))]
    pub const fn const_new() -> Notify {
        Notify {
            state: AtomicUsize::new(0),
            waiters: Mutex::const_new(LinkedList::new()),
        }
    }
    /// Wait for a notification.
    ///
    /// Equivalent to:
    ///
    /// ```ignore
    /// async fn notified(&self);
    /// ```
    ///
    /// Each `Notify` value holds a single permit. If a permit is available from
    /// an earlier call to [`notify_one()`], then `notified().await` will complete
    /// immediately, consuming that permit. Otherwise, `notified().await` waits
    /// for a permit to be made available by the next call to `notify_one()`.
    ///
    /// The `Notified` future is not guaranteed to receive wakeups from calls to
    /// `notify_one()` if it has not yet been polled. See the documentation for
    /// [`Notified::enable()`] for more details.
    ///
    /// The `Notified` future is guaranteed to receive wakeups from
    /// `notify_waiters()` as soon as it has been created, even if it has not
    /// yet been polled.
    ///
    /// [`notify_one()`]: Notify::notify_one
    /// [`Notified::enable()`]: Notified::enable
    ///
    /// # Cancel safety
    ///
    /// This method uses a queue to fairly distribute notifications in the order
    /// they were requested. Cancelling a call to `notified` makes you lose your
    /// place in the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::Notify;
    /// use std::sync::Arc;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let notify = Arc::new(Notify::new());
    /// let notify2 = notify.clone();
    ///
    /// tokio::spawn(async move {
    ///     notify2.notified().await;
    ///     println!("received notification");
    /// });
    ///
    /// println!("sending notification");
    /// notify.notify_one();
    /// # }
    /// ```
    pub fn notified(&self) -> Notified<'_> {
        panic!("STUB: not implemented");
    }
    /// Wait for a notification with an owned `Future`.
    ///
    /// Unlike [`Self::notified`] which returns a future tied to the `Notify`'s
    /// lifetime, `notified_owned` creates a self-contained future that owns its
    /// notification state, making it safe to move between threads.
    ///
    /// See [`Self::notified`] for more details.
    ///
    /// # Cancel safety
    ///
    /// This method uses a queue to fairly distribute notifications in the order
    /// they were requested. Cancelling a call to `notified_owned` makes you lose your
    /// place in the queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::sync::Arc;
    /// use tokio::sync::Notify;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let notify = Arc::new(Notify::new());
    ///
    /// for _ in 0..10 {
    ///     let notified = notify.clone().notified_owned();
    ///     tokio::spawn(async move {
    ///         notified.await;
    ///         println!("received notification");
    ///     });
    /// }
    ///
    /// println!("sending notification");
    /// notify.notify_waiters();
    /// # }
    /// ```
    pub fn notified_owned(self: Arc<Self>) -> OwnedNotified {
        panic!("STUB: not implemented");
    }
    /// Notifies the first waiting task.
    ///
    /// If a task is currently waiting, that task is notified. Otherwise, a
    /// permit is stored in this `Notify` value and the **next** call to
    /// [`notified().await`] will complete immediately consuming the permit made
    /// available by this call to `notify_one()`.
    ///
    /// At most one permit may be stored by `Notify`. Many sequential calls to
    /// `notify_one` will result in a single permit being stored. The next call to
    /// `notified().await` will complete immediately, but the one after that
    /// will wait.
    ///
    /// [`notified().await`]: Notify::notified()
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::Notify;
    /// use std::sync::Arc;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let notify = Arc::new(Notify::new());
    /// let notify2 = notify.clone();
    ///
    /// tokio::spawn(async move {
    ///     notify2.notified().await;
    ///     println!("received notification");
    /// });
    ///
    /// println!("sending notification");
    /// notify.notify_one();
    /// # }
    /// ```
    #[cfg_attr(docsrs, doc(alias = "notify"))]
    pub fn notify_one(&self) {
        panic!("STUB: not implemented");
    }
    /// Notifies the last waiting task.
    ///
    /// This function behaves similar to `notify_one`. The only difference is that it wakes
    /// the most recently added waiter instead of the oldest waiter.
    ///
    /// Check the [`notify_one()`] documentation for more info and
    /// examples.
    ///
    /// [`notify_one()`]: Notify::notify_one
    pub fn notify_last(&self) {
        panic!("STUB: not implemented");
    }
    fn notify_with_strategy(&self, strategy: NotifyOneStrategy) {
        panic!("STUB: not implemented");
    }
    /// Notifies all waiting tasks.
    ///
    /// If a task is currently waiting, that task is notified. Unlike with
    /// `notify_one()`, no permit is stored to be used by the next call to
    /// `notified().await`. The purpose of this method is to notify all
    /// already registered waiters. Registering for notification is done by
    /// acquiring an instance of the `Notified` future via calling `notified()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::Notify;
    /// use std::sync::Arc;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let notify = Arc::new(Notify::new());
    /// let notify2 = notify.clone();
    ///
    /// let notified1 = notify.notified();
    /// let notified2 = notify.notified();
    ///
    /// let handle = tokio::spawn(async move {
    ///     println!("sending notifications");
    ///     notify2.notify_waiters();
    /// });
    ///
    /// notified1.await;
    /// notified2.await;
    /// println!("received notifications");
    /// # }
    /// ```
    pub fn notify_waiters(&self) {
        panic!("STUB: not implemented");
    }
    fn inner_notify_waiters<'a>(
        &'a self,
        curr: usize,
        mut waiters: crate::loom::sync::MutexGuard<'a, LinkedList<Waiter>>,
    ) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn lock_waiter_list(&self) -> NotifyGuard<'_> {
        panic!("STUB: not implemented");
    }
}
impl Default for Notify {
    fn default() -> Notify {
        panic!("STUB: not implemented");
    }
}
impl UnwindSafe for Notify {}
impl RefUnwindSafe for Notify {}
fn notify_locked(
    waiters: &mut LinkedList<Waiter>,
    state: &AtomicUsize,
    curr: usize,
    strategy: NotifyOneStrategy,
) -> Option<Waker> {
    panic!("STUB: not implemented");
}
impl Notified<'_> {
    /// Adds this future to the list of futures that are ready to receive
    /// wakeups from calls to [`notify_one`].
    ///
    /// Polling the future also adds it to the list, so this method should only
    /// be used if you want to add the future to the list before the first call
    /// to `poll`. (In fact, this method is equivalent to calling `poll` except
    /// that no `Waker` is registered.)
    ///
    /// This has no effect on notifications sent using [`notify_waiters`], which
    /// are received as long as they happen after the creation of the `Notified`
    /// regardless of whether `enable` or `poll` has been called.
    ///
    /// This method returns true if the `Notified` is ready. This happens in the
    /// following situations:
    ///
    ///  1. The `notify_waiters` method was called between the creation of the
    ///     `Notified` and the call to this method.
    ///  2. This is the first call to `enable` or `poll` on this future, and the
    ///     `Notify` was holding a permit from a previous call to `notify_one`.
    ///     The call consumes the permit in that case.
    ///  3. The future has previously been enabled or polled, and it has since
    ///     then been marked ready by either consuming a permit from the
    ///     `Notify`, or by a call to `notify_one` or `notify_waiters` that
    ///     removed it from the list of futures ready to receive wakeups.
    ///
    /// If this method returns true, any future calls to poll on the same future
    /// will immediately return `Poll::Ready`.
    ///
    /// # Examples
    ///
    /// Unbound multi-producer multi-consumer (mpmc) channel.
    ///
    /// The call to `enable` is important because otherwise if you have two
    /// calls to `recv` and two calls to `send` in parallel, the following could
    /// happen:
    ///
    ///  1. Both calls to `try_recv` return `None`.
    ///  2. Both new elements are added to the vector.
    ///  3. The `notify_one` method is called twice, adding only a single
    ///     permit to the `Notify`.
    ///  4. Both calls to `recv` reach the `Notified` future. One of them
    ///     consumes the permit, and the other sleeps forever.
    ///
    /// By adding the `Notified` futures to the list by calling `enable` before
    /// `try_recv`, the `notify_one` calls in step three would remove the
    /// futures from the list and mark them notified instead of adding a permit
    /// to the `Notify`. This ensures that both futures are woken.
    ///
    /// ```
    /// use tokio::sync::Notify;
    ///
    /// use std::collections::VecDeque;
    /// use std::sync::Mutex;
    ///
    /// struct Channel<T> {
    ///     messages: Mutex<VecDeque<T>>,
    ///     notify_on_sent: Notify,
    /// }
    ///
    /// impl<T> Channel<T> {
    ///     pub fn send(&self, msg: T) {
    ///         let mut locked_queue = self.messages.lock().unwrap();
    ///         locked_queue.push_back(msg);
    ///         drop(locked_queue);
    ///
    ///         // Send a notification to one of the calls currently
    ///         // waiting in a call to `recv`.
    ///         self.notify_on_sent.notify_one();
    ///     }
    ///
    ///     pub fn try_recv(&self) -> Option<T> {
    ///         let mut locked_queue = self.messages.lock().unwrap();
    ///         locked_queue.pop_front()
    ///     }
    ///
    ///     pub async fn recv(&self) -> T {
    ///         let future = self.notify_on_sent.notified();
    ///         tokio::pin!(future);
    ///
    ///         loop {
    ///             // Make sure that no wakeup is lost if we get
    ///             // `None` from `try_recv`.
    ///             future.as_mut().enable();
    ///
    ///             if let Some(msg) = self.try_recv() {
    ///                 return msg;
    ///             }
    ///
    ///             // Wait for a call to `notify_one`.
    ///             //
    ///             // This uses `.as_mut()` to avoid consuming the future,
    ///             // which lets us call `Pin::set` below.
    ///             future.as_mut().await;
    ///
    ///             // Reset the future in case another call to
    ///             // `try_recv` got the message before us.
    ///             future.set(self.notify_on_sent.notified());
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// [`notify_one`]: Notify::notify_one()
    /// [`notify_waiters`]: Notify::notify_waiters()
    pub fn enable(self: Pin<&mut Self>) -> bool {
        panic!("STUB: not implemented");
    }
    fn project(self: Pin<&mut Self>) -> NotifiedProject<'_> {
        panic!("STUB: not implemented");
    }
    fn poll_notified(self: Pin<&mut Self>, waker: Option<&Waker>) -> Poll<()> {
        panic!("STUB: not implemented");
    }
}
impl Future for Notified<'_> {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        panic!("STUB: not implemented");
    }
}
impl Drop for Notified<'_> {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
impl OwnedNotified {
    /// Adds this future to the list of futures that are ready to receive
    /// wakeups from calls to [`notify_one`].
    ///
    /// See [`Notified::enable`] for more details.
    ///
    /// [`notify_one`]: Notify::notify_one()
    pub fn enable(self: Pin<&mut Self>) -> bool {
        panic!("STUB: not implemented");
    }
    /// A custom `project` implementation is used in place of `pin-project-lite`
    /// as a custom drop implementation is needed.
    fn project(self: Pin<&mut Self>) -> NotifiedProject<'_> {
        panic!("STUB: not implemented");
    }
    fn poll_notified(self: Pin<&mut Self>, waker: Option<&Waker>) -> Poll<()> {
        panic!("STUB: not implemented");
    }
}
impl Future for OwnedNotified {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        panic!("STUB: not implemented");
    }
}
impl Drop for OwnedNotified {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
impl NotifiedProject<'_> {
    fn poll_notified(self, waker: Option<&Waker>) -> Poll<()> {
        panic!("STUB: not implemented");
    }
    fn drop_notified(self) {
        panic!("STUB: not implemented");
    }
}
/// # Safety
///
/// `Waiter` is forced to be !Unpin.
unsafe impl linked_list::Link for Waiter {
    type Handle = NonNull<Waiter>;
    type Target = Waiter;
    fn as_raw(handle: &NonNull<Waiter>) -> NonNull<Waiter> {
        panic!("STUB: not implemented");
    }
    unsafe fn from_raw(ptr: NonNull<Waiter>) -> NonNull<Waiter> {
        panic!("STUB: not implemented");
    }
    unsafe fn pointers(
        target: NonNull<Waiter>,
    ) -> NonNull<linked_list::Pointers<Waiter>> {
        panic!("STUB: not implemented");
    }
}
fn is_unpin<T: Unpin>() {
    panic!("STUB: not implemented");
}
/// A guard that provides exclusive access to a `Notify`'s internal
/// waiters list.
///
/// While this guard is held, the `Notify` instance's waiter list is locked.
pub(crate) struct NotifyGuard<'a> {
    guarded_notify: &'a Notify,
    guarded_waiters: crate::loom::sync::MutexGuard<'a, LinkedList<Waiter>>,
    current_state: usize,
}
impl NotifyGuard<'_> {
    pub(crate) fn notify_waiters(self) {
        panic!("STUB: not implemented");
    }
}
