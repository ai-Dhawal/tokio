//! Split a single value implementing `AsyncRead + AsyncWrite` into separate
//! `AsyncRead` and `AsyncWrite` handles.
//!
//! To restore this read/write object from its `split::ReadHalf` and
//! `split::WriteHalf` use `unsplit`.
use crate::io::{AsyncRead, AsyncWrite, ReadBuf};
use std::fmt;
use std::io;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::Mutex;
use std::task::{Context, Poll};
cfg_io_util! {
    #[doc = " The readable half of a value returned from [`split`](split())."] pub struct
    ReadHalf < T > { inner : Arc < Inner < T >>, } #[doc =
    " The writable half of a value returned from [`split`](split())."] pub struct
    WriteHalf < T > { inner : Arc < Inner < T >>, } #[doc =
    " Splits a single value implementing `AsyncRead + AsyncWrite` into separate"] #[doc =
    " `AsyncRead` and `AsyncWrite` handles."] #[doc = ""] #[doc =
    " To restore this read/write object from its `ReadHalf` and"] #[doc =
    " `WriteHalf` use [`unsplit`](ReadHalf::unsplit())."] pub fn split < T > (stream : T)
    -> (ReadHalf < T >, WriteHalf < T >) where T : AsyncRead + AsyncWrite, { let
    is_write_vectored = stream.is_write_vectored(); let inner = Arc::new(Inner { stream :
    Mutex::new(stream), is_write_vectored, }); let rd = ReadHalf { inner : inner.clone(),
    }; let wr = WriteHalf { inner }; (rd, wr) }
}
struct Inner<T> {
    stream: Mutex<T>,
    is_write_vectored: bool,
}
impl<T> Inner<T> {
    fn with_lock<R>(&self, f: impl FnOnce(Pin<&mut T>) -> R) -> R {
        panic!("STUB: not implemented");
    }
}
impl<T> ReadHalf<T> {
    /// Checks if this `ReadHalf` and some `WriteHalf` were split from the same
    /// stream.
    pub fn is_pair_of(&self, other: &WriteHalf<T>) -> bool {
        panic!("STUB: not implemented");
    }
    /// Reunites with a previously split `WriteHalf`.
    ///
    /// # Panics
    ///
    /// If this `ReadHalf` and the given `WriteHalf` do not originate from the
    /// same `split` operation this method will panic.
    /// This can be checked ahead of time by calling [`is_pair_of()`](Self::is_pair_of).
    #[track_caller]
    pub fn unsplit(self, wr: WriteHalf<T>) -> T
    where
        T: Unpin,
    {
        panic!("STUB: not implemented");
    }
}
impl<T> WriteHalf<T> {
    /// Checks if this `WriteHalf` and some `ReadHalf` were split from the same
    /// stream.
    pub fn is_pair_of(&self, other: &ReadHalf<T>) -> bool {
        panic!("STUB: not implemented");
    }
}
impl<T: AsyncRead> AsyncRead for ReadHalf<T> {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        panic!("STUB: not implemented");
    }
}
impl<T: AsyncWrite> AsyncWrite for WriteHalf<T> {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        panic!("STUB: not implemented");
    }
    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), io::Error>> {
        panic!("STUB: not implemented");
    }
    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), io::Error>> {
        panic!("STUB: not implemented");
    }
    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[io::IoSlice<'_>],
    ) -> Poll<Result<usize, io::Error>> {
        panic!("STUB: not implemented");
    }
    fn is_write_vectored(&self) -> bool {
        panic!("STUB: not implemented");
    }
}
unsafe impl<T: Send> Send for ReadHalf<T> {}
unsafe impl<T: Send> Send for WriteHalf<T> {}
unsafe impl<T: Sync> Sync for ReadHalf<T> {}
unsafe impl<T: Sync> Sync for WriteHalf<T> {}
impl<T: fmt::Debug> fmt::Debug for ReadHalf<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl<T: fmt::Debug> fmt::Debug for WriteHalf<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
