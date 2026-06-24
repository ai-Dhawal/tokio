use crate::io::sys;
use crate::io::{AsyncRead, AsyncWrite, ReadBuf};
use std::cmp;
use std::future::Future;
use std::io;
use std::io::prelude::*;
use std::mem::MaybeUninit;
use std::pin::Pin;
use std::task::{ready, Context, Poll};
/// `T` should not implement _both_ Read and Write.
#[derive(Debug)]
pub(crate) struct Blocking<T> {
    inner: Option<T>,
    state: State<T>,
    /// `true` if the lower IO layer needs flushing.
    need_flush: bool,
}
#[derive(Debug)]
pub(crate) struct Buf {
    buf: Vec<u8>,
    pos: usize,
}
pub(crate) const DEFAULT_MAX_BUF_SIZE: usize = 2 * 1024 * 1024;
#[derive(Debug)]
enum State<T> {
    Idle(Option<Buf>),
    Busy(sys::Blocking<(io::Result<usize>, Buf, T)>),
}
cfg_io_blocking! {
    impl < T > Blocking < T > { #[doc = " # Safety"] #[doc = ""] #[doc =
    " The `Read` implementation of `inner` must never read from the buffer"] #[doc =
    " it is borrowing and must correctly report the length of the data"] #[doc =
    " written into the buffer."] #[cfg_attr(feature = "fs", allow(dead_code))] pub
    (crate) unsafe fn new(inner : T) -> Blocking < T > { Blocking { inner : Some(inner),
    state : State::Idle(Some(Buf::with_capacity(0))), need_flush : false, } } }
}
impl<T> AsyncRead for Blocking<T>
where
    T: Read + Unpin + Send + 'static,
{
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        dst: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        panic!("STUB: not implemented");
    }
}
impl<T> AsyncWrite for Blocking<T>
where
    T: Write + Unpin + Send + 'static,
{
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        src: &[u8],
    ) -> Poll<io::Result<usize>> {
        panic!("STUB: not implemented");
    }
    fn poll_flush(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), io::Error>> {
        panic!("STUB: not implemented");
    }
    fn poll_shutdown(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Result<(), io::Error>> {
        panic!("STUB: not implemented");
    }
}
/// Repeats operations that are interrupted.
macro_rules! uninterruptibly {
    ($e:expr) => {
        { loop { match $e { Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
        res => break res, } } }
    };
}
impl Buf {
    pub(crate) fn with_capacity(n: usize) -> Buf {
        panic!("STUB: not implemented");
    }
    pub(crate) fn is_empty(&self) -> bool {
        panic!("STUB: not implemented");
    }
    pub(crate) fn len(&self) -> usize {
        panic!("STUB: not implemented");
    }
    pub(crate) fn copy_to(&mut self, dst: &mut ReadBuf<'_>) -> usize {
        panic!("STUB: not implemented");
    }
    pub(crate) fn copy_from(&mut self, src: &[u8], max_buf_size: usize) -> usize {
        panic!("STUB: not implemented");
    }
    pub(crate) fn bytes(&self) -> &[u8] {
        panic!("STUB: not implemented");
    }
    /// # Safety
    ///
    /// `rd` must not read from the buffer `read` is borrowing and must correctly
    /// report the length of the data written into the buffer.
    pub(crate) unsafe fn read_from<T: Read>(
        &mut self,
        rd: &mut T,
        max_buf_size: usize,
    ) -> io::Result<usize> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn write_to<T: Write>(&mut self, wr: &mut T) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
}
cfg_io_uring! {
    impl Buf { #[doc = " Prepare the internal buffer for an io-uring read operation."]
    #[doc = ""] #[doc =
    " Returns a pointer to the spare capacity and the length available"] #[doc =
    " for the kernel to write into."] pub (crate) fn prepare_uring_read(& mut self,
    max_buf_size : usize) -> (* mut u8, u32) { assert!(self.is_empty()); self.buf
    .reserve(max_buf_size); let spare = self.buf.spare_capacity_mut(); let len =
    std::cmp::min(spare.len(), max_buf_size); let ptr = spare.as_mut_ptr().cast::< u8 >
    (); (ptr, len as u32) } #[doc = " Complete an io-uring read operation."] #[doc = ""]
    #[doc = " # Safety"] #[doc = ""] #[doc =
    " The caller must ensure that the kernel wrote exactly `n` bytes"] #[doc =
    " into the buffer that was returned by `prepare_uring_read`."] pub (crate) unsafe fn
    complete_uring_read(& mut self, n : usize) { assert_eq!(self.pos, 0); unsafe { self
    .buf.set_len(n) }; } }
}
cfg_fs! {
    impl Buf { pub (crate) fn discard_read(& mut self) -> i64 { let ret = - (self.bytes()
    .len() as i64); self.pos = 0; self.buf.clear(); ret } pub (crate) fn copy_from_bufs(&
    mut self, bufs : & [io::IoSlice <'_ >], max_buf_size : usize) -> usize { assert!(self
    .is_empty()); let mut rem = max_buf_size; for buf in bufs { if rem == 0 { break } let
    len = buf.len().min(rem); self.buf.extend_from_slice(& buf[..len]); rem -= len; }
    max_buf_size - rem } }
}
