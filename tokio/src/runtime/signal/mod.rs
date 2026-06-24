#![cfg_attr(not(feature = "rt"), allow(dead_code))]
//! Signal driver
use crate::runtime::{driver, io};
use crate::signal::registry::globals;
use mio::net::UnixStream;
use std::io::{self as std_io, Read};
use std::sync::{Arc, Weak};
use std::time::Duration;
/// Responsible for registering wakeups when an OS signal is received, and
/// subsequently dispatching notifications to any signal listeners as appropriate.
///
/// Note: this driver relies on having an enabled IO driver in order to listen to
/// pipe write wakeups.
#[derive(Debug)]
pub(crate) struct Driver {
    /// Thread parker. The `Driver` park implementation delegates to this.
    io: io::Driver,
    /// A pipe for receiving wake events from the signal handler
    receiver: UnixStream,
    /// Shared state. The driver keeps a strong ref and the handle keeps a weak
    /// ref. The weak ref is used to check if the driver is still active before
    /// trying to register a signal handler.
    inner: Arc<()>,
}
#[derive(Debug, Default)]
pub(crate) struct Handle {
    /// Paired w/ the `Arc` above and is used to check if the driver is still
    /// around before attempting to register a signal handler.
    inner: Weak<()>,
}
impl Driver {
    /// Creates a new signal `Driver` instance that delegates wakeups to `park`.
    pub(crate) fn new(io: io::Driver, io_handle: &io::Handle) -> std_io::Result<Self> {
        panic!("STUB: not implemented");
    }
    /// Returns a handle to this event loop which can be sent across threads
    /// and can be used as a proxy to the event loop itself.
    pub(crate) fn handle(&self) -> Handle {
        panic!("STUB: not implemented");
    }
    pub(crate) fn park(&mut self, handle: &driver::Handle) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn park_timeout(&mut self, handle: &driver::Handle, duration: Duration) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn shutdown(&mut self, handle: &driver::Handle) {
        panic!("STUB: not implemented");
    }
    fn process(&mut self) {
        panic!("STUB: not implemented");
    }
}
impl Handle {
    pub(crate) fn check_inner(&self) -> std_io::Result<()> {
        panic!("STUB: not implemented");
    }
}
