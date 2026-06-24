use io_uring::{squeue::Entry, IoUring, Probe};
use mio::unix::SourceFd;
use slab::Slab;
use crate::runtime::driver::op::CancelData;
use crate::runtime::driver::op::CqeResult;
use crate::runtime::driver::op::{Cancellable, Lifecycle};
use crate::{io::Interest, loom::sync::Mutex};
use super::{Handle, TOKEN_WAKEUP};
use std::os::fd::{AsRawFd, FromRawFd, OwnedFd, RawFd};
use std::{io, mem, task::Waker};
const DEFAULT_RING_SIZE: u32 = 256;
pub(crate) struct UringContext {
    pub(crate) uring: Option<io_uring::IoUring>,
    pub(crate) ops: slab::Slab<Lifecycle>,
}
impl UringContext {
    pub(crate) fn new() -> Self {
        panic!("STUB: not implemented");
    }
    pub(crate) fn ring(&self) -> &io_uring::IoUring {
        panic!("STUB: not implemented");
    }
    pub(crate) fn ring_mut(&mut self) -> &mut io_uring::IoUring {
        panic!("STUB: not implemented");
    }
    /// Perform `io_uring_setup` system call, and Returns true if this
    /// actually initialized the io_uring.
    ///
    /// If the machine doesn't support io_uring, then this will return an
    /// `ENOSYS` error.
    pub(crate) fn try_init(&mut self, probe: &mut Probe) -> io::Result<bool> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn dispatch_completions(&mut self) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn submit(&mut self) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn remove_op(&mut self, index: usize) -> Lifecycle {
        panic!("STUB: not implemented");
    }
}
/// Drop the driver, cancelling any in-progress ops and waiting for them to terminate.
impl Drop for UringContext {
    fn drop(&mut self) {
        panic!("STUB: not implemented");
    }
}
impl Handle {
    fn add_uring_source(&self, uringfd: RawFd) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn get_uring(&self) -> &Mutex<UringContext> {
        panic!("STUB: not implemented");
    }
    /// Returns `true` if io_uring has already been initialized and the given
    /// opcode is supported. Returns `false` if io_uring hasn't been
    /// initialized yet or is unsupported. Unlike `check_and_init`, this
    /// doesn't attempt initialization.
    #[cfg_attr(test, allow(dead_code))]
    pub(crate) fn is_uring_ready(&self, opcode: u8) -> bool {
        panic!("STUB: not implemented");
    }
    /// Returns `true` if the io_uring probe has already been attempted
    /// (regardless of whether io_uring is supported). Returns `false` if
    /// no probe has been attempted yet.
    #[cfg_attr(test, allow(dead_code))]
    pub(crate) fn is_uring_probed(&self) -> bool {
        panic!("STUB: not implemented");
    }
    /// Check if the io_uring context is initialized. If not, it will try to initialize it.
    /// Then, check if the provided opcode is supported.
    ///
    /// If both the context initialization succeeds and the opcode is supported,
    /// this returns `Ok(true)`.
    /// If either io_uring is unsupported or the opcode is unsupported,
    /// this returns `Ok(false)`.
    /// An error is returned if an io_uring syscall returns an unexpected error value.
    ///
    /// TODO: This would like to be a synchronous function,
    /// but we require `OnceLock::get_or_try_init`.
    /// <https://github.com/rust-lang/rust/issues/109737>
    pub(crate) async fn check_and_init(&self, opcode: u8) -> io::Result<bool> {
        panic!("STUB: not implemented");
    }
    /// Initialize the io_uring context if it hasn't been initialized yet.
    fn try_init(&self, probe: &mut Probe) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
    /// Register an operation with the io_uring.
    ///
    /// If this is the first io_uring operation, it will also initialize the io_uring context.
    /// If io_uring isn't supported, this function returns an `ENOSYS` error, so the caller can
    /// perform custom handling, such as falling back to an alternative mechanism.
    ///
    /// # Safety
    ///
    /// Callers must ensure that parameters of the entry (such as buffer) are valid and will
    /// be valid for the entire duration of the operation, otherwise it may cause memory problems.
    pub(crate) unsafe fn register_op(
        &self,
        entry: Entry,
        waker: Waker,
    ) -> io::Result<usize> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn cancel_op<T: Cancellable>(&self, index: usize, data: Option<T>) {
        panic!("STUB: not implemented");
    }
}
