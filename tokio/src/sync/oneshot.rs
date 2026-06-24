#![cfg_attr(not(feature = "sync"), allow(dead_code, unreachable_pub))]
//! A one-shot channel is used for sending a single message between
//! asynchronous tasks. The [`channel`] function is used to create a
//! [`Sender`] and [`Receiver`] handle pair that form the channel.
//!
//! The `Sender` handle is used by the producer to send the value.
//! The `Receiver` handle is used by the consumer to receive the value.
//!
//! Each handle can be used on separate tasks.
//!
//! Since the `send` method is not async, it can be used anywhere. This includes
//! sending between two runtimes, and using it from non-async code.
//!
//! If the [`Receiver`] is closed before receiving a message which has already
//! been sent, the message will remain in the channel until the receiver is
//! dropped, at which point the message will be dropped immediately.
//!
//! # Examples
//!
//! ```
//! use tokio::sync::oneshot;
//!
//! # #[tokio::main(flavor = "current_thread")]
//! # async fn main() {
//! let (tx, rx) = oneshot::channel();
//!
//! tokio::spawn(async move {
//!     if let Err(_) = tx.send(3) {
//!         println!("the receiver dropped");
//!     }
//! });
//!
//! match rx.await {
//!     Ok(v) => println!("got = {:?}", v),
//!     Err(_) => println!("the sender dropped"),
//! }
//! # }
//! ```
//!
//! If the sender is dropped without sending, the receiver will fail with
//! [`error::RecvError`]:
//!
//! ```
//! use tokio::sync::oneshot;
//!
//! # #[tokio::main(flavor = "current_thread")]
//! # async fn main() {
//! let (tx, rx) = oneshot::channel::<u32>();
//!
//! tokio::spawn(async move {
//!     drop(tx);
//! });
//!
//! match rx.await {
//!     Ok(_) => panic!("This doesn't happen"),
//!     Err(_) => println!("the sender dropped"),
//! }
//! # }
//! ```
//!
//! To use a `oneshot` channel in a `tokio::select!` loop, add `&mut` in front of
//! the channel.
//!
//! ```
//! use tokio::sync::oneshot;
//! use tokio::time::{interval, sleep, Duration};
//!
//! # #[tokio::main(flavor = "current_thread")]
//! # async fn _doc() {}
//! # #[tokio::main(flavor = "current_thread", start_paused = true)]
//! # async fn main() {
//! let (send, mut recv) = oneshot::channel();
//! let mut interval = interval(Duration::from_millis(100));
//!
//! # let handle =
//! tokio::spawn(async move {
//!     sleep(Duration::from_secs(1)).await;
//!     send.send("shut down").unwrap();
//! });
//!
//! loop {
//!     tokio::select! {
//!         _ = interval.tick() => println!("Another 100ms"),
//!         msg = &mut recv => {
//!             println!("Got message: {}", msg.unwrap());
//!             break;
//!         }
//!     }
//! }
//! # handle.await.unwrap();
//! # }
//! ```
//!
//! To use a `Sender` from a destructor, put it in an [`Option`] and call
//! [`Option::take`].
//!
//! ```
//! use tokio::sync::oneshot;
//!
//! struct SendOnDrop {
//!     sender: Option<oneshot::Sender<&'static str>>,
//! }
//! impl Drop for SendOnDrop {
//!     fn drop(&mut self) {
//!         if let Some(sender) = self.sender.take() {
//!             // Using `let _ =` to ignore send errors.
//!             let _ = sender.send("I got dropped!");
//!         }
//!     }
//! }
//!
//! # #[tokio::main(flavor = "current_thread")]
//! # async fn _doc() {}
//! # #[tokio::main(flavor = "current_thread")]
//! # async fn main() {
//! let (send, recv) = oneshot::channel();
//!
//! let send_on_drop = SendOnDrop { sender: Some(send) };
//! drop(send_on_drop);
//!
//! assert_eq!(recv.await, Ok("I got dropped!"));
//! # }
//! ```
use crate::loom::cell::UnsafeCell;
use crate::loom::sync::atomic::AtomicUsize;
use crate::loom::sync::Arc;
#[cfg(all(tokio_unstable, feature = "tracing"))]
use crate::util::trace;
use std::fmt;
use std::future::Future;
use std::mem::MaybeUninit;
use std::pin::Pin;
use std::sync::atomic::Ordering::{self, AcqRel, Acquire};
use std::task::Poll::{Pending, Ready};
use std::task::{ready, Context, Poll, Waker};
/// Sends a value to the associated [`Receiver`].
///
/// A pair of both a [`Sender`] and a [`Receiver`]  are created by the
/// [`channel`](fn@channel) function.
///
/// # Examples
///
/// ```
/// use tokio::sync::oneshot;
///
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() {
/// let (tx, rx) = oneshot::channel();
///
/// tokio::spawn(async move {
///     if let Err(_) = tx.send(3) {
///         println!("the receiver dropped");
///     }
/// });
///
/// match rx.await {
///     Ok(v) => println!("got = {:?}", v),
///     Err(_) => println!("the sender dropped"),
/// }
/// # }
/// ```
///
/// If the sender is dropped without sending, the receiver will fail with
/// [`error::RecvError`]:
///
/// ```
/// use tokio::sync::oneshot;
///
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() {
/// let (tx, rx) = oneshot::channel::<u32>();
///
/// tokio::spawn(async move {
///     drop(tx);
/// });
///
/// match rx.await {
///     Ok(_) => panic!("This doesn't happen"),
///     Err(_) => println!("the sender dropped"),
/// }
/// # }
/// ```
///
/// To use a `Sender` from a destructor, put it in an [`Option`] and call
/// [`Option::take`].
///
/// ```
/// use tokio::sync::oneshot;
///
/// struct SendOnDrop {
///     sender: Option<oneshot::Sender<&'static str>>,
/// }
/// impl Drop for SendOnDrop {
///     fn drop(&mut self) {
///         if let Some(sender) = self.sender.take() {
///             // Using `let _ =` to ignore send errors.
///             let _ = sender.send("I got dropped!");
///         }
///     }
/// }
///
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn _doc() {}
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() {
/// let (send, recv) = oneshot::channel();
///
/// let send_on_drop = SendOnDrop { sender: Some(send) };
/// drop(send_on_drop);
///
/// assert_eq!(recv.await, Ok("I got dropped!"));
/// # }
/// ```
///
/// [`Option`]: std::option::Option
/// [`Option::take`]: std::option::Option::take
#[derive(Debug)]
pub struct Sender<T> {
    inner: Option<Arc<Inner<T>>>,
    #[cfg(all(tokio_unstable, feature = "tracing"))]
    resource_span: tracing::Span,
}
/// Receives a value from the associated [`Sender`].
///
/// A pair of both a [`Sender`] and a [`Receiver`]  are created by the
/// [`channel`](fn@channel) function.
///
/// This channel has no `recv` method because the receiver itself implements the
/// [`Future`] trait. To receive a `Result<T, `[`error::RecvError`]`>`, `.await` the `Receiver` object directly.
///
/// The `poll` method on the `Future` trait is allowed to spuriously return
/// `Poll::Pending` even if the message has been sent. If such a spurious
/// failure happens, then the caller will be woken when the spurious failure has
/// been resolved so that the caller can attempt to receive the message again.
/// Note that receiving such a wakeup does not guarantee that the next call will
/// succeed — it could fail with another spurious failure. (A spurious failure
/// does not mean that the message is lost. It is just delayed.)
///
/// [`Future`]: trait@std::future::Future
///
/// # Cancel safety
///
/// Awaiting a `&mut Receiver<T>` is cancel safe. If it is used as a branch in
/// [`tokio::select!`](crate::select) and another branch completes first, it is
/// guaranteed that no message was received on this
/// channel.
///
/// # Examples
///
/// ```
/// use tokio::sync::oneshot;
///
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() {
/// let (tx, rx) = oneshot::channel();
///
/// tokio::spawn(async move {
///     if let Err(_) = tx.send(3) {
///         println!("the receiver dropped");
///     }
/// });
///
/// match rx.await {
///     Ok(v) => println!("got = {:?}", v),
///     Err(_) => println!("the sender dropped"),
/// }
/// # }
/// ```
///
/// If the sender is dropped without sending, the receiver will fail with
/// [`error::RecvError`]:
///
/// ```
/// use tokio::sync::oneshot;
///
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() {
/// let (tx, rx) = oneshot::channel::<u32>();
///
/// tokio::spawn(async move {
///     drop(tx);
/// });
///
/// match rx.await {
///     Ok(_) => panic!("This doesn't happen"),
///     Err(_) => println!("the sender dropped"),
/// }
/// # }
/// ```
///
/// To use a `Receiver` in a `tokio::select!` loop, add `&mut` in front of the
/// channel.
///
/// ```
/// use tokio::sync::oneshot;
/// use tokio::time::{interval, sleep, Duration};
///
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn _doc() {}
/// # #[tokio::main(flavor = "current_thread", start_paused = true)]
/// # async fn main() {
/// let (send, mut recv) = oneshot::channel();
/// let mut interval = interval(Duration::from_millis(100));
///
/// # let handle =
/// tokio::spawn(async move {
///     sleep(Duration::from_secs(1)).await;
///     send.send("shut down").unwrap();
/// });
///
/// loop {
///     tokio::select! {
///         _ = interval.tick() => println!("Another 100ms"),
///         msg = &mut recv => {
///             println!("Got message: {}", msg.unwrap());
///             break;
///         }
///     }
/// }
/// # handle.await.unwrap();
/// # }
/// ```
#[derive(Debug)]
pub struct Receiver<T> {
    inner: Option<Arc<Inner<T>>>,
    #[cfg(all(tokio_unstable, feature = "tracing"))]
    resource_span: tracing::Span,
    #[cfg(all(tokio_unstable, feature = "tracing"))]
    async_op_span: tracing::Span,
    #[cfg(all(tokio_unstable, feature = "tracing"))]
    async_op_poll_span: tracing::Span,
}
pub mod error {
    //! `Oneshot` error types.
    use std::fmt;
    /// Error returned by the `Future` implementation for `Receiver`.
    ///
    /// This error is returned by the receiver when the sender is dropped without sending.
    #[derive(Debug, Eq, PartialEq, Clone)]
    pub struct RecvError(pub(super) ());
    /// Error returned by the `try_recv` function on `Receiver`.
    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum TryRecvError {
        /// The send half of the channel has not yet sent a value.
        Empty,
        /// The send half of the channel was dropped without sending a value.
        Closed,
    }
    impl fmt::Display for RecvError {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            panic!("STUB: not implemented");
        }
    }
    impl std::error::Error for RecvError {}
    impl fmt::Display for TryRecvError {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            panic!("STUB: not implemented");
        }
    }
    impl std::error::Error for TryRecvError {}
}
use self::error::*;
struct Inner<T> {
    /// Manages the state of the inner cell.
    state: AtomicUsize,
    /// The value. This is set by `Sender` and read by `Receiver`. The state of
    /// the cell is tracked by `state`.
    value: UnsafeCell<Option<T>>,
    /// The task to notify when the receiver drops without consuming the value.
    ///
    /// ## Safety
    ///
    /// The `TX_TASK_SET` bit in the `state` field is set if this field is
    /// initialized. If that bit is unset, this field may be uninitialized.
    tx_task: Task,
    /// The task to notify when the value is sent.
    ///
    /// ## Safety
    ///
    /// The `RX_TASK_SET` bit in the `state` field is set if this field is
    /// initialized. If that bit is unset, this field may be uninitialized.
    rx_task: Task,
}
struct Task(UnsafeCell<MaybeUninit<Waker>>);
impl Task {
    /// # Safety
    ///
    /// The caller must do the necessary synchronization to ensure that
    /// the [`Self::0`] contains the valid [`Waker`] during the call.
    unsafe fn will_wake(&self, cx: &mut Context<'_>) -> bool {
        panic!("STUB: not implemented");
    }
    /// # Safety
    ///
    /// The caller must do the necessary synchronization to ensure that
    /// the [`Self::0`] contains the valid [`Waker`] during the call.
    unsafe fn with_task<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&Waker) -> R,
    {
        panic!("STUB: not implemented");
    }
    /// # Safety
    ///
    /// The caller must do the necessary synchronization to ensure that
    /// the [`Self::0`] contains the valid [`Waker`] during the call.
    unsafe fn drop_task(&self) {
        panic!("STUB: not implemented");
    }
    /// # Safety
    ///
    /// The caller must do the necessary synchronization to ensure that
    /// the [`Self::0`] contains the valid [`Waker`] during the call.
    unsafe fn set_task(&self, cx: &mut Context<'_>) {
        panic!("STUB: not implemented");
    }
}
#[derive(Clone, Copy)]
struct State(usize);
/// Creates a new one-shot channel for sending single values across asynchronous
/// tasks.
///
/// The function returns separate "send" and "receive" handles. The `Sender`
/// handle is used by the producer to send the value. The `Receiver` handle is
/// used by the consumer to receive the value.
///
/// Each handle can be used on separate tasks.
///
/// # Examples
///
/// ```
/// use tokio::sync::oneshot;
///
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() {
/// let (tx, rx) = oneshot::channel();
///
/// tokio::spawn(async move {
///     if let Err(_) = tx.send(3) {
///         println!("the receiver dropped");
///     }
/// });
///
/// match rx.await {
///     Ok(v) => println!("got = {:?}", v),
///     Err(_) => println!("the sender dropped"),
/// }
/// # }
/// ```
#[track_caller]
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    panic!("STUB: not implemented");
}
impl<T> Sender<T> {
    /// Attempts to send a value on this channel, returning it back if it could
    /// not be sent.
    ///
    /// This method consumes `self` as only one value may ever be sent on a `oneshot`
    /// channel. It is not marked async because sending a message to a `oneshot`
    /// channel never requires any form of waiting.  Because of this, the `send`
    /// method can be used in both synchronous and asynchronous code without
    /// problems.
    ///
    /// A successful send occurs when it is determined that the other end of the
    /// channel has not hung up already. An unsuccessful send would be one where
    /// the corresponding receiver has already been deallocated. Note that a
    /// return value of `Err` means that the data will never be received, but
    /// a return value of `Ok` does *not* mean that the data will be received.
    /// It is possible for the corresponding receiver to hang up immediately
    /// after this function returns `Ok`.
    ///
    /// # Examples
    ///
    /// Send a value to another task
    ///
    /// ```
    /// use tokio::sync::oneshot;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let (tx, rx) = oneshot::channel();
    ///
    /// tokio::spawn(async move {
    ///     if let Err(_) = tx.send(3) {
    ///         println!("the receiver dropped");
    ///     }
    /// });
    ///
    /// match rx.await {
    ///     Ok(v) => println!("got = {:?}", v),
    ///     Err(_) => println!("the sender dropped"),
    /// }
    /// # }
    /// ```
    pub fn send(mut self, t: T) -> Result<(), T> {
        panic!("STUB: not implemented");
    }
    /// Waits for the associated [`Receiver`] handle to close.
    ///
    /// A [`Receiver`] is closed by either calling [`close`] explicitly or the
    /// [`Receiver`] value is dropped.
    ///
    /// This function is useful when paired with `select!` to abort a
    /// computation when the receiver is no longer interested in the result.
    ///
    /// # Return
    ///
    /// Returns a `Future` which must be awaited on.
    ///
    /// [`Receiver`]: Receiver
    /// [`close`]: Receiver::close
    ///
    /// # Examples
    ///
    /// Basic usage
    ///
    /// ```
    /// use tokio::sync::oneshot;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let (mut tx, rx) = oneshot::channel::<()>();
    ///
    /// tokio::spawn(async move {
    ///     drop(rx);
    /// });
    ///
    /// tx.closed().await;
    /// println!("the receiver dropped");
    /// # }
    /// ```
    ///
    /// Paired with select
    ///
    /// ```
    /// use tokio::sync::oneshot;
    /// use tokio::time::{self, Duration};
    ///
    /// async fn compute() -> String {
    ///     // Complex computation returning a `String`
    /// # "hello".to_string()
    /// }
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let (mut tx, rx) = oneshot::channel();
    ///
    /// tokio::spawn(async move {
    ///     tokio::select! {
    ///         _ = tx.closed() => {
    ///             // The receiver dropped, no need to do any further work
    ///         }
    ///         value = compute() => {
    ///             // The send can fail if the channel was closed at the exact same
    ///             // time as when compute() finished, so just ignore the failure.
    ///             let _ = tx.send(value);
    ///         }
    ///     }
    /// });
    ///
    /// // Wait for up to 10 seconds
    /// let _ = time::timeout(Duration::from_secs(10), rx).await;
    /// # }
    /// ```
    pub async fn closed(&mut self) {
        panic!("STUB: not implemented");
    }
    /// Returns `true` if the associated [`Receiver`] handle has been dropped.
    ///
    /// A [`Receiver`] is closed by either calling [`close`] explicitly or the
    /// [`Receiver`] value is dropped.
    ///
    /// If `true` is returned, a call to `send` will always result in an error.
    ///
    /// [`Receiver`]: Receiver
    /// [`close`]: Receiver::close
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::oneshot;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let (tx, rx) = oneshot::channel();
    ///
    /// assert!(!tx.is_closed());
    ///
    /// drop(rx);
    ///
    /// assert!(tx.is_closed());
    /// assert!(tx.send("never received").is_err());
    /// # }
    /// ```
    pub fn is_closed(&self) -> bool {
        panic!("STUB: not implemented");
    }
    /// Checks whether the `oneshot` channel has been closed, and if not, schedules the
    /// `Waker` in the provided `Context` to receive a notification when the channel is
    /// closed.
    ///
    /// A [`Receiver`] is closed by either calling [`close`] explicitly, or when the
    /// [`Receiver`] value is dropped.
    ///
    /// Note that on multiple calls to poll, only the `Waker` from the `Context` passed
    /// to the most recent call will be scheduled to receive a wakeup.
    ///
    /// [`Receiver`]: struct@crate::sync::oneshot::Receiver
    /// [`close`]: fn@crate::sync::oneshot::Receiver::close
    ///
    /// # Return value
    ///
    /// This function returns:
    ///
    ///  * `Poll::Pending` if the channel is still open.
    ///  * `Poll::Ready(())` if the channel is closed.
    ///
    /// # Examples
    ///
    /// ```
    /// use tokio::sync::oneshot;
    ///
    /// use std::future::poll_fn;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let (mut tx, mut rx) = oneshot::channel::<()>();
    ///
    /// tokio::spawn(async move {
    ///     rx.close();
    /// });
    ///
    /// poll_fn(|cx| tx.poll_closed(cx)).await;
    ///
    /// println!("the receiver dropped");
    /// # }
    /// ```
    pub fn poll_closed(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        panic!("STUB: not implemented");
    }
}
impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
impl<T> Receiver<T> {
    /// Prevents the associated [`Sender`] handle from sending a value.
    ///
    /// Any `send` operation which happens after calling `close` is guaranteed
    /// to fail. After calling `close`, [`try_recv`] should be called to
    /// receive a value if one was sent **before** the call to `close`
    /// completed.
    ///
    /// This function is useful to perform a graceful shutdown and ensure that a
    /// value will not be sent into the channel and never received.
    ///
    /// `close` is no-op if a message is already received or the channel
    /// is already closed.
    ///
    /// [`Sender`]: Sender
    /// [`try_recv`]: Receiver::try_recv
    ///
    /// # Examples
    ///
    /// Prevent a value from being sent
    ///
    /// ```
    /// use tokio::sync::oneshot;
    /// use tokio::sync::oneshot::error::TryRecvError;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let (tx, mut rx) = oneshot::channel();
    ///
    /// assert!(!tx.is_closed());
    ///
    /// rx.close();
    ///
    /// assert!(tx.is_closed());
    /// assert!(tx.send("never received").is_err());
    ///
    /// match rx.try_recv() {
    ///     Err(TryRecvError::Closed) => {}
    ///     _ => unreachable!(),
    /// }
    /// # }
    /// ```
    ///
    /// Receive a value sent **before** calling `close`
    ///
    /// ```
    /// use tokio::sync::oneshot;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let (tx, mut rx) = oneshot::channel();
    ///
    /// assert!(tx.send("will receive").is_ok());
    ///
    /// rx.close();
    ///
    /// let msg = rx.try_recv().unwrap();
    /// assert_eq!(msg, "will receive");
    /// # }
    /// ```
    pub fn close(&mut self) {
        panic!("STUB: not implemented");
    }
    /// Checks if this receiver is terminated.
    ///
    /// This function returns true if this receiver has already yielded a [`Poll::Ready`] result.
    /// If so, this receiver should no longer be polled.
    ///
    /// # Examples
    ///
    /// Sending a value and polling it.
    ///
    /// ```
    /// use tokio::sync::oneshot;
    ///
    /// use std::task::Poll;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let (tx, mut rx) = oneshot::channel();
    ///
    /// // A receiver is not terminated when it is initialized.
    /// assert!(!rx.is_terminated());
    ///
    /// // A receiver is not terminated it is polled and is still pending.
    /// let poll = futures::poll!(&mut rx);
    /// assert_eq!(poll, Poll::Pending);
    /// assert!(!rx.is_terminated());
    ///
    /// // A receiver is not terminated if a value has been sent, but not yet read.
    /// tx.send(0).unwrap();
    /// assert!(!rx.is_terminated());
    ///
    /// // A receiver *is* terminated after it has been polled and yielded a value.
    /// assert_eq!((&mut rx).await, Ok(0));
    /// assert!(rx.is_terminated());
    /// # }
    /// ```
    ///
    /// Dropping the sender.
    ///
    /// ```
    /// use tokio::sync::oneshot;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let (tx, mut rx) = oneshot::channel::<()>();
    ///
    /// // A receiver is not immediately terminated when the sender is dropped.
    /// drop(tx);
    /// assert!(!rx.is_terminated());
    ///
    /// // A receiver *is* terminated after it has been polled and yielded an error.
    /// let _ = (&mut rx).await.unwrap_err();
    /// assert!(rx.is_terminated());
    /// # }
    /// ```
    pub fn is_terminated(&self) -> bool {
        panic!("STUB: not implemented");
    }
    /// Checks if a channel is empty.
    ///
    /// This method returns `true` if the channel has no messages.
    ///
    /// It is not necessarily safe to poll an empty receiver, which may have
    /// already yielded a value. Use [`is_terminated()`][Self::is_terminated]
    /// to check whether or not a receiver can be safely polled, instead.
    ///
    /// # Examples
    ///
    /// Sending a value.
    ///
    /// ```
    /// use tokio::sync::oneshot;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let (tx, mut rx) = oneshot::channel();
    /// assert!(rx.is_empty());
    ///
    /// tx.send(0).unwrap();
    /// assert!(!rx.is_empty());
    ///
    /// let _ = (&mut rx).await;
    /// assert!(rx.is_empty());
    /// # }
    /// ```
    ///
    /// Dropping the sender.
    ///
    /// ```
    /// use tokio::sync::oneshot;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let (tx, mut rx) = oneshot::channel::<()>();
    ///
    /// // A channel is empty if the sender is dropped.
    /// drop(tx);
    /// assert!(rx.is_empty());
    ///
    /// // A closed channel still yields an error, however.
    /// (&mut rx).await.expect_err("should yield an error");
    /// assert!(rx.is_empty());
    /// # }
    /// ```
    ///
    /// Terminated channels are empty.
    ///
    /// ```should_panic,ignore-wasm
    /// use tokio::sync::oneshot;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let (tx, mut rx) = oneshot::channel();
    ///     tx.send(0).unwrap();
    ///     let _ = (&mut rx).await;
    ///
    ///     // NB: an empty channel is not necessarily safe to poll!
    ///     assert!(rx.is_empty());
    ///     let _ = (&mut rx).await;
    /// }
    /// ```
    pub fn is_empty(&self) -> bool {
        panic!("STUB: not implemented");
    }
    /// Attempts to receive a value.
    ///
    /// If a pending value exists in the channel, it is returned. If no value
    /// has been sent, the current task **will not** be registered for
    /// future notification.
    ///
    /// This function is useful to call from outside the context of an
    /// asynchronous task.
    ///
    /// Note that unlike the `poll` method, the `try_recv` method cannot fail
    /// spuriously. Any send or close event that happens before this call to
    /// `try_recv` will be correctly returned to the caller.
    ///
    /// # Return
    ///
    /// - `Ok(T)` if a value is pending in the channel.
    /// - `Err(TryRecvError::Empty)` if no value has been sent yet.
    /// - `Err(TryRecvError::Closed)` if the sender has dropped without sending
    ///   a value, or if the message has already been received.
    ///
    /// # Examples
    ///
    /// `try_recv` before a value is sent, then after.
    ///
    /// ```
    /// use tokio::sync::oneshot;
    /// use tokio::sync::oneshot::error::TryRecvError;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let (tx, mut rx) = oneshot::channel();
    ///
    /// match rx.try_recv() {
    ///     // The channel is currently empty
    ///     Err(TryRecvError::Empty) => {}
    ///     _ => unreachable!(),
    /// }
    ///
    /// // Send a value
    /// tx.send("hello").unwrap();
    ///
    /// match rx.try_recv() {
    ///      Ok(value) => assert_eq!(value, "hello"),
    ///      _ => unreachable!(),
    /// }
    /// # }
    /// ```
    ///
    /// `try_recv` when the sender dropped before sending a value
    ///
    /// ```
    /// use tokio::sync::oneshot;
    /// use tokio::sync::oneshot::error::TryRecvError;
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let (tx, mut rx) = oneshot::channel::<()>();
    ///
    /// drop(tx);
    ///
    /// match rx.try_recv() {
    ///     // The channel will never receive a value.
    ///     Err(TryRecvError::Closed) => {}
    ///     _ => unreachable!(),
    /// }
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
    ///
    /// ```
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// use std::thread;
    /// use tokio::sync::oneshot;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let (tx, rx) = oneshot::channel::<u8>();
    ///
    ///     let sync_code = thread::spawn(move || {
    ///         assert_eq!(Ok(10), rx.blocking_recv());
    ///     });
    ///
    ///     let _ = tx.send(10);
    ///     sync_code.join().unwrap();
    /// }
    /// # }
    /// ```
    #[track_caller]
    #[cfg(feature = "sync")]
    #[cfg_attr(docsrs, doc(alias = "recv_blocking"))]
    pub fn blocking_recv(self) -> Result<T, RecvError> {
        panic!("STUB: not implemented");
    }
}
impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
impl<T> Future for Receiver<T> {
    type Output = Result<T, RecvError>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        panic!("STUB: not implemented");
    }
}
impl<T> Inner<T> {
    fn complete(&self) -> bool {
        panic!("STUB: not implemented");
    }
    fn poll_recv(&self, cx: &mut Context<'_>) -> Poll<Result<T, RecvError>> {
        panic!("STUB: not implemented");
    }
    /// Called by `Receiver` to indicate that the value will never be received.
    fn close(&self) -> State {
        panic!("STUB: not implemented");
    }
    /// Consumes the value. This function does not check `state`.
    ///
    /// # Safety
    ///
    /// Calling this method concurrently on multiple threads will result in a
    /// data race. The `VALUE_SENT` state bit is used to ensure that only the
    /// sender *or* the receiver will call this method at a given point in time.
    /// If `VALUE_SENT` is not set, then only the sender may call this method;
    /// if it is set, then only the receiver may call this method.
    unsafe fn consume_value(&self) -> Option<T> {
        panic!("STUB: not implemented");
    }
    /// Returns true if there is a value. This function does not check `state`.
    ///
    /// # Safety
    ///
    /// Calling this method concurrently on multiple threads will result in a
    /// data race. The `VALUE_SENT` state bit is used to ensure that only the
    /// sender *or* the receiver will call this method at a given point in time.
    /// If `VALUE_SENT` is not set, then only the sender may call this method;
    /// if it is set, then only the receiver may call this method.
    unsafe fn has_value(&self) -> bool {
        panic!("STUB: not implemented");
    }
}
unsafe impl<T: Send> Send for Inner<T> {}
unsafe impl<T: Send> Sync for Inner<T> {}
fn mut_load(this: &mut AtomicUsize) -> usize {
    panic!("STUB: not implemented");
}
impl<T> Drop for Inner<T> {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
impl<T: fmt::Debug> fmt::Debug for Inner<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
/// Indicates that a waker for the receiving task has been set.
///
/// # Safety
///
/// If this bit is not set, the `rx_task` field may be uninitialized.
const RX_TASK_SET: usize = 0b00001;
/// Indicates that a value has been stored in the channel's inner `UnsafeCell`.
///
/// # Safety
///
/// This bit controls which side of the channel is permitted to access the
/// `UnsafeCell`. If it is set, the `UnsafeCell` may ONLY be accessed by the
/// receiver. If this bit is NOT set, the `UnsafeCell` may ONLY be accessed by
/// the sender.
const VALUE_SENT: usize = 0b00010;
const CLOSED: usize = 0b00100;
/// Indicates that a waker for the sending task has been set.
///
/// # Safety
///
/// If this bit is not set, the `tx_task` field may be uninitialized.
const TX_TASK_SET: usize = 0b01000;
impl State {
    fn new() -> State {
        panic!("STUB: not implemented");
    }
    fn is_complete(self) -> bool {
        panic!("STUB: not implemented");
    }
    fn set_complete(cell: &AtomicUsize) -> State {
        panic!("STUB: not implemented");
    }
    fn is_rx_task_set(self) -> bool {
        panic!("STUB: not implemented");
    }
    fn set_rx_task(cell: &AtomicUsize) -> State {
        panic!("STUB: not implemented");
    }
    fn unset_rx_task(cell: &AtomicUsize) -> State {
        panic!("STUB: not implemented");
    }
    fn is_closed(self) -> bool {
        panic!("STUB: not implemented");
    }
    fn set_closed(cell: &AtomicUsize) -> State {
        panic!("STUB: not implemented");
    }
    fn set_tx_task(cell: &AtomicUsize) -> State {
        panic!("STUB: not implemented");
    }
    fn unset_tx_task(cell: &AtomicUsize) -> State {
        panic!("STUB: not implemented");
    }
    fn is_tx_task_set(self) -> bool {
        panic!("STUB: not implemented");
    }
    fn as_usize(self) -> usize {
        panic!("STUB: not implemented");
    }
    fn load(cell: &AtomicUsize, order: Ordering) -> State {
        panic!("STUB: not implemented");
    }
}
impl fmt::Debug for State {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
