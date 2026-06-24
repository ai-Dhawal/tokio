use super::{cancellation_queue, RegistrationQueue, Wheel};
/// Local context for the time driver, used when the runtime wants to
/// fire/cancel timers.
pub(crate) struct LocalContext {
    pub(crate) wheel: Wheel,
    pub(crate) registration_queue: RegistrationQueue,
    pub(crate) canc_tx: cancellation_queue::Sender,
    pub(crate) canc_rx: cancellation_queue::Receiver,
}
impl LocalContext {
    pub(crate) fn new() -> Self {
        panic!("STUB: not implemented");
    }
}
pub(crate) enum TempLocalContext<'a> {
    /// The runtime is running, we can access it.
    Running { registration_queue: &'a mut RegistrationQueue },
    #[cfg(feature = "rt-multi-thread")]
    /// The runtime is shutting down, no timers can be registered.
    Shutdown,
}
impl<'a> TempLocalContext<'a> {
    pub(crate) fn new_running(cx: &'a mut LocalContext) -> Self {
        panic!("STUB: not implemented");
    }
    #[cfg(feature = "rt-multi-thread")]
    pub(crate) fn new_shutdown() -> Self {
        panic!("STUB: not implemented");
    }
}
