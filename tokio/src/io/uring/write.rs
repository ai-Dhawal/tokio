use crate::runtime::driver::op::{CancelData, Cancellable, Completable, CqeResult, Op};
use crate::util::as_ref::OwnedBuf;
use io_uring::{opcode, types};
use std::io::{self, Error};
use std::os::fd::{AsRawFd, OwnedFd};
#[derive(Debug)]
pub(crate) struct Write {
    buf: OwnedBuf,
    fd: OwnedFd,
}
impl Completable for Write {
    type Output = (io::Result<u32>, OwnedBuf, OwnedFd);
    fn complete(self, cqe: CqeResult) -> Self::Output {
        panic!("STUB: not implemented");
    }
    fn complete_with_error(self, err: Error) -> Self::Output {
        panic!("STUB: not implemented");
    }
}
impl Cancellable for Write {
    fn cancel(self) -> CancelData {
        panic!("STUB: not implemented");
    }
}
impl Op<Write> {
    /// Issue a write that starts at `buf_offset` within `buf` and writes some bytes
    /// into `file` at `file_offset`.
    pub(crate) fn write_at(
        fd: OwnedFd,
        buf: OwnedBuf,
        buf_offset: usize,
        file_offset: u64,
    ) -> io::Result<Self> {
        panic!("STUB: not implemented");
    }
}
