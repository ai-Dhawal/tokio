use crate::io::blocking::Buf;
use crate::io::uring::utils::{ArcFd, UringFd};
use crate::runtime::driver::op::{CancelData, Cancellable, Completable, CqeResult, Op};
use io_uring::{opcode, types};
use std::fmt;
use std::io::{self, Error};
use std::os::fd::OwnedFd;
/// Trait for buffers that can be used with io-uring read operations.
pub(crate) trait ReadBuffer: Send + 'static {
    /// Prepare the buffer for a read operation.
    /// Returns a pointer and length for the io-uring SQE.
    fn uring_read_prepare(&mut self, max_len: usize) -> (*mut u8, u32);
    /// Complete a read of `n` bytes.
    ///
    /// # Safety
    ///
    /// The caller must ensure the kernel wrote exactly `n` bytes
    /// into the buffer at the pointer returned by `uring_read_prepare`.
    unsafe fn uring_read_complete(&mut self, n: u32);
}
impl ReadBuffer for Vec<u8> {
    fn uring_read_prepare(&mut self, max_len: usize) -> (*mut u8, u32) {
        panic!("STUB: not implemented");
    }
    unsafe fn uring_read_complete(&mut self, n: u32) {
        panic!("STUB: not implemented");
    }
}
impl ReadBuffer for Buf {
    fn uring_read_prepare(&mut self, max_len: usize) -> (*mut u8, u32) {
        panic!("STUB: not implemented");
    }
    unsafe fn uring_read_complete(&mut self, n: u32) {
        panic!("STUB: not implemented");
    }
}
pub(crate) struct Read<B, F = ArcFd> {
    fd: F,
    buf: B,
}
impl<B: fmt::Debug, F> fmt::Debug for Read<B, F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl<B: ReadBuffer, F: UringFd> Completable for Read<B, F> {
    type Output = (io::Result<u32>, F, B);
    fn complete(self, cqe: CqeResult) -> Self::Output {
        panic!("STUB: not implemented");
    }
    fn complete_with_error(self, err: Error) -> Self::Output {
        panic!("STUB: not implemented");
    }
}
impl Cancellable for Read<Vec<u8>, OwnedFd> {
    fn cancel(self) -> CancelData {
        panic!("STUB: not implemented");
    }
}
impl Cancellable for Read<Buf, ArcFd> {
    fn cancel(self) -> CancelData {
        panic!("STUB: not implemented");
    }
}
impl<B, F> Op<Read<B, F>>
where
    B: ReadBuffer + fmt::Debug,
    F: UringFd,
    Read<B, F>: Cancellable,
{
    /// Submit a read operation via io-uring.
    ///
    /// `max_len` is the maximum number of bytes to read.
    /// `offset` is the file offset; use `u64::MAX` for the current cursor.
    pub(crate) fn read_at(fd: F, mut buf: B, max_len: usize, offset: u64) -> Self {
        panic!("STUB: not implemented");
    }
}
