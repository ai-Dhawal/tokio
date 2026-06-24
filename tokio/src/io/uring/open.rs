use super::utils::cstr;
use crate::fs::UringOpenOptions;
use crate::runtime::driver::op::{CancelData, Cancellable, Completable, CqeResult, Op};
use io_uring::{opcode, types};
use std::ffi::CString;
use std::io::{self, Error};
use std::os::fd::FromRawFd;
use std::path::Path;
#[derive(Debug)]
pub(crate) struct Open {
    /// This field will be read by the kernel during the operation, so we
    /// need to ensure it is valid for the entire duration of the operation.
    #[allow(dead_code)]
    path: CString,
}
impl Completable for Open {
    type Output = io::Result<crate::fs::File>;
    fn complete(self, cqe: CqeResult) -> Self::Output {
        panic!("STUB: not implemented");
    }
    fn complete_with_error(self, err: Error) -> Self::Output {
        panic!("STUB: not implemented");
    }
}
impl Cancellable for Open {
    fn cancel(self) -> CancelData {
        panic!("STUB: not implemented");
    }
}
impl Op<Open> {
    /// Submit a request to open a file.
    pub(crate) fn open(path: &Path, options: &UringOpenOptions) -> io::Result<Op<Open>> {
        panic!("STUB: not implemented");
    }
}
