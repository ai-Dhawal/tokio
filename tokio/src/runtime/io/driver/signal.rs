use super::{Driver, Handle, TOKEN_SIGNAL};
use std::io;
impl Handle {
    pub(crate) fn register_signal_receiver(
        &self,
        receiver: &mut mio::net::UnixStream,
    ) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
}
impl Driver {
    pub(crate) fn consume_signal_ready(&mut self) -> bool {
        panic!("STUB: not implemented");
    }
}
