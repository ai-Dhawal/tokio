#![cfg_attr(not(feature = "net"), allow(dead_code))]
use crate::io::interest::Interest;
use crate::runtime::io::{Direction, Handle, ReadyEvent, ScheduledIo};
use crate::runtime::scheduler;
use mio::event::Source;
use std::io;
use std::sync::Arc;
use std::task::{ready, Context, Poll};
cfg_io_driver! {
    #[doc = " Associates an I/O resource with the reactor instance that drives it."]
    #[doc = ""] #[doc =
    " A registration represents an I/O resource registered with a Reactor such"] #[doc =
    " that it will receive task notifications on readiness. This is the lowest"] #[doc =
    " level API for integrating with a reactor."] #[doc = ""] #[doc =
    " The association between an I/O resource is made by calling"] #[doc =
    " [`new_with_interest_and_handle`]."] #[doc =
    " Once the association is established, it remains established until the"] #[doc =
    " registration instance is dropped."] #[doc = ""] #[doc =
    " A registration instance represents two separate readiness streams. One"] #[doc =
    " for the read readiness and one for write readiness. These streams are"] #[doc =
    " independent and can be consumed from separate tasks."] #[doc = ""] #[doc =
    " **Note**: while `Registration` is `Sync`, the caller must ensure that"] #[doc =
    " there are at most two tasks that use a registration instance"] #[doc =
    " concurrently. One task for [`poll_read_ready`] and one task for"] #[doc =
    " [`poll_write_ready`]. While violating this requirement is \"safe\" from a"] #[doc =
    " Rust memory safety point of view, it will result in unexpected behavior"] #[doc =
    " in the form of lost notifications and tasks hanging."] #[doc = ""] #[doc =
    " ## Platform-specific events"] #[doc = ""] #[doc =
    " `Registration` also allows receiving platform-specific `mio::Ready`"] #[doc =
    " events. These events are included as part of the read readiness event"] #[doc =
    " stream. The write readiness event stream is only for `Ready::writable()`"] #[doc =
    " events."] #[doc = ""] #[doc =
    " [`new_with_interest_and_handle`]: method@Self::new_with_interest_and_handle"] #[doc
    = " [`poll_read_ready`]: method@Self::poll_read_ready`"] #[doc =
    " [`poll_write_ready`]: method@Self::poll_write_ready`"] #[derive(Debug)] pub (crate)
    struct Registration { #[doc = " Handle to the associated runtime."] #[doc = ""] #[doc
    = " TODO: this can probably be moved into `ScheduledIo`."] handle :
    scheduler::Handle, #[doc = " Reference to state stored by the driver."] shared : Arc
    < ScheduledIo >, }
}
unsafe impl Send for Registration {}
unsafe impl Sync for Registration {}
impl Registration {
    /// Registers the I/O resource with the reactor for the provided handle, for
    /// a specific `Interest`. This does not add `hup` or `error` so if you are
    /// interested in those states, you will need to add them to the readiness
    /// state passed to this function.
    ///
    /// # Return
    ///
    /// - `Ok` if the registration happened successfully
    /// - `Err` if an error was encountered during registration
    #[track_caller]
    pub(crate) fn new_with_interest_and_handle(
        io: &mut impl Source,
        interest: Interest,
        handle: scheduler::Handle,
    ) -> io::Result<Registration> {
        panic!("STUB: not implemented");
    }
    /// Deregisters the I/O resource from the reactor it is associated with.
    ///
    /// This function must be called before the I/O resource associated with the
    /// registration is dropped.
    ///
    /// Note that deregistering does not guarantee that the I/O resource can be
    /// registered with a different reactor. Some I/O resource types can only be
    /// associated with a single reactor instance for their lifetime.
    ///
    /// # Return
    ///
    /// If the deregistration was successful, `Ok` is returned. Any calls to
    /// `Reactor::turn` that happen after a successful call to `deregister` will
    /// no longer result in notifications getting sent for this registration.
    ///
    /// `Err` is returned if an error is encountered.
    pub(crate) fn deregister(&mut self, io: &mut impl Source) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn clear_readiness(&self, event: ReadyEvent) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn poll_read_ready(
        &self,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<ReadyEvent>> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn poll_write_ready(
        &self,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<ReadyEvent>> {
        panic!("STUB: not implemented");
    }
    #[cfg(not(all(target_os = "wasi", target_env = "p1")))]
    pub(crate) fn poll_read_io<R>(
        &self,
        cx: &mut Context<'_>,
        f: impl FnMut() -> io::Result<R>,
    ) -> Poll<io::Result<R>> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn poll_write_io<R>(
        &self,
        cx: &mut Context<'_>,
        f: impl FnMut() -> io::Result<R>,
    ) -> Poll<io::Result<R>> {
        panic!("STUB: not implemented");
    }
    /// Polls for events on the I/O resource's `direction` readiness stream.
    ///
    /// If called with a task context, notify the task when a new event is
    /// received.
    fn poll_ready(
        &self,
        cx: &mut Context<'_>,
        direction: Direction,
    ) -> Poll<io::Result<ReadyEvent>> {
        panic!("STUB: not implemented");
    }
    fn poll_io<R>(
        &self,
        cx: &mut Context<'_>,
        direction: Direction,
        mut f: impl FnMut() -> io::Result<R>,
    ) -> Poll<io::Result<R>> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn try_io<R>(
        &self,
        interest: Interest,
        f: impl FnOnce() -> io::Result<R>,
    ) -> io::Result<R> {
        panic!("STUB: not implemented");
    }
    pub(crate) async fn readiness(&self, interest: Interest) -> io::Result<ReadyEvent> {
        panic!("STUB: not implemented");
    }
    pub(crate) async fn async_io<R>(
        &self,
        interest: Interest,
        mut f: impl FnMut() -> io::Result<R>,
    ) -> io::Result<R> {
        panic!("STUB: not implemented");
    }
    fn handle(&self) -> &Handle {
        panic!("STUB: not implemented");
    }
}
impl Drop for Registration {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
fn gone() -> io::Error {
    panic!("STUB: not implemented");
}
