//! A multi-producer, multi-consumer broadcast queue. Each sent value is seen by
//! all consumers.
//!
//! A [`Sender`] is used to broadcast values to **all** connected [`Receiver`]
//! values. [`Sender`] handles are clone-able, allowing concurrent send and
//! receive actions. [`Sender`] and [`Receiver`] are both `Send` and `Sync` as
//! long as `T` is `Send`.
//!
//! When a value is sent, **all** [`Receiver`] handles are notified and will
//! receive the value. The value is stored once inside the channel and cloned on
//! demand for each receiver. Once all receivers have received a clone of the
//! value, the value is released from the channel.
//!
//! A channel is created by calling [`channel`], specifying the maximum number
//! of messages the channel can retain at any given time.
//!
//! New [`Receiver`] handles are created by calling [`Sender::subscribe`]. The
//! returned [`Receiver`] will receive values sent **after** the call to
//! `subscribe`.
//!
//! This channel is also suitable for the single-producer multi-consumer
//! use-case, where a single sender broadcasts values to many receivers.
//!
//! ## Lagging
//!
//! As sent messages must be retained until **all** [`Receiver`] handles receive
//! a clone, broadcast channels are susceptible to the "slow receiver" problem.
//! In this case, all but one receiver are able to receive values at the rate
//! they are sent. Because one receiver is stalled, the channel starts to fill
//! up.
//!
//! This broadcast channel implementation handles this case by setting a hard
//! upper bound on the number of values the channel may retain at any given
//! time. This upper bound is passed to the [`channel`] function as an argument.
//!
//! If a value is sent when the channel is at capacity, the oldest value
//! currently held by the channel is released. This frees up space for the new
//! value. Any receiver that has not yet seen the released value will return
//! [`RecvError::Lagged`] the next time [`recv`] is called.
//!
//! Once [`RecvError::Lagged`] is returned, the lagging receiver's position is
//! updated to the oldest value contained by the channel. The next call to
//! [`recv`] will return this value.
//!
//! This behavior enables a receiver to detect when it has lagged so far behind
//! that data has been dropped. The caller may decide how to respond to this:
//! either by aborting its task or by tolerating lost messages and resuming
//! consumption of the channel.
//!
//! ## Closing
//!
//! When **all** [`Sender`] handles have been dropped, no new values may be
//! sent. At this point, the channel is "closed". Once a receiver has received
//! all values retained by the channel, the next call to [`recv`] will return
//! with [`RecvError::Closed`].
//!
//! When a [`Receiver`] handle is dropped, any messages not read by the receiver
//! will be marked as read. If this receiver was the only one not to have read
//! that message, the message will be dropped at this point.
//!
//! [`Sender`]: crate::sync::broadcast::Sender
//! [`Sender::subscribe`]: crate::sync::broadcast::Sender::subscribe
//! [`Receiver`]: crate::sync::broadcast::Receiver
//! [`channel`]: crate::sync::broadcast::channel
//! [`RecvError::Lagged`]: crate::sync::broadcast::error::RecvError::Lagged
//! [`RecvError::Closed`]: crate::sync::broadcast::error::RecvError::Closed
//! [`recv`]: crate::sync::broadcast::Receiver::recv
//!
//! # Examples
//!
//! Basic usage
//!
//! ```
//! use tokio::sync::broadcast;
//!
//! # #[tokio::main(flavor = "current_thread")]
//! # async fn main() {
//! let (tx, mut rx1) = broadcast::channel(16);
//! let mut rx2 = tx.subscribe();
//!
//! tokio::spawn(async move {
//!     assert_eq!(rx1.recv().await.unwrap(), 10);
//!     assert_eq!(rx1.recv().await.unwrap(), 20);
//! });
//!
//! tokio::spawn(async move {
//!     assert_eq!(rx2.recv().await.unwrap(), 10);
//!     assert_eq!(rx2.recv().await.unwrap(), 20);
//! });
//!
//! tx.send(10).unwrap();
//! tx.send(20).unwrap();
//! # }
//! ```
//!
//! Handling lag
//!
//! ```
//! use tokio::sync::broadcast;
//!
//! # #[tokio::main(flavor = "current_thread")]
//! # async fn main() {
//! let (tx, mut rx) = broadcast::channel(2);
//!
//! tx.send(10).unwrap();
//! tx.send(20).unwrap();
//! tx.send(30).unwrap();
//!
//! // The receiver lagged behind
//! assert!(rx.recv().await.is_err());
//!
//! // At this point, we can abort or continue with lost messages
//!
//! assert_eq!(20, rx.recv().await.unwrap());
//! assert_eq!(30, rx.recv().await.unwrap());
//! # }
//! ```
use crate::loom::cell::UnsafeCell;
use crate::loom::sync::atomic::{AtomicBool, AtomicUsize};
use crate::loom::sync::{Arc, Mutex, MutexGuard};
use crate::task::coop::cooperative;
use crate::util::linked_list::{self, GuardedLinkedList, LinkedList};
use crate::util::WakeList;
use std::fmt;
use std::future::Future;
use std::marker::PhantomPinned;
use std::pin::Pin;
use std::ptr::NonNull;
use std::sync::atomic::Ordering::{AcqRel, Acquire, Relaxed, Release, SeqCst};
use std::task::{ready, Context, Poll, Waker};
/// Sending-half of the [`broadcast`] channel.
///
/// May be used from many threads. Messages can be sent with
/// [`send`][Sender::send].
///
/// # Examples
///
/// ```
/// use tokio::sync::broadcast;
///
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() {
/// let (tx, mut rx1) = broadcast::channel(16);
/// let mut rx2 = tx.subscribe();
///
/// tokio::spawn(async move {
///     assert_eq!(rx1.recv().await.unwrap(), 10);
///     assert_eq!(rx1.recv().await.unwrap(), 20);
/// });
///
/// tokio::spawn(async move {
///     assert_eq!(rx2.recv().await.unwrap(), 10);
///     assert_eq!(rx2.recv().await.unwrap(), 20);
/// });
///
/// tx.send(10).unwrap();
/// tx.send(20).unwrap();
/// # }
/// ```
///
/// [`broadcast`]: crate::sync::broadcast
pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}
/// A sender that does not prevent the channel from being closed.
///
/// If all [`Sender`] instances of a channel were dropped and only `WeakSender`
/// instances remain, the channel is closed.
///
/// In order to send messages, the `WeakSender` needs to be upgraded using
/// [`WeakSender::upgrade`], which returns `Option<Sender>`. It returns `None`
/// if all `Sender`s have been dropped, and otherwise it returns a `Sender`.
///
/// [`Sender`]: Sender
/// [`WeakSender::upgrade`]: WeakSender::upgrade
///
/// # Examples
///
/// ```
/// use tokio::sync::broadcast::channel;
///
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() {
/// let (tx, _rx) = channel::<i32>(15);
/// let tx_weak = tx.downgrade();
///
/// // Upgrading will succeed because `tx` still exists.
/// assert!(tx_weak.upgrade().is_some());
///
/// // If we drop `tx`, then it will fail.
/// drop(tx);
/// assert!(tx_weak.clone().upgrade().is_none());
/// # }
/// ```
pub struct WeakSender<T> {
    shared: Arc<Shared<T>>,
}
/// Receiving-half of the [`broadcast`] channel.
///
/// Must not be used concurrently. Messages may be retrieved using
/// [`recv`][Receiver::recv].
///
/// To turn this receiver into a `Stream`, you can use the [`BroadcastStream`]
/// wrapper.
///
/// [`BroadcastStream`]: https://docs.rs/tokio-stream/0.1/tokio_stream/wrappers/struct.BroadcastStream.html
///
/// # Examples
///
/// ```
/// use tokio::sync::broadcast;
///
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() {
/// let (tx, mut rx1) = broadcast::channel(16);
/// let mut rx2 = tx.subscribe();
///
/// tokio::spawn(async move {
///     assert_eq!(rx1.recv().await.unwrap(), 10);
///     assert_eq!(rx1.recv().await.unwrap(), 20);
/// });
///
/// tokio::spawn(async move {
///     assert_eq!(rx2.recv().await.unwrap(), 10);
///     assert_eq!(rx2.recv().await.unwrap(), 20);
/// });
///
/// tx.send(10).unwrap();
/// tx.send(20).unwrap();
/// # }
/// ```
///
/// [`broadcast`]: crate::sync::broadcast
pub struct Receiver<T> {
    /// State shared with all receivers and senders.
    shared: Arc<Shared<T>>,
    /// Next position to read from
    next: u64,
}
pub mod error {
    //! Broadcast error types
    use std::fmt;
    /// Error returned by the [`send`] function on a [`Sender`].
    ///
    /// A **send** operation can only fail if there are no active receivers,
    /// implying that the message could never be received. The error contains the
    /// message being sent as a payload so it can be recovered.
    ///
    /// [`send`]: crate::sync::broadcast::Sender::send
    /// [`Sender`]: crate::sync::broadcast::Sender
    #[derive(Debug)]
    pub struct SendError<T>(pub T);
    impl<T> fmt::Display for SendError<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            panic!("STUB: not implemented");
        }
    }
    impl<T: fmt::Debug> std::error::Error for SendError<T> {}
    /// An error returned from the [`recv`] function on a [`Receiver`].
    ///
    /// [`recv`]: crate::sync::broadcast::Receiver::recv
    /// [`Receiver`]: crate::sync::broadcast::Receiver
    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum RecvError {
        /// There are no more active senders implying no further messages will ever
        /// be sent.
        Closed,
        /// The receiver lagged too far behind. Attempting to receive again will
        /// return the oldest message still retained by the channel.
        ///
        /// Includes the number of skipped messages.
        Lagged(u64),
    }
    impl fmt::Display for RecvError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            panic!("STUB: not implemented");
        }
    }
    impl std::error::Error for RecvError {}
    /// An error returned from the [`try_recv`] function on a [`Receiver`].
    ///
    /// [`try_recv`]: crate::sync::broadcast::Receiver::try_recv
    /// [`Receiver`]: crate::sync::broadcast::Receiver
    #[derive(Debug, PartialEq, Eq, Clone)]
    pub enum TryRecvError {
        /// The channel is currently empty. There are still active
        /// [`Sender`] handles, so data may yet become available.
        ///
        /// [`Sender`]: crate::sync::broadcast::Sender
        Empty,
        /// There are no more active senders implying no further messages will ever
        /// be sent.
        Closed,
        /// The receiver lagged too far behind and has been forcibly disconnected.
        /// Attempting to receive again will return the oldest message still
        /// retained by the channel.
        ///
        /// Includes the number of skipped messages.
        Lagged(u64),
    }
    impl fmt::Display for TryRecvError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            panic!("STUB: not implemented");
        }
    }
    impl std::error::Error for TryRecvError {}
}
use self::error::{RecvError, SendError, TryRecvError};
use super::Notify;
/// Data shared between senders and receivers.
struct Shared<T> {
    /// slots in the channel.
    buffer: Box<[Mutex<Slot<T>>]>,
    /// Mask a position -> index.
    mask: usize,
    /// Tail of the queue. Includes the rx wait list.
    tail: Mutex<Tail>,
    /// Number of outstanding Sender handles.
    num_tx: AtomicUsize,
    /// Number of outstanding weak Sender handles.
    num_weak_tx: AtomicUsize,
    /// Notify when the last subscribed [`Receiver`] drops.
    notify_last_rx_drop: Notify,
}
/// Next position to write a value.
struct Tail {
    /// Next position to write to.
    pos: u64,
    /// Number of active receivers.
    rx_cnt: usize,
    /// True if the channel is closed.
    closed: bool,
    /// Receivers waiting for a value.
    waiters: LinkedList<Waiter>,
}
/// Slot in the buffer.
struct Slot<T> {
    /// Remaining number of receivers that are expected to see this value.
    ///
    /// When this goes to zero, the value is released.
    ///
    /// An atomic is used as it is mutated concurrently with the slot read lock
    /// acquired.
    rem: AtomicUsize,
    /// Uniquely identifies the `send` stored in the slot.
    pos: u64,
    /// The value being broadcast.
    ///
    /// The value is set by `send` when the write lock is held. When a reader
    /// drops, `rem` is decremented. When it hits zero, the value is dropped.
    val: Option<T>,
}
/// An entry in the wait queue.
struct Waiter {
    /// True if queued.
    queued: AtomicBool,
    /// Task waiting on the broadcast channel.
    waker: Option<Waker>,
    /// Intrusive linked-list pointers.
    pointers: linked_list::Pointers<Waiter>,
    /// Should not be `Unpin`.
    _p: PhantomPinned,
}
impl Waiter {
    fn new() -> Self {
        panic!("STUB: not implemented");
    }
}
generate_addr_of_methods! {
    impl <> Waiter { unsafe fn addr_of_pointers(self : NonNull < Self >) -> NonNull <
    linked_list::Pointers < Waiter >> { & self.pointers } }
}
struct RecvGuard<'a, T> {
    slot: MutexGuard<'a, Slot<T>>,
}
/// Receive a value future.
struct Recv<'a, T> {
    /// Receiver being waited on.
    receiver: &'a mut Receiver<T>,
    /// Entry in the waiter `LinkedList`.
    waiter: WaiterCell,
}
struct WaiterCell(UnsafeCell<Waiter>);
unsafe impl Send for WaiterCell {}
unsafe impl Sync for WaiterCell {}
/// Max number of receivers. Reserve space to lock.
const MAX_RECEIVERS: usize = usize::MAX >> 2;
/// Create a bounded, multi-producer, multi-consumer channel where each sent
/// value is broadcasted to all active receivers.
///
/// **Note:** The actual capacity may be greater than the provided `capacity`.
///
/// All data sent on [`Sender`] will become available on every active
/// [`Receiver`] in the same order as it was sent.
///
/// The `Sender` can be cloned to `send` to the same channel from multiple
/// points in the process or it can be used concurrently from an `Arc`. New
/// `Receiver` handles are created by calling [`Sender::subscribe`].
///
/// If all [`Receiver`] handles are dropped, the `send` method will return a
/// [`SendError`]. Similarly, if all [`Sender`] handles are dropped, the [`recv`]
/// method will return a [`RecvError`].
///
/// [`Sender`]: crate::sync::broadcast::Sender
/// [`Sender::subscribe`]: crate::sync::broadcast::Sender::subscribe
/// [`Receiver`]: crate::sync::broadcast::Receiver
/// [`recv`]: crate::sync::broadcast::Receiver::recv
/// [`SendError`]: crate::sync::broadcast::error::SendError
/// [`RecvError`]: crate::sync::broadcast::error::RecvError
///
/// # Examples
///
/// ```
/// use tokio::sync::broadcast;
///
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() {
/// let (tx, mut rx1) = broadcast::channel(16);
/// let mut rx2 = tx.subscribe();
///
/// tokio::spawn(async move {
///     assert_eq!(rx1.recv().await.unwrap(), 10);
///     assert_eq!(rx1.recv().await.unwrap(), 20);
/// });
///
/// tokio::spawn(async move {
///     assert_eq!(rx2.recv().await.unwrap(), 10);
///     assert_eq!(rx2.recv().await.unwrap(), 20);
/// });
///
/// tx.send(10).unwrap();
/// tx.send(20).unwrap();
/// # }
/// ```
///
/// # Panics
///
/// This will panic if `capacity` is equal to `0`.
///
/// This pre-allocates space for `capacity` messages. Allocation failure may result in a panic or
/// [an allocation error](std::alloc::handle_alloc_error).
#[track_caller]
pub fn channel<T: Clone>(capacity: usize) -> (Sender<T>, Receiver<T>) {
    panic!("STUB: not implemented");
}
impl<T> Sender<T> {
    /// Creates the sending-half of the [`broadcast`] channel.
    ///
    /// See the documentation of [`broadcast::channel`] for more information on this method.
    ///
    /// [`broadcast`]: crate::sync::broadcast
    /// [`broadcast::channel`]: crate::sync::broadcast::channel
    #[track_caller]
    pub fn new(capacity: usize) -> Self {
        panic!("STUB: not implemented");
    }
    /// Creates the sending-half of the [`broadcast`](self) channel, and provide the receiver
    /// count.
    ///
    /// See the documentation of [`broadcast::channel`](self::channel) for more errors when
    /// calling this function.
    ///
    /// # Safety:
    ///
    /// The caller must ensure that the amount of receivers for this Sender is correct before
    /// the channel functionalities are used, the count is zero by default, as this function
    /// does not create any receivers by itself.
    #[track_caller]
    unsafe fn new_with_receiver_count(
        receiver_count: usize,
        mut capacity: usize,
    ) -> Self {
        panic!("STUB: not implemented");
    }
    /// Attempts to send a value to all active [`Receiver`] handles, returning
    /// it back if it could not be sent.
    ///
    /// A successful send occurs when there is at least one active [`Receiver`]
    /// handle. An unsuccessful send would be one where all associated
    /// [`Receiver`] handles have already been dropped.
    ///
    /// # Return
    ///
    /// On success, the number of subscribed [`Receiver`] handles is returned.
    /// This does not mean that this number of receivers will see the message as
    /// a receiver may drop or lag ([see lagging](self#lagging)) before receiving
    /// the message.
    ///
    /// # Note
    ///
    /// A return value of `Ok` **does not** mean that the sent value will be
    /// observed by all or any of the active [`Receiver`] handles. [`Receiver`]
    /// handles may be dropped before receiving the sent message.
    ///
    /// A return value of `Err` **does not** mean that future calls to `send`
    /// will fail. New [`Receiver`] handles may be created by calling
    /// [`subscribe`].
    ///
    /// [`Receiver`]: crate::sync::broadcast::Receiver
    /// [`subscribe`]: crate::sync::broadcast::Sender::subscribe
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::broadcast;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let (tx, mut rx1) = broadcast::channel(16);
    /// let mut rx2 = tx.subscribe();
    ///
    /// tokio::spawn(async move {
    ///     assert_eq!(rx1.recv().await.unwrap(), 10);
    ///     assert_eq!(rx1.recv().await.unwrap(), 20);
    /// });
    ///
    /// tokio::spawn(async move {
    ///     assert_eq!(rx2.recv().await.unwrap(), 10);
    ///     assert_eq!(rx2.recv().await.unwrap(), 20);
    /// });
    ///
    /// tx.send(10).unwrap();
    /// tx.send(20).unwrap();
    /// # }
    /// ```
    pub fn send(&self, value: T) -> Result<usize, SendError<T>> {
        panic!("STUB: not implemented");
    }
    /// Creates a new [`Receiver`] handle that will receive values sent **after**
    /// this call to `subscribe`.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::broadcast;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let (tx, _rx) = broadcast::channel(16);
    ///
    /// // Will not be seen
    /// tx.send(10).unwrap();
    ///
    /// let mut rx = tx.subscribe();
    ///
    /// tx.send(20).unwrap();
    ///
    /// let value = rx.recv().await.unwrap();
    /// assert_eq!(20, value);
    /// # }
    /// ```
    pub fn subscribe(&self) -> Receiver<T> {
        panic!("STUB: not implemented");
    }
    /// Converts the `Sender` to a [`WeakSender`] that does not count
    /// towards RAII semantics, i.e. if all `Sender` instances of the
    /// channel were dropped and only `WeakSender` instances remain,
    /// the channel is closed.
    #[must_use = "Downgrade creates a WeakSender without destroying the original non-weak sender."]
    pub fn downgrade(&self) -> WeakSender<T> {
        panic!("STUB: not implemented");
    }
    /// Returns the number of queued values.
    ///
    /// A value is queued until it has either been seen by all receivers that were alive at the time
    /// it was sent, or has been evicted from the queue by subsequent sends that exceeded the
    /// queue's capacity.
    ///
    /// # Note
    ///
    /// In contrast to [`Receiver::len`], this method only reports queued values and not values that
    /// have been evicted from the queue before being seen by all receivers.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::broadcast;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let (tx, mut rx1) = broadcast::channel(16);
    /// let mut rx2 = tx.subscribe();
    ///
    /// tx.send(10).unwrap();
    /// tx.send(20).unwrap();
    /// tx.send(30).unwrap();
    ///
    /// assert_eq!(tx.len(), 3);
    ///
    /// rx1.recv().await.unwrap();
    ///
    /// // The len is still 3 since rx2 hasn't seen the first value yet.
    /// assert_eq!(tx.len(), 3);
    ///
    /// rx2.recv().await.unwrap();
    ///
    /// assert_eq!(tx.len(), 2);
    /// # }
    /// ```
    pub fn len(&self) -> usize {
        panic!("STUB: not implemented");
    }
    /// Returns true if there are no queued values.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::broadcast;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let (tx, mut rx1) = broadcast::channel(16);
    /// let mut rx2 = tx.subscribe();
    ///
    /// assert!(tx.is_empty());
    ///
    /// tx.send(10).unwrap();
    ///
    /// assert!(!tx.is_empty());
    ///
    /// rx1.recv().await.unwrap();
    ///
    /// // The queue is still not empty since rx2 hasn't seen the value.
    /// assert!(!tx.is_empty());
    ///
    /// rx2.recv().await.unwrap();
    ///
    /// assert!(tx.is_empty());
    /// # }
    /// ```
    pub fn is_empty(&self) -> bool {
        panic!("STUB: not implemented");
    }
    /// Returns the number of active receivers.
    ///
    /// An active receiver is a [`Receiver`] handle returned from [`channel`] or
    /// [`subscribe`]. These are the handles that will receive values sent on
    /// this [`Sender`].
    ///
    /// # Note
    ///
    /// It is not guaranteed that a sent message will reach this number of
    /// receivers. Active receivers may never call [`recv`] again before
    /// dropping.
    ///
    /// [`recv`]: crate::sync::broadcast::Receiver::recv
    /// [`Receiver`]: crate::sync::broadcast::Receiver
    /// [`Sender`]: crate::sync::broadcast::Sender
    /// [`subscribe`]: crate::sync::broadcast::Sender::subscribe
    /// [`channel`]: crate::sync::broadcast::channel
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::broadcast;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let (tx, _rx1) = broadcast::channel(16);
    ///
    /// assert_eq!(1, tx.receiver_count());
    ///
    /// let mut _rx2 = tx.subscribe();
    ///
    /// assert_eq!(2, tx.receiver_count());
    ///
    /// tx.send(10).unwrap();
    /// # }
    /// ```
    pub fn receiver_count(&self) -> usize {
        panic!("STUB: not implemented");
    }
    /// Returns `true` if senders belong to the same channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::broadcast;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let (tx, _rx) = broadcast::channel::<()>(16);
    /// let tx2 = tx.clone();
    ///
    /// assert!(tx.same_channel(&tx2));
    ///
    /// let (tx3, _rx3) = broadcast::channel::<()>(16);
    ///
    /// assert!(!tx3.same_channel(&tx2));
    /// # }
    /// ```
    pub fn same_channel(&self, other: &Self) -> bool {
        panic!("STUB: not implemented");
    }
    /// A future which completes when the number of [Receiver]s subscribed to this `Sender` reaches
    /// zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use futures::FutureExt;
    /// use tokio::sync::broadcast;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let (tx, mut rx1) = broadcast::channel::<u32>(16);
    /// let mut rx2 = tx.subscribe();
    ///
    /// let _ = tx.send(10);
    ///
    /// assert_eq!(rx1.recv().await.unwrap(), 10);
    /// drop(rx1);
    /// assert!(tx.closed().now_or_never().is_none());
    ///
    /// assert_eq!(rx2.recv().await.unwrap(), 10);
    /// drop(rx2);
    /// assert!(tx.closed().now_or_never().is_some());
    /// # }
    /// ```
    pub async fn closed(&self) {
        panic!("STUB: not implemented");
    }
    fn close_channel(&self) {
        panic!("STUB: not implemented");
    }
    /// Returns the number of [`Sender`] handles.
    pub fn strong_count(&self) -> usize {
        panic!("STUB: not implemented");
    }
    /// Returns the number of [`WeakSender`] handles.
    pub fn weak_count(&self) -> usize {
        panic!("STUB: not implemented");
    }
}
/// Create a new `Receiver` which reads starting from the tail.
fn new_receiver<T>(shared: Arc<Shared<T>>) -> Receiver<T> {
    panic!("STUB: not implemented");
}
/// List used in `Shared::notify_rx`. It wraps a guarded linked list
/// and gates the access to it on the `Shared.tail` mutex. It also empties
/// the list on drop.
struct WaitersList<'a, T> {
    list: GuardedLinkedList<Waiter>,
    is_empty: bool,
    shared: &'a Shared<T>,
}
impl<'a, T> Drop for WaitersList<'a, T> {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
impl<'a, T> WaitersList<'a, T> {
    fn new(
        unguarded_list: LinkedList<Waiter>,
        guard: Pin<&'a Waiter>,
        shared: &'a Shared<T>,
    ) -> Self {
        panic!("STUB: not implemented");
    }
    /// Removes the last element from the guarded list. Modifying this list
    /// requires an exclusive access to the main list in `Notify`.
    fn pop_back_locked(&mut self, _tail: &mut Tail) -> Option<NonNull<Waiter>> {
        panic!("STUB: not implemented");
    }
}
impl<T> Shared<T> {
    fn notify_rx<'a, 'b: 'a>(&'b self, mut tail: MutexGuard<'a, Tail>) {
        panic!("STUB: not implemented");
    }
}
impl<T> Clone for Sender<T> {
    fn clone(&self) -> Sender<T> {
        panic!("STUB: not implemented");
    }
}
impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
impl<T> WeakSender<T> {
    /// Tries to convert a `WeakSender` into a [`Sender`].
    ///
    /// This will return `Some` if there are other `Sender` instances alive and
    /// the channel wasn't previously dropped, otherwise `None` is returned.
    #[must_use]
    pub fn upgrade(&self) -> Option<Sender<T>> {
        panic!("STUB: not implemented");
    }
    /// Returns the number of [`Sender`] handles.
    pub fn strong_count(&self) -> usize {
        panic!("STUB: not implemented");
    }
    /// Returns the number of [`WeakSender`] handles.
    pub fn weak_count(&self) -> usize {
        panic!("STUB: not implemented");
    }
}
impl<T> Clone for WeakSender<T> {
    fn clone(&self) -> WeakSender<T> {
        panic!("STUB: not implemented");
    }
}
impl<T> Drop for WeakSender<T> {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
impl<T> Receiver<T> {
    /// Returns the number of messages that were sent into the channel and that
    /// this [`Receiver`] has yet to receive.
    ///
    /// If the returned value from `len` is larger than the next largest power of 2
    /// of the capacity of the channel any call to [`recv`] will return an
    /// `Err(RecvError::Lagged)` and any call to [`try_recv`] will return an
    /// `Err(TryRecvError::Lagged)`, e.g. if the capacity of the channel is 10,
    /// [`recv`] will start to return `Err(RecvError::Lagged)` once `len` returns
    /// values larger than 16.
    ///
    /// [`Receiver`]: crate::sync::broadcast::Receiver
    /// [`recv`]: crate::sync::broadcast::Receiver::recv
    /// [`try_recv`]: crate::sync::broadcast::Receiver::try_recv
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::broadcast;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let (tx, mut rx1) = broadcast::channel(16);
    ///
    /// tx.send(10).unwrap();
    /// tx.send(20).unwrap();
    ///
    /// assert_eq!(rx1.len(), 2);
    /// assert_eq!(rx1.recv().await.unwrap(), 10);
    /// assert_eq!(rx1.len(), 1);
    /// assert_eq!(rx1.recv().await.unwrap(), 20);
    /// assert_eq!(rx1.len(), 0);
    /// # }
    /// ```
    pub fn len(&self) -> usize {
        panic!("STUB: not implemented");
    }
    /// Returns true if there aren't any messages in the channel that the [`Receiver`]
    /// has yet to receive.
    ///
    /// [`Receiver`]: crate::sync::broadcast::Receiver
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::broadcast;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let (tx, mut rx1) = broadcast::channel(16);
    ///
    /// assert!(rx1.is_empty());
    ///
    /// tx.send(10).unwrap();
    /// tx.send(20).unwrap();
    ///
    /// assert!(!rx1.is_empty());
    /// assert_eq!(rx1.recv().await.unwrap(), 10);
    /// assert_eq!(rx1.recv().await.unwrap(), 20);
    /// assert!(rx1.is_empty());
    /// # }
    /// ```
    pub fn is_empty(&self) -> bool {
        panic!("STUB: not implemented");
    }
    /// Returns `true` if receivers belong to the same channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::broadcast;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let (tx, rx) = broadcast::channel::<()>(16);
    /// let rx2 = tx.subscribe();
    ///
    /// assert!(rx.same_channel(&rx2));
    ///
    /// let (_tx3, rx3) = broadcast::channel::<()>(16);
    ///
    /// assert!(!rx3.same_channel(&rx2));
    /// # }
    /// ```
    pub fn same_channel(&self, other: &Self) -> bool {
        panic!("STUB: not implemented");
    }
    /// Locks the next value if there is one.
    fn recv_ref(
        &mut self,
        waiter: Option<(&UnsafeCell<Waiter>, &Waker)>,
    ) -> Result<RecvGuard<'_, T>, TryRecvError> {
        panic!("STUB: not implemented");
    }
    /// Returns the number of [`Sender`] handles.
    pub fn sender_strong_count(&self) -> usize {
        panic!("STUB: not implemented");
    }
    /// Returns the number of [`WeakSender`] handles.
    pub fn sender_weak_count(&self) -> usize {
        panic!("STUB: not implemented");
    }
    /// Checks if a channel is closed.
    ///
    /// This method returns `true` if the channel has been closed. The channel is closed
    /// when all [`Sender`] have been dropped.
    ///
    /// [`Sender`]: crate::sync::broadcast::Sender
    ///
    /// # Examples
    /// ```
    /// use tokio::sync::broadcast;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let (tx, rx) = broadcast::channel::<()>(10);
    /// assert!(!rx.is_closed());
    ///
    /// drop(tx);
    ///
    /// assert!(rx.is_closed());
    /// # }
    /// ```
    pub fn is_closed(&self) -> bool {
        panic!("STUB: not implemented");
    }
}
impl<T: Clone> Receiver<T> {
    /// Re-subscribes to the channel starting from the current tail element.
    ///
    /// This [`Receiver`] handle will receive a clone of all values sent
    /// **after** it has resubscribed. This will not include elements that are
    /// in the queue of the current receiver. Consider the following example.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::broadcast;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let (tx, mut rx) = broadcast::channel(2);
    ///
    /// tx.send(1).unwrap();
    /// let mut rx2 = rx.resubscribe();
    /// tx.send(2).unwrap();
    ///
    /// assert_eq!(rx2.recv().await.unwrap(), 2);
    /// assert_eq!(rx.recv().await.unwrap(), 1);
    /// # }
    /// ```
    pub fn resubscribe(&self) -> Self {
        panic!("STUB: not implemented");
    }
    /// Receives the next value for this receiver.
    ///
    /// Each [`Receiver`] handle will receive a clone of all values sent
    /// **after** it has subscribed.
    ///
    /// `Err(RecvError::Closed)` is returned when all `Sender` halves have
    /// dropped, indicating that no further values can be sent on the channel.
    ///
    /// If the [`Receiver`] handle falls behind, once the channel is full, newly
    /// sent values will overwrite old values. At this point, a call to [`recv`]
    /// will return with `Err(RecvError::Lagged)` and the [`Receiver`]'s
    /// internal cursor is updated to point to the oldest value still held by
    /// the channel. A subsequent call to [`recv`] will return this value
    /// **unless** it has been since overwritten.
    ///
    /// # Cancel safety
    ///
    /// This method is cancel safe. If `recv` is used as a branch in
    /// [`tokio::select!`](crate::select) and another branch
    /// completes first, it is guaranteed that no messages were received on this
    /// channel.
    ///
    /// [`Receiver`]: crate::sync::broadcast::Receiver
    /// [`recv`]: crate::sync::broadcast::Receiver::recv
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::broadcast;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let (tx, mut rx1) = broadcast::channel(16);
    /// let mut rx2 = tx.subscribe();
    ///
    /// tokio::spawn(async move {
    ///     assert_eq!(rx1.recv().await.unwrap(), 10);
    ///     assert_eq!(rx1.recv().await.unwrap(), 20);
    /// });
    ///
    /// tokio::spawn(async move {
    ///     assert_eq!(rx2.recv().await.unwrap(), 10);
    ///     assert_eq!(rx2.recv().await.unwrap(), 20);
    /// });
    ///
    /// tx.send(10).unwrap();
    /// tx.send(20).unwrap();
    /// # }
    /// ```
    ///
    /// Handling lag
    ///
    /// ```
    /// use tokio::sync::broadcast;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let (tx, mut rx) = broadcast::channel(2);
    ///
    /// tx.send(10).unwrap();
    /// tx.send(20).unwrap();
    /// tx.send(30).unwrap();
    ///
    /// // The receiver lagged behind
    /// assert!(rx.recv().await.is_err());
    ///
    /// // At this point, we can abort or continue with lost messages
    ///
    /// assert_eq!(20, rx.recv().await.unwrap());
    /// assert_eq!(30, rx.recv().await.unwrap());
    /// # }
    /// ```
    pub async fn recv(&mut self) -> Result<T, RecvError> {
        panic!("STUB: not implemented");
    }
    /// Attempts to return a pending value on this receiver without awaiting.
    ///
    /// This is useful for a flavor of "optimistic check" before deciding to
    /// await on a receiver.
    ///
    /// Compared with [`recv`], this function has three failure cases instead of two
    /// (one for closed, one for an empty buffer, one for a lagging receiver).
    ///
    /// `Err(TryRecvError::Closed)` is returned when all `Sender` halves have
    /// dropped, indicating that no further values can be sent on the channel.
    ///
    /// If the [`Receiver`] handle falls behind, once the channel is full, newly
    /// sent values will overwrite old values. At this point, a call to [`recv`]
    /// will return with `Err(TryRecvError::Lagged)` and the [`Receiver`]'s
    /// internal cursor is updated to point to the oldest value still held by
    /// the channel. A subsequent call to [`try_recv`] will return this value
    /// **unless** it has been since overwritten. If there are no values to
    /// receive, `Err(TryRecvError::Empty)` is returned.
    ///
    /// [`recv`]: crate::sync::broadcast::Receiver::recv
    /// [`try_recv`]: crate::sync::broadcast::Receiver::try_recv
    /// [`Receiver`]: crate::sync::broadcast::Receiver
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::broadcast;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let (tx, mut rx) = broadcast::channel(16);
    ///
    /// assert!(rx.try_recv().is_err());
    ///
    /// tx.send(10).unwrap();
    ///
    /// let value = rx.try_recv().unwrap();
    /// assert_eq!(10, value);
    /// # }
    /// ```
    pub fn try_recv(&mut self) -> Result<T, TryRecvError> {
        panic!("STUB: not implemented");
    }
    /// Blocking receive to call outside of asynchronous contexts.
    ///
    /// # Panics
    ///
    /// This function panics if called within an asynchronous execution
    /// context.
    ///
    /// # Examples
    /// ```
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// use std::thread;
    /// use tokio::sync::broadcast;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let (tx, mut rx) = broadcast::channel(16);
    ///
    ///     let sync_code = thread::spawn(move || {
    ///         assert_eq!(rx.blocking_recv(), Ok(10));
    ///     });
    ///
    ///     let _ = tx.send(10);
    ///     sync_code.join().unwrap();
    /// }
    /// # }
    /// ```
    pub fn blocking_recv(&mut self) -> Result<T, RecvError> {
        panic!("STUB: not implemented");
    }
}
impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
impl<'a, T> Recv<'a, T> {
    fn new(receiver: &'a mut Receiver<T>) -> Recv<'a, T> {
        panic!("STUB: not implemented");
    }
    /// A custom `project` implementation is used in place of `pin-project-lite`
    /// as a custom drop implementation is needed.
    fn project(self: Pin<&mut Self>) -> (&mut Receiver<T>, &UnsafeCell<Waiter>) {
        panic!("STUB: not implemented");
    }
}
impl<'a, T> Future for Recv<'a, T>
where
    T: Clone,
{
    type Output = Result<T, RecvError>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<T, RecvError>> {
        panic!("STUB: not implemented");
    }
}
impl<'a, T> Drop for Recv<'a, T> {
    fn drop(&mut self) {
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
impl<T> fmt::Debug for Sender<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl<T> fmt::Debug for WeakSender<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl<T> fmt::Debug for Receiver<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl<'a, T> RecvGuard<'a, T> {
    fn clone_value(&self) -> Option<T>
    where
        T: Clone,
    {
        panic!("STUB: not implemented");
    }
}
impl<'a, T> Drop for RecvGuard<'a, T> {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
fn is_unpin<T: Unpin>() {
    panic!("STUB: not implemented");
}
#[cfg(not(loom))]
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn receiver_count_on_sender_constructor() {
        let sender = Sender::<i32>::new(16);
        assert_eq!(sender.receiver_count(), 0);
        let rx_1 = sender.subscribe();
        assert_eq!(sender.receiver_count(), 1);
        let rx_2 = rx_1.resubscribe();
        assert_eq!(sender.receiver_count(), 2);
        let rx_3 = sender.subscribe();
        assert_eq!(sender.receiver_count(), 3);
        drop(rx_3);
        drop(rx_1);
        assert_eq!(sender.receiver_count(), 1);
        drop(rx_2);
        assert_eq!(sender.receiver_count(), 0);
    }
    #[cfg(not(loom))]
    #[test]
    fn receiver_count_on_channel_constructor() {
        let (sender, rx) = channel::<i32>(16);
        assert_eq!(sender.receiver_count(), 1);
        let _rx_2 = rx.resubscribe();
        assert_eq!(sender.receiver_count(), 2);
    }
}
