cfg_signal_internal_and_unix! {
    mod signal;
}
cfg_io_uring! {
    mod uring; use uring::UringContext; use crate ::sync::OnceCell;
}
use crate::io::interest::Interest;
use crate::io::ready::Ready;
use crate::loom::sync::Mutex;
use crate::runtime::driver;
use crate::runtime::io::registration_set;
use crate::runtime::io::{IoDriverMetrics, RegistrationSet, ScheduledIo};
use mio::event::Source;
use std::fmt;
use std::io;
use std::sync::Arc;
use std::time::Duration;
/// I/O driver, backed by Mio.
pub(crate) struct Driver {
    /// True when an event with the signal token is received
    signal_ready: bool,
    /// Reuse the `mio::Events` value across calls to poll.
    events: mio::Events,
    /// The system event queue.
    poll: mio::Poll,
}
/// A reference to an I/O driver.
pub(crate) struct Handle {
    /// Registers I/O resources.
    registry: mio::Registry,
    /// Tracks all registrations
    registrations: RegistrationSet,
    /// State that should be synchronized
    synced: Mutex<registration_set::Synced>,
    /// Used to wake up the reactor from a call to `turn`.
    /// Not supported on `Wasi` due to lack of threading support.
    #[cfg(not(target_os = "wasi"))]
    waker: mio::Waker,
    pub(crate) metrics: IoDriverMetrics,
    #[cfg(
        all(
            tokio_unstable,
            feature = "io-uring",
            feature = "rt",
            feature = "fs",
            target_os = "linux",
        )
    )]
    pub(crate) uring_context: Mutex<UringContext>,
    #[cfg(
        all(
            tokio_unstable,
            feature = "io-uring",
            feature = "rt",
            feature = "fs",
            target_os = "linux",
        )
    )]
    pub(crate) uring_probe: OnceCell<Option<io_uring::Probe>>,
}
#[derive(Debug)]
pub(crate) struct ReadyEvent {
    pub(super) tick: u8,
    pub(crate) ready: Ready,
    pub(super) is_shutdown: bool,
}
cfg_net_unix!(
    impl ReadyEvent { pub (crate) fn with_ready(& self, ready : Ready) -> Self { Self {
    ready, tick : self.tick, is_shutdown : self.is_shutdown, } } }
);
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub(super) enum Direction {
    Read,
    Write,
}
pub(super) enum Tick {
    Set,
    Clear(u8),
}
const TOKEN_WAKEUP: mio::Token = mio::Token(0);
const TOKEN_SIGNAL: mio::Token = mio::Token(1);
fn _assert_kinds() {
    panic!("STUB: not implemented");
}
impl Driver {
    /// Creates a new event loop, returning any error that happened during the
    /// creation.
    pub(crate) fn new(nevents: usize) -> io::Result<(Driver, Handle)> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn park(&mut self, rt_handle: &driver::Handle) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn park_timeout(
        &mut self,
        rt_handle: &driver::Handle,
        duration: Duration,
    ) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn shutdown(&mut self, rt_handle: &driver::Handle) {
        panic!("STUB: not implemented");
    }
    fn turn(&mut self, handle: &Handle, max_wait: Option<Duration>) {
        panic!("STUB: not implemented");
    }
}
impl fmt::Debug for Driver {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl Handle {
    /// Forces a reactor blocked in a call to `turn` to wakeup, or otherwise
    /// makes the next call to `turn` return immediately.
    ///
    /// This method is intended to be used in situations where a notification
    /// needs to otherwise be sent to the main reactor. If the reactor is
    /// currently blocked inside of `turn` then it will wake up and soon return
    /// after this method has been called. If the reactor is not currently
    /// blocked in `turn`, then the next call to `turn` will not block and
    /// return immediately.
    pub(crate) fn unpark(&self) {
        panic!("STUB: not implemented");
    }
    /// Registers an I/O resource with the reactor for a given `mio::Ready` state.
    ///
    /// The registration token is returned.
    pub(super) fn add_source(
        &self,
        source: &mut impl mio::event::Source,
        interest: Interest,
    ) -> io::Result<Arc<ScheduledIo>> {
        panic!("STUB: not implemented");
    }
    /// Deregisters an I/O resource from the reactor.
    pub(super) fn deregister_source(
        &self,
        registration: &Arc<ScheduledIo>,
        source: &mut impl Source,
    ) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
    fn release_pending_registrations(&self) {
        panic!("STUB: not implemented");
    }
}
impl fmt::Debug for Handle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl Direction {
    pub(super) fn mask(self) -> Ready {
        panic!("STUB: not implemented");
    }
}
