use crate::runtime::{scheduler, Timer};
use crate::time::{error::Error, Duration, Instant};
use crate::util::trace;
use pin_project_lite::pin_project;
use std::future::Future;
use std::panic::Location;
use std::pin::Pin;
use std::task::{self, ready, Poll};
/// Waits until `deadline` is reached.
///
/// No work is performed while awaiting on the sleep future to complete. `Sleep`
/// operates at millisecond granularity and should not be used for tasks that
/// require high-resolution timers.
///
/// To run something regularly on a schedule, see [`interval`].
///
/// # Cancellation
///
/// Canceling a sleep instance is done by dropping the returned future. No additional
/// cleanup work is required.
///
/// # Examples
///
/// Wait 100ms and print "100 ms have elapsed".
///
/// ```
/// use tokio::time::{sleep_until, Instant, Duration};
///
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() {
/// sleep_until(Instant::now() + Duration::from_millis(100)).await;
/// println!("100 ms have elapsed");
/// # }
/// ```
///
/// See the documentation for the [`Sleep`] type for more examples.
///
/// # Panics
///
/// This function panics if there is no current timer set.
///
/// It can be triggered when [`Builder::enable_time`] or
/// [`Builder::enable_all`] are not included in the builder.
///
/// It can also panic whenever a timer is created outside of a
/// Tokio runtime. That is why `rt.block_on(sleep(...))` will panic,
/// since the function is executed outside of the runtime.
/// Whereas `rt.block_on(async {sleep(...).await})` doesn't panic.
/// And this is because wrapping the function on an async makes it lazy,
/// and so gets executed inside the runtime successfully without
/// panicking.
///
/// [`Sleep`]: struct@crate::time::Sleep
/// [`interval`]: crate::time::interval()
/// [`Builder::enable_time`]: crate::runtime::Builder::enable_time
/// [`Builder::enable_all`]: crate::runtime::Builder::enable_all
#[cfg_attr(docsrs, doc(alias = "delay_until"))]
#[track_caller]
pub fn sleep_until(deadline: Instant) -> Sleep {
    panic!("STUB: not implemented");
}
/// Waits until `duration` has elapsed.
///
/// Equivalent to `sleep_until(Instant::now() + duration)`. An asynchronous
/// analog to `std::thread::sleep`.
///
/// No work is performed while awaiting on the sleep future to complete. `Sleep`
/// operates at millisecond granularity and should not be used for tasks that
/// require high-resolution timers. The implementation is platform specific,
/// and some platforms (specifically Windows) will provide timers with a
/// larger resolution than 1 ms.
///
/// To run something regularly on a schedule, see [`interval`].
///
/// # Cancellation
///
/// Canceling a sleep instance is done by dropping the returned future. No additional
/// cleanup work is required.
///
/// # Examples
///
/// Wait 100ms and print "100 ms have elapsed".
///
/// ```
/// use tokio::time::{sleep, Duration};
///
/// # #[tokio::main(flavor = "current_thread")]
/// # async fn main() {
/// sleep(Duration::from_millis(100)).await;
/// println!("100 ms have elapsed");
/// # }
/// ```
///
/// See the documentation for the [`Sleep`] type for more examples.
///
/// # Panics
///
/// This function panics if there is no current timer set.
///
/// It can be triggered when [`Builder::enable_time`] or
/// [`Builder::enable_all`] are not included in the builder.
///
/// It can also panic whenever a timer is created outside of a
/// Tokio runtime. That is why `rt.block_on(sleep(...))` will panic,
/// since the function is executed outside of the runtime.
/// Whereas `rt.block_on(async {sleep(...).await})` doesn't panic.
/// And this is because wrapping the function on an async makes it lazy,
/// and so gets executed inside the runtime successfully without
/// panicking.
///
/// [`Sleep`]: struct@crate::time::Sleep
/// [`interval`]: crate::time::interval()
/// [`Builder::enable_time`]: crate::runtime::Builder::enable_time
/// [`Builder::enable_all`]: crate::runtime::Builder::enable_all
#[cfg_attr(docsrs, doc(alias = "delay_for"))]
#[cfg_attr(docsrs, doc(alias = "wait"))]
#[track_caller]
pub fn sleep(duration: Duration) -> Sleep {
    panic!("STUB: not implemented");
}
pin_project! {
    #[doc = " Future returned by [`sleep`](sleep) and [`sleep_until`](sleep_until)."]
    #[doc = ""] #[doc =
    " This type does not implement the `Unpin` trait, which means that if you"] #[doc =
    " use it with [`select!`] or by calling `poll`, you have to pin it first."] #[doc =
    " If you use it with `.await`, this does not apply."] #[doc = ""] #[doc =
    " # Examples"] #[doc = ""] #[doc = " Wait 100ms and print \"100 ms have elapsed\"."]
    #[doc = ""] #[doc = " ```"] #[doc = " use tokio::time::{sleep, Duration};"] #[doc =
    ""] #[doc = " # #[tokio::main(flavor = \"current_thread\")]"] #[doc =
    " # async fn main() {"] #[doc = " sleep(Duration::from_millis(100)).await;"] #[doc =
    " println!(\"100 ms have elapsed\");"] #[doc = " # }"] #[doc = " ```"] #[doc = ""]
    #[doc = " Use with [`select!`]. Pinning the `Sleep` with [`tokio::pin!`] is"] #[doc =
    " necessary when the same `Sleep` is selected on multiple times."] #[doc =
    " ```no_run"] #[doc = " use tokio::time::{self, Duration, Instant};"] #[doc = ""]
    #[doc = " # #[tokio::main(flavor = \"current_thread\")]"] #[doc =
    " # async fn main() {"] #[doc =
    " let sleep = time::sleep(Duration::from_millis(10));"] #[doc =
    " tokio::pin!(sleep);"] #[doc = ""] #[doc = " loop {"] #[doc =
    "     tokio::select! {"] #[doc = "         () = &mut sleep => {"] #[doc =
    "             println!(\"timer elapsed\");"] #[doc =
    "             sleep.as_mut().reset(Instant::now() + Duration::from_millis(50));"]
    #[doc = "         },"] #[doc = "     }"] #[doc = " }"] #[doc = " # }"] #[doc =
    " ```"] #[doc =
    " Use in a struct with boxing. By pinning the `Sleep` with a `Box`, the"] #[doc =
    " `HasSleep` struct implements `Unpin`, even though `Sleep` does not."] #[doc =
    " ```"] #[doc = " use std::future::Future;"] #[doc = " use std::pin::Pin;"] #[doc =
    " use std::task::{Context, Poll};"] #[doc = " use tokio::time::Sleep;"] #[doc = ""]
    #[doc = " struct HasSleep {"] #[doc = "     sleep: Pin<Box<Sleep>>,"] #[doc = " }"]
    #[doc = ""] #[doc = " impl Future for HasSleep {"] #[doc = "     type Output = ();"]
    #[doc = ""] #[doc =
    "     fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {"] #[doc =
    "         self.sleep.as_mut().poll(cx)"] #[doc = "     }"] #[doc = " }"] #[doc =
    " ```"] #[doc =
    " Use in a struct with pin projection. This method avoids the `Box`, but"] #[doc =
    " the `HasSleep` struct will not be `Unpin` as a consequence."] #[doc = " ```"] #[doc
    = " use std::future::Future;"] #[doc = " use std::pin::Pin;"] #[doc =
    " use std::task::{Context, Poll};"] #[doc = " use tokio::time::Sleep;"] #[doc =
    " use pin_project_lite::pin_project;"] #[doc = ""] #[doc = " pin_project! {"] #[doc =
    "     struct HasSleep {"] #[doc = "         #[pin]"] #[doc =
    "         sleep: Sleep,"] #[doc = "     }"] #[doc = " }"] #[doc = ""] #[doc =
    " impl Future for HasSleep {"] #[doc = "     type Output = ();"] #[doc = ""] #[doc =
    "     fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {"] #[doc =
    "         self.project().sleep.poll(cx)"] #[doc = "     }"] #[doc = " }"] #[doc =
    " ```"] #[doc = ""] #[doc = " [`select!`]: ../macro.select.html"] #[doc =
    " [`tokio::pin!`]: ../macro.pin.html"] #[project(! Unpin)] #[cfg_attr(docsrs,
    doc(alias = "Delay"))] #[derive(Debug)] #[must_use =
    "futures do nothing unless you `.await` or poll them"] pub struct Sleep { deadline :
    Instant, driver : scheduler::Handle, inner : Inner, #[pin] timer : Option < Timer >,
    }
}
cfg_trace! {
    #[derive(Debug)] struct Inner { ctx : trace::AsyncOpTracingCtx, }
}
cfg_not_trace! {
    #[derive(Debug)] struct Inner {}
}
impl Sleep {
    #[cfg_attr(not(all(tokio_unstable, feature = "tracing")), allow(unused_variables))]
    #[track_caller]
    pub(crate) fn new_timeout(
        deadline: Instant,
        location: Option<&'static Location<'static>>,
    ) -> Sleep {
        panic!("STUB: not implemented");
    }
    pub(crate) fn far_future(location: Option<&'static Location<'static>>) -> Sleep {
        panic!("STUB: not implemented");
    }
    /// Returns the instant at which the future will complete.
    pub fn deadline(&self) -> Instant {
        panic!("STUB: not implemented");
    }
    /// Returns `true` if `Sleep` has elapsed.
    ///
    /// A `Sleep` instance is elapsed when the requested duration has elapsed.
    pub fn is_elapsed(&self) -> bool {
        panic!("STUB: not implemented");
    }
    /// Resets the `Sleep` instance to a new deadline.
    ///
    /// Calling this function allows changing the instant at which the `Sleep`
    /// future completes without having to create new associated state.
    ///
    /// This function can be called both before and after the future has
    /// completed.
    ///
    /// To call this method, you will usually combine the call with
    /// [`Pin::as_mut`], which lets you call the method without consuming the
    /// `Sleep` itself.
    ///
    /// # Example
    ///
    /// ```
    /// use tokio::time::{Duration, Instant};
    ///
    /// # #[tokio::main(flavor = "current_thread")]
    /// # async fn main() {
    /// let sleep = tokio::time::sleep(Duration::from_millis(10));
    /// tokio::pin!(sleep);
    ///
    /// sleep.as_mut().reset(Instant::now() + Duration::from_millis(20));
    /// # }
    /// ```
    ///
    /// See also the top-level examples.
    ///
    /// [`Pin::as_mut`]: fn@std::pin::Pin::as_mut
    pub fn reset(self: Pin<&mut Self>, deadline: Instant) {
        panic!("STUB: not implemented");
    }
    /// Resets the `Sleep` instance to a new deadline.
    ///
    /// Unlike [`reset`][Self::reset], this __removes__ the internal timer.
    pub(super) fn reset_without_timer(self: Pin<&mut Self>, deadline: Instant) {
        panic!("STUB: not implemented");
    }
    fn poll_elapsed(
        self: Pin<&mut Self>,
        cx: &mut task::Context<'_>,
    ) -> Poll<Result<(), Error>> {
        panic!("STUB: not implemented");
    }
}
impl Future for Sleep {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        panic!("STUB: not implemented");
    }
}
