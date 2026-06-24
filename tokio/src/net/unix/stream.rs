use crate::io::{AsyncRead, AsyncWrite, Interest, PollEvented, ReadBuf, Ready};
use crate::net::unix::split::{split, ReadHalf, WriteHalf};
use crate::net::unix::split_owned::{split_owned, OwnedReadHalf, OwnedWriteHalf};
use crate::net::unix::ucred::{self, UCred};
use crate::net::unix::SocketAddr;
use crate::util::check_socket_for_blocking;
use std::fmt;
use std::future::poll_fn;
use std::io::{self, Read, Write};
use std::net::Shutdown;
#[cfg(target_os = "android")]
use std::os::android::net::SocketAddrExt;
#[cfg(target_os = "linux")]
use std::os::linux::net::SocketAddrExt;
#[cfg(any(target_os = "linux", target_os = "android"))]
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, RawFd};
use std::os::unix::net::{self, SocketAddr as StdSocketAddr};
use std::path::Path;
use std::pin::Pin;
use std::task::{Context, Poll};
cfg_io_util! {
    use bytes::BufMut;
}
cfg_net_unix! {
    #[doc = " A structure representing a connected Unix socket."] #[doc = ""] #[doc =
    " This socket can be connected directly with [`UnixStream::connect`] or accepted"]
    #[doc = " from a listener with [`UnixListener::accept`]. Additionally, a pair of"]
    #[doc = " anonymous Unix sockets can be created with `UnixStream::pair`."] #[doc =
    ""] #[doc = " To shut down the stream in the write direction, you can call the"]
    #[doc =
    " [`shutdown()`] method. This will cause the other peer to receive a read of"] #[doc
    = " length 0, indicating that no more data will be sent. This only closes"] #[doc =
    " the stream in one direction."] #[doc = ""] #[doc =
    " [`shutdown()`]: fn@crate::io::AsyncWriteExt::shutdown"] #[doc =
    " [`UnixListener::accept`]: crate::net::UnixListener::accept"] #[cfg_attr(docsrs,
    doc(alias = "uds"))] pub struct UnixStream { io : PollEvented < mio::net::UnixStream
    >, }
}
impl UnixStream {
    pub(crate) async fn connect_mio(
        sys: mio::net::UnixStream,
    ) -> io::Result<UnixStream> {
        panic!("STUB: not implemented");
    }
    /// Connects to the socket named by `path`.
    ///
    /// This function will create a new Unix socket and connect to the path
    /// specified, associating the returned stream with the default event loop's
    /// handle.
    ///
    /// # Panics
    ///
    /// This function panics if it is not called from within a runtime with
    /// IO enabled.
    ///
    /// The runtime is usually set implicitly when this function is called
    /// from a future driven by a tokio runtime, otherwise runtime can be set
    /// explicitly with [`Runtime::enter`](crate::runtime::Runtime::enter) function.
    pub async fn connect<P>(path: P) -> io::Result<UnixStream>
    where
        P: AsRef<Path>,
    {
        panic!("STUB: not implemented");
    }
    /// Connects to the socket named by `socket_addr`.
    ///
    /// This function will create a new Unix socket and connect to the address
    /// specified, associating the returned stream with the default event
    /// loop's handle.
    ///
    /// # Panics
    ///
    /// This function panics if it is not called from within a runtime with
    /// IO enabled.
    ///
    /// The runtime is usually set implicitly when this function is called
    /// from a future driven by a tokio runtime, otherwise runtime can be set
    /// explicitly with [`Runtime::enter`](crate::runtime::Runtime::enter) function.
    pub async fn connect_addr(socket_addr: &SocketAddr) -> io::Result<UnixStream> {
        panic!("STUB: not implemented");
    }
    /// Waits for any of the requested ready states.
    ///
    /// This function is usually paired with `try_read()` or `try_write()`. It
    /// can be used to concurrently read / write to the same socket on a single
    /// task without splitting the socket.
    ///
    /// The function may complete without the socket being ready. This is a
    /// false-positive and attempting an operation will return with
    /// `io::ErrorKind::WouldBlock`. The function can also return with an empty
    /// [`Ready`] set, so you should always check the returned value and possibly
    /// wait again if the requested states are not set.
    ///
    /// # Cancel safety
    ///
    /// This method is cancel safe. Once a readiness event occurs, the method
    /// will continue to return immediately until the readiness event is
    /// consumed by an attempt to read or write that fails with `WouldBlock` or
    /// `Poll::Pending`.
    ///
    /// # Examples
    ///
    /// Concurrently read and write to the stream on the same task without
    /// splitting.
    ///
    /// ```no_run
    /// use tokio::io::Interest;
    /// use tokio::net::UnixStream;
    /// use std::error::Error;
    /// use std::io;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     let dir = tempfile::tempdir().unwrap();
    ///     let bind_path = dir.path().join("bind_path");
    ///     let stream = UnixStream::connect(bind_path).await?;
    ///
    ///     loop {
    ///         let ready = stream.ready(Interest::READABLE | Interest::WRITABLE).await?;
    ///
    ///         if ready.is_readable() {
    ///             let mut data = vec![0; 1024];
    ///             // Try to read data, this may still fail with `WouldBlock`
    ///             // if the readiness event is a false positive.
    ///             match stream.try_read(&mut data) {
    ///                 Ok(n) => {
    ///                     println!("read {} bytes", n);
    ///                 }
    ///                 Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
    ///                     continue;
    ///                 }
    ///                 Err(e) => {
    ///                     return Err(e.into());
    ///                 }
    ///             }
    ///
    ///         }
    ///
    ///         if ready.is_writable() {
    ///             // Try to write data, this may still fail with `WouldBlock`
    ///             // if the readiness event is a false positive.
    ///             match stream.try_write(b"hello world") {
    ///                 Ok(n) => {
    ///                     println!("write {} bytes", n);
    ///                 }
    ///                 Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
    ///                     continue;
    ///                 }
    ///                 Err(e) => {
    ///                     return Err(e.into());
    ///                 }
    ///             }
    ///         }
    ///     }
    /// }
    /// ```
    pub async fn ready(&self, interest: Interest) -> io::Result<Ready> {
        panic!("STUB: not implemented");
    }
    /// Waits for the socket to become readable.
    ///
    /// This function is equivalent to `ready(Interest::READABLE)` and is usually
    /// paired with `try_read()`.
    ///
    /// # Cancel safety
    ///
    /// This method is cancel safe. Once a readiness event occurs, the method
    /// will continue to return immediately until the readiness event is
    /// consumed by an attempt to read that fails with `WouldBlock` or
    /// `Poll::Pending`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::net::UnixStream;
    /// use std::error::Error;
    /// use std::io;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     // Connect to a peer
    ///     let dir = tempfile::tempdir().unwrap();
    ///     let bind_path = dir.path().join("bind_path");
    ///     let stream = UnixStream::connect(bind_path).await?;
    ///
    ///     let mut msg = vec![0; 1024];
    ///
    ///     loop {
    ///         // Wait for the socket to be readable
    ///         stream.readable().await?;
    ///
    ///         // Try to read data, this may still fail with `WouldBlock`
    ///         // if the readiness event is a false positive.
    ///         match stream.try_read(&mut msg) {
    ///             Ok(n) => {
    ///                 msg.truncate(n);
    ///                 break;
    ///             }
    ///             Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
    ///                 continue;
    ///             }
    ///             Err(e) => {
    ///                 return Err(e.into());
    ///             }
    ///         }
    ///     }
    ///
    ///     println!("GOT = {:?}", msg);
    ///     Ok(())
    /// }
    /// ```
    pub async fn readable(&self) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
    /// Polls for read readiness.
    ///
    /// If the unix stream is not currently ready for reading, this method will
    /// store a clone of the `Waker` from the provided `Context`. When the unix
    /// stream becomes ready for reading, `Waker::wake` will be called on the
    /// waker.
    ///
    /// Note that on multiple calls to `poll_read_ready` or `poll_read`, only
    /// the `Waker` from the `Context` passed to the most recent call is
    /// scheduled to receive a wakeup. (However, `poll_write_ready` retains a
    /// second, independent waker.)
    ///
    /// This function is intended for cases where creating and pinning a future
    /// via [`readable`] is not feasible. Where possible, using [`readable`] is
    /// preferred, as this supports polling from multiple tasks at once.
    ///
    /// # Return value
    ///
    /// The function returns:
    ///
    /// * `Poll::Pending` if the unix stream is not ready for reading.
    /// * `Poll::Ready(Ok(()))` if the unix stream is ready for reading.
    /// * `Poll::Ready(Err(e))` if an error is encountered.
    ///
    /// # Errors
    ///
    /// This function may encounter any standard I/O error except `WouldBlock`.
    ///
    /// [`readable`]: method@Self::readable
    pub fn poll_read_ready(&self, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        panic!("STUB: not implemented");
    }
    /// Try to read data from the stream into the provided buffer, returning how
    /// many bytes were read.
    ///
    /// Receives any pending data from the socket but does not wait for new data
    /// to arrive. On success, returns the number of bytes read. Because
    /// `try_read()` is non-blocking, the buffer does not have to be stored by
    /// the async task and can exist entirely on the stack.
    ///
    /// Usually, [`readable()`] or [`ready()`] is used with this function.
    ///
    /// [`readable()`]: UnixStream::readable()
    /// [`ready()`]: UnixStream::ready()
    ///
    /// # Return
    ///
    /// If data is successfully read, `Ok(n)` is returned, where `n` is the
    /// number of bytes read. If `n` is `0`, then it can indicate one of two scenarios:
    ///
    /// 1. The stream's read half is closed and will no longer yield data.
    /// 2. The specified buffer was 0 bytes in length.
    ///
    /// If the stream is not ready to read data,
    /// `Err(io::ErrorKind::WouldBlock)` is returned.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::net::UnixStream;
    /// use std::error::Error;
    /// use std::io;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     // Connect to a peer
    ///     let dir = tempfile::tempdir().unwrap();
    ///     let bind_path = dir.path().join("bind_path");
    ///     let stream = UnixStream::connect(bind_path).await?;
    ///
    ///     loop {
    ///         // Wait for the socket to be readable
    ///         stream.readable().await?;
    ///
    ///         // Creating the buffer **after** the `await` prevents it from
    ///         // being stored in the async task.
    ///         let mut buf = [0; 4096];
    ///
    ///         // Try to read data, this may still fail with `WouldBlock`
    ///         // if the readiness event is a false positive.
    ///         match stream.try_read(&mut buf) {
    ///             Ok(0) => break,
    ///             Ok(n) => {
    ///                 println!("read {} bytes", n);
    ///             }
    ///             Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
    ///                 continue;
    ///             }
    ///             Err(e) => {
    ///                 return Err(e.into());
    ///             }
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn try_read(&self, buf: &mut [u8]) -> io::Result<usize> {
        panic!("STUB: not implemented");
    }
    /// Tries to read data from the stream into the provided buffers, returning
    /// how many bytes were read.
    ///
    /// Data is copied to fill each buffer in order, with the final buffer
    /// written to possibly being only partially filled. This method behaves
    /// equivalently to a single call to [`try_read()`] with concatenated
    /// buffers.
    ///
    /// Receives any pending data from the socket but does not wait for new data
    /// to arrive. On success, returns the number of bytes read. Because
    /// `try_read_vectored()` is non-blocking, the buffer does not have to be
    /// stored by the async task and can exist entirely on the stack.
    ///
    /// Usually, [`readable()`] or [`ready()`] is used with this function.
    ///
    /// [`try_read()`]: UnixStream::try_read()
    /// [`readable()`]: UnixStream::readable()
    /// [`ready()`]: UnixStream::ready()
    ///
    /// # Return
    ///
    /// If data is successfully read, `Ok(n)` is returned, where `n` is the
    /// number of bytes read. `Ok(0)` indicates the stream's read half is closed
    /// and will no longer yield data. If the stream is not ready to read data
    /// `Err(io::ErrorKind::WouldBlock)` is returned.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::net::UnixStream;
    /// use std::error::Error;
    /// use std::io::{self, IoSliceMut};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     // Connect to a peer
    ///     let dir = tempfile::tempdir().unwrap();
    ///     let bind_path = dir.path().join("bind_path");
    ///     let stream = UnixStream::connect(bind_path).await?;
    ///
    ///     loop {
    ///         // Wait for the socket to be readable
    ///         stream.readable().await?;
    ///
    ///         // Creating the buffer **after** the `await` prevents it from
    ///         // being stored in the async task.
    ///         let mut buf_a = [0; 512];
    ///         let mut buf_b = [0; 1024];
    ///         let mut bufs = [
    ///             IoSliceMut::new(&mut buf_a),
    ///             IoSliceMut::new(&mut buf_b),
    ///         ];
    ///
    ///         // Try to read data, this may still fail with `WouldBlock`
    ///         // if the readiness event is a false positive.
    ///         match stream.try_read_vectored(&mut bufs) {
    ///             Ok(0) => break,
    ///             Ok(n) => {
    ///                 println!("read {} bytes", n);
    ///             }
    ///             Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
    ///                 continue;
    ///             }
    ///             Err(e) => {
    ///                 return Err(e.into());
    ///             }
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn try_read_vectored(
        &self,
        bufs: &mut [io::IoSliceMut<'_>],
    ) -> io::Result<usize> {
        panic!("STUB: not implemented");
    }
    cfg_io_util! {
        #[doc =
        " Tries to read data from the stream into the provided buffer, advancing the"]
        #[doc = " buffer's internal cursor, returning how many bytes were read."] #[doc =
        ""] #[doc =
        " Receives any pending data from the socket but does not wait for new data"]
        #[doc = " to arrive. On success, returns the number of bytes read. Because"]
        #[doc =
        " `try_read_buf()` is non-blocking, the buffer does not have to be stored by"]
        #[doc = " the async task and can exist entirely on the stack."] #[doc = ""] #[doc
        = " Usually, [`readable()`] or [`ready()`] is used with this function."] #[doc =
        ""] #[doc = " [`readable()`]: UnixStream::readable()"] #[doc =
        " [`ready()`]: UnixStream::ready()"] #[doc = ""] #[doc = " # Return"] #[doc = ""]
        #[doc = " If data is successfully read, `Ok(n)` is returned, where `n` is the"]
        #[doc =
        " number of bytes read. `Ok(0)` indicates the stream's read half is closed"]
        #[doc =
        " and will no longer yield data. If the stream is not ready to read data"] #[doc
        = " `Err(io::ErrorKind::WouldBlock)` is returned."] #[doc = ""] #[doc =
        " # Examples"] #[doc = ""] #[doc = " ```no_run"] #[doc =
        " use tokio::net::UnixStream;"] #[doc = " use std::error::Error;"] #[doc =
        " use std::io;"] #[doc = ""] #[doc = " #[tokio::main]"] #[doc =
        " async fn main() -> Result<(), Box<dyn Error>> {"] #[doc =
        "     // Connect to a peer"] #[doc =
        "     let dir = tempfile::tempdir().unwrap();"] #[doc =
        "     let bind_path = dir.path().join(\"bind_path\");"] #[doc =
        "     let stream = UnixStream::connect(bind_path).await?;"] #[doc = ""] #[doc =
        "     loop {"] #[doc = "         // Wait for the socket to be readable"] #[doc =
        "         stream.readable().await?;"] #[doc = ""] #[doc =
        "         let mut buf = Vec::with_capacity(4096);"] #[doc = ""] #[doc =
        "         // Try to read data, this may still fail with `WouldBlock`"] #[doc =
        "         // if the readiness event is a false positive."] #[doc =
        "         match stream.try_read_buf(&mut buf) {"] #[doc =
        "             Ok(0) => break,"] #[doc = "             Ok(n) => {"] #[doc =
        "                 println!(\"read {} bytes\", n);"] #[doc = "             }"]
        #[doc = "             Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {"]
        #[doc = "                 continue;"] #[doc = "             }"] #[doc =
        "             Err(e) => {"] #[doc = "                 return Err(e.into());"]
        #[doc = "             }"] #[doc = "         }"] #[doc = "     }"] #[doc = ""]
        #[doc = "     Ok(())"] #[doc = " }"] #[doc = " ```"] pub fn try_read_buf < B :
        BufMut > (& self, buf : & mut B) -> io::Result < usize > { self.io.registration()
        .try_io(Interest::READABLE, || { use std::io::Read; let dst = buf.chunk_mut();
        let dst = unsafe { & mut * (dst as * mut _ as * mut [std::mem::MaybeUninit < u8
        >] as * mut [u8]) }; let n = (&* self.io).read(dst) ?; unsafe { buf
        .advance_mut(n); } Ok(n) }) }
    }
    /// Waits for the socket to become writable.
    ///
    /// This function is equivalent to `ready(Interest::WRITABLE)` and is usually
    /// paired with `try_write()`.
    ///
    /// # Cancel safety
    ///
    /// This method is cancel safe. Once a readiness event occurs, the method
    /// will continue to return immediately until the readiness event is
    /// consumed by an attempt to write that fails with `WouldBlock` or
    /// `Poll::Pending`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::net::UnixStream;
    /// use std::error::Error;
    /// use std::io;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     // Connect to a peer
    ///     let dir = tempfile::tempdir().unwrap();
    ///     let bind_path = dir.path().join("bind_path");
    ///     let stream = UnixStream::connect(bind_path).await?;
    ///
    ///     loop {
    ///         // Wait for the socket to be writable
    ///         stream.writable().await?;
    ///
    ///         // Try to write data, this may still fail with `WouldBlock`
    ///         // if the readiness event is a false positive.
    ///         match stream.try_write(b"hello world") {
    ///             Ok(n) => {
    ///                 break;
    ///             }
    ///             Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
    ///                 continue;
    ///             }
    ///             Err(e) => {
    ///                 return Err(e.into());
    ///             }
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn writable(&self) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
    /// Polls for write readiness.
    ///
    /// If the unix stream is not currently ready for writing, this method will
    /// store a clone of the `Waker` from the provided `Context`. When the unix
    /// stream becomes ready for writing, `Waker::wake` will be called on the
    /// waker.
    ///
    /// Note that on multiple calls to `poll_write_ready` or `poll_write`, only
    /// the `Waker` from the `Context` passed to the most recent call is
    /// scheduled to receive a wakeup. (However, `poll_read_ready` retains a
    /// second, independent waker.)
    ///
    /// This function is intended for cases where creating and pinning a future
    /// via [`writable`] is not feasible. Where possible, using [`writable`] is
    /// preferred, as this supports polling from multiple tasks at once.
    ///
    /// # Return value
    ///
    /// The function returns:
    ///
    /// * `Poll::Pending` if the unix stream is not ready for writing.
    /// * `Poll::Ready(Ok(()))` if the unix stream is ready for writing.
    /// * `Poll::Ready(Err(e))` if an error is encountered.
    ///
    /// # Errors
    ///
    /// This function may encounter any standard I/O error except `WouldBlock`.
    ///
    /// [`writable`]: method@Self::writable
    pub fn poll_write_ready(&self, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        panic!("STUB: not implemented");
    }
    /// Tries to write a buffer to the stream, returning how many bytes were
    /// written.
    ///
    /// The function will attempt to write the entire contents of `buf`, but
    /// only part of the buffer may be written.
    ///
    /// This function is usually paired with `writable()`.
    ///
    /// # Return
    ///
    /// If data is successfully written, `Ok(n)` is returned, where `n` is the
    /// number of bytes written. If the stream is not ready to write data,
    /// `Err(io::ErrorKind::WouldBlock)` is returned.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::net::UnixStream;
    /// use std::error::Error;
    /// use std::io;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     // Connect to a peer
    ///     let dir = tempfile::tempdir().unwrap();
    ///     let bind_path = dir.path().join("bind_path");
    ///     let stream = UnixStream::connect(bind_path).await?;
    ///
    ///     loop {
    ///         // Wait for the socket to be writable
    ///         stream.writable().await?;
    ///
    ///         // Try to write data, this may still fail with `WouldBlock`
    ///         // if the readiness event is a false positive.
    ///         match stream.try_write(b"hello world") {
    ///             Ok(n) => {
    ///                 break;
    ///             }
    ///             Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
    ///                 continue;
    ///             }
    ///             Err(e) => {
    ///                 return Err(e.into());
    ///             }
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn try_write(&self, buf: &[u8]) -> io::Result<usize> {
        panic!("STUB: not implemented");
    }
    /// Tries to write several buffers to the stream, returning how many bytes
    /// were written.
    ///
    /// Data is written from each buffer in order, with the final buffer read
    /// from possible being only partially consumed. This method behaves
    /// equivalently to a single call to [`try_write()`] with concatenated
    /// buffers.
    ///
    /// This function is usually paired with `writable()`.
    ///
    /// [`try_write()`]: UnixStream::try_write()
    ///
    /// # Return
    ///
    /// If data is successfully written, `Ok(n)` is returned, where `n` is the
    /// number of bytes written. If the stream is not ready to write data,
    /// `Err(io::ErrorKind::WouldBlock)` is returned.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::net::UnixStream;
    /// use std::error::Error;
    /// use std::io;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     // Connect to a peer
    ///     let dir = tempfile::tempdir().unwrap();
    ///     let bind_path = dir.path().join("bind_path");
    ///     let stream = UnixStream::connect(bind_path).await?;
    ///
    ///     let bufs = [io::IoSlice::new(b"hello "), io::IoSlice::new(b"world")];
    ///
    ///     loop {
    ///         // Wait for the socket to be writable
    ///         stream.writable().await?;
    ///
    ///         // Try to write data, this may still fail with `WouldBlock`
    ///         // if the readiness event is a false positive.
    ///         match stream.try_write_vectored(&bufs) {
    ///             Ok(n) => {
    ///                 break;
    ///             }
    ///             Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
    ///                 continue;
    ///             }
    ///             Err(e) => {
    ///                 return Err(e.into());
    ///             }
    ///         }
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn try_write_vectored(&self, buf: &[io::IoSlice<'_>]) -> io::Result<usize> {
        panic!("STUB: not implemented");
    }
    /// Tries to read or write from the socket using a user-provided IO operation.
    ///
    /// If the socket is ready, the provided closure is called. The closure
    /// should attempt to perform IO operation on the socket by manually
    /// calling the appropriate syscall. If the operation fails because the
    /// socket is not actually ready, then the closure should return a
    /// `WouldBlock` error and the readiness flag is cleared. The return value
    /// of the closure is then returned by `try_io`.
    ///
    /// If the socket is not ready, then the closure is not called
    /// and a `WouldBlock` error is returned.
    ///
    /// The closure should only return a `WouldBlock` error if it has performed
    /// an IO operation on the socket that failed due to the socket not being
    /// ready. Returning a `WouldBlock` error in any other situation will
    /// incorrectly clear the readiness flag, which can cause the socket to
    /// behave incorrectly.
    ///
    /// The closure should not perform the IO operation using any of the methods
    /// defined on the Tokio `UnixStream` type, as this will mess with the
    /// readiness flag and can cause the socket to behave incorrectly.
    ///
    /// This method is not intended to be used with combined interests.
    /// The closure should perform only one type of IO operation, so it should not
    /// require more than one ready state. This method may panic or sleep forever
    /// if it is called with a combined interest.
    ///
    /// Usually, [`readable()`], [`writable()`] or [`ready()`] is used with this function.
    ///
    /// [`readable()`]: UnixStream::readable()
    /// [`writable()`]: UnixStream::writable()
    /// [`ready()`]: UnixStream::ready()
    pub fn try_io<R>(
        &self,
        interest: Interest,
        f: impl FnOnce() -> io::Result<R>,
    ) -> io::Result<R> {
        panic!("STUB: not implemented");
    }
    /// Reads or writes from the socket using a user-provided IO operation.
    ///
    /// The readiness of the socket is awaited and when the socket is ready,
    /// the provided closure is called. The closure should attempt to perform
    /// IO operation on the socket by manually calling the appropriate syscall.
    /// If the operation fails because the socket is not actually ready,
    /// then the closure should return a `WouldBlock` error. In such case the
    /// readiness flag is cleared and the socket readiness is awaited again.
    /// This loop is repeated until the closure returns an `Ok` or an error
    /// other than `WouldBlock`.
    ///
    /// The closure should only return a `WouldBlock` error if it has performed
    /// an IO operation on the socket that failed due to the socket not being
    /// ready. Returning a `WouldBlock` error in any other situation will
    /// incorrectly clear the readiness flag, which can cause the socket to
    /// behave incorrectly.
    ///
    /// The closure should not perform the IO operation using any of the methods
    /// defined on the Tokio `UnixStream` type, as this will mess with the
    /// readiness flag and can cause the socket to behave incorrectly.
    ///
    /// This method is not intended to be used with combined interests.
    /// The closure should perform only one type of IO operation, so it should not
    /// require more than one ready state. This method may panic or sleep forever
    /// if it is called with a combined interest.
    pub async fn async_io<R>(
        &self,
        interest: Interest,
        mut f: impl FnMut() -> io::Result<R>,
    ) -> io::Result<R> {
        panic!("STUB: not implemented");
    }
    /// Creates new [`UnixStream`] from a [`std::os::unix::net::UnixStream`].
    ///
    /// This function is intended to be used to wrap a `UnixStream` from the
    /// standard library in the Tokio equivalent.
    ///
    /// # Notes
    ///
    /// The caller is responsible for ensuring that the stream is in
    /// non-blocking mode. Otherwise all I/O operations on the stream
    /// will block the thread, which will cause unexpected behavior.
    /// Non-blocking mode can be set using [`set_nonblocking`].
    ///
    /// Passing a listener in blocking mode is always erroneous,
    /// and the behavior in that case may change in the future.
    /// For example, it could panic.
    ///
    /// [`set_nonblocking`]: std::os::unix::net::UnixStream::set_nonblocking
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::net::UnixStream;
    /// use std::os::unix::net::UnixStream as StdUnixStream;
    /// # use std::error::Error;
    ///
    /// # async fn dox() -> Result<(), Box<dyn Error>> {
    /// let std_stream = StdUnixStream::connect("/path/to/the/socket")?;
    /// std_stream.set_nonblocking(true)?;
    /// let stream = UnixStream::from_std(std_stream)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if it is not called from within a runtime with
    /// IO enabled.
    ///
    /// The runtime is usually set implicitly when this function is called
    /// from a future driven by a tokio runtime, otherwise runtime can be set
    /// explicitly with [`Runtime::enter`](crate::runtime::Runtime::enter) function.
    #[track_caller]
    pub fn from_std(stream: net::UnixStream) -> io::Result<UnixStream> {
        panic!("STUB: not implemented");
    }
    /// Turns a [`tokio::net::UnixStream`] into a [`std::os::unix::net::UnixStream`].
    ///
    /// The returned [`std::os::unix::net::UnixStream`] will have nonblocking
    /// mode set as `true`.  Use [`set_nonblocking`] to change the blocking
    /// mode if needed.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::error::Error;
    /// use std::io::Read;
    /// use tokio::net::UnixListener;
    /// # use tokio::net::UnixStream;
    /// # use tokio::io::AsyncWriteExt;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    /// #   if cfg!(miri) { return Ok(()); } // No Unix domain sockets in miri.
    ///     let dir = tempfile::tempdir().unwrap();
    ///     let bind_path = dir.path().join("bind_path");
    ///
    ///     let mut data = [0u8; 12];
    ///     let listener = UnixListener::bind(&bind_path)?;
    /// #   let handle = tokio::spawn(async {
    /// #       let mut stream = UnixStream::connect(bind_path).await.unwrap();
    /// #       stream.write(b"Hello world!").await.unwrap();
    /// #   });
    ///     let (tokio_unix_stream, _) = listener.accept().await?;
    ///     let mut std_unix_stream = tokio_unix_stream.into_std()?;
    /// #   handle.await.expect("The task being joined has panicked");
    ///     std_unix_stream.set_nonblocking(false)?;
    ///     std_unix_stream.read_exact(&mut data)?;
    /// #   assert_eq!(b"Hello world!", &data);
    ///     Ok(())
    /// }
    /// ```
    /// [`tokio::net::UnixStream`]: UnixStream
    /// [`std::os::unix::net::UnixStream`]: std::os::unix::net::UnixStream
    /// [`set_nonblocking`]: fn@std::os::unix::net::UnixStream::set_nonblocking
    pub fn into_std(self) -> io::Result<std::os::unix::net::UnixStream> {
        panic!("STUB: not implemented");
    }
    /// Creates an unnamed pair of connected sockets.
    ///
    /// This function will create a pair of interconnected Unix sockets for
    /// communicating back and forth between one another. Each socket will
    /// be associated with the default event loop's handle.
    pub fn pair() -> io::Result<(UnixStream, UnixStream)> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn new(stream: mio::net::UnixStream) -> io::Result<UnixStream> {
        panic!("STUB: not implemented");
    }
    /// Returns the socket address of the local half of this connection.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::net::UnixStream;
    ///
    /// # async fn dox() -> Result<(), Box<dyn std::error::Error>> {
    /// let dir = tempfile::tempdir().unwrap();
    /// let bind_path = dir.path().join("bind_path");
    /// let stream = UnixStream::connect(bind_path).await?;
    ///
    /// println!("{:?}", stream.local_addr()?);
    /// # Ok(())
    /// # }
    /// ```
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        panic!("STUB: not implemented");
    }
    /// Returns the socket address of the remote half of this connection.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::net::UnixStream;
    ///
    /// # async fn dox() -> Result<(), Box<dyn std::error::Error>> {
    /// let dir = tempfile::tempdir().unwrap();
    /// let bind_path = dir.path().join("bind_path");
    /// let stream = UnixStream::connect(bind_path).await?;
    ///
    /// println!("{:?}", stream.peer_addr()?);
    /// # Ok(())
    /// # }
    /// ```
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        panic!("STUB: not implemented");
    }
    /// Returns effective credentials of the process which called `connect` or `pair`.
    pub fn peer_cred(&self) -> io::Result<UCred> {
        panic!("STUB: not implemented");
    }
    /// Returns the value of the `SO_ERROR` option.
    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        panic!("STUB: not implemented");
    }
    /// Shuts down the read, write, or both halves of this connection.
    ///
    /// This function will cause all pending and future I/O calls on the
    /// specified portions to immediately return with an appropriate value
    /// (see the documentation of `Shutdown`).
    pub(super) fn shutdown_std(&self, how: Shutdown) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
    #[allow(clippy::needless_lifetimes)]
    /// Splits a `UnixStream` into a read half and a write half, which can be used
    /// to read and write the stream concurrently.
    ///
    /// This method is more efficient than [`into_split`], but the halves cannot be
    /// moved into independently spawned tasks.
    ///
    /// [`into_split`]: Self::into_split()
    pub fn split<'a>(&'a mut self) -> (ReadHalf<'a>, WriteHalf<'a>) {
        panic!("STUB: not implemented");
    }
    /// Splits a `UnixStream` into a read half and a write half, which can be used
    /// to read and write the stream concurrently.
    ///
    /// Unlike [`split`], the owned halves can be moved to separate tasks, however
    /// this comes at the cost of a heap allocation.
    ///
    /// **Note:** Dropping the write half will only shut down the write half of the
    /// stream. This is equivalent to calling [`shutdown()`] on the `UnixStream`.
    ///
    /// [`split`]: Self::split()
    /// [`shutdown()`]: fn@crate::io::AsyncWriteExt::shutdown
    pub fn into_split(self) -> (OwnedReadHalf, OwnedWriteHalf) {
        panic!("STUB: not implemented");
    }
}
impl TryFrom<net::UnixStream> for UnixStream {
    type Error = io::Error;
    /// Consumes stream, returning the tokio I/O object.
    ///
    /// This is equivalent to
    /// [`UnixStream::from_std(stream)`](UnixStream::from_std).
    fn try_from(stream: net::UnixStream) -> io::Result<Self> {
        panic!("STUB: not implemented");
    }
}
impl AsyncRead for UnixStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        panic!("STUB: not implemented");
    }
}
impl AsyncWrite for UnixStream {
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        panic!("STUB: not implemented");
    }
    fn poll_write_vectored(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bufs: &[io::IoSlice<'_>],
    ) -> Poll<io::Result<usize>> {
        panic!("STUB: not implemented");
    }
    fn is_write_vectored(&self) -> bool {
        panic!("STUB: not implemented");
    }
    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        panic!("STUB: not implemented");
    }
    fn poll_shutdown(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        panic!("STUB: not implemented");
    }
}
impl UnixStream {
    pub(crate) fn poll_read_priv(
        &self,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        panic!("STUB: not implemented");
    }
    pub(crate) fn poll_write_priv(
        &self,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        panic!("STUB: not implemented");
    }
    pub(super) fn poll_write_vectored_priv(
        &self,
        cx: &mut Context<'_>,
        bufs: &[io::IoSlice<'_>],
    ) -> Poll<io::Result<usize>> {
        panic!("STUB: not implemented");
    }
}
impl fmt::Debug for UnixStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl AsRef<Self> for UnixStream {
    fn as_ref(&self) -> &Self {
        panic!("STUB: not implemented");
    }
}
impl AsRawFd for UnixStream {
    fn as_raw_fd(&self) -> RawFd {
        panic!("STUB: not implemented");
    }
}
impl AsFd for UnixStream {
    fn as_fd(&self) -> BorrowedFd<'_> {
        panic!("STUB: not implemented");
    }
}
