//! A shutdown channel.
//!
//! Each worker holds the `Sender` half. When all the `Sender` halves are
//! dropped, the `Receiver` receives a notification.
use crate::loom::sync::Arc;
use crate::sync::oneshot;
use std::time::Duration;
#[derive(Debug, Clone)]
pub(super) struct Sender {
    _tx: Arc<oneshot::Sender<()>>,
}
#[derive(Debug)]
pub(super) struct Receiver {
    rx: oneshot::Receiver<()>,
}
pub(super) fn channel() -> (Sender, Receiver) {
    panic!("STUB: not implemented");
}
impl Receiver {
    /// Blocks the current thread until all `Sender` handles drop.
    ///
    /// If `timeout` is `Some`, the thread is blocked for **at most** `timeout`
    /// duration. If `timeout` is `None`, then the thread is blocked until the
    /// shutdown signal is received.
    ///
    /// If the timeout has elapsed, it returns `false`, otherwise it returns `true`.
    pub(crate) fn wait(&mut self, timeout: Option<Duration>) -> bool {
        panic!("STUB: not implemented");
    }
}
