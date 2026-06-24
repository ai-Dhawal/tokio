cfg_not_wasi! {
    use std::time::Duration;
}
cfg_not_wasip1! {
    use crate ::net:: { to_socket_addrs, ToSocketAddrs }; use std::future::poll_fn;
}
use crate::io::{AsyncRead, AsyncWrite, Interest, PollEvented, ReadBuf, Ready};
use crate::net::tcp::split::{split, ReadHalf, WriteHalf};
use crate::net::tcp::split_owned::{split_owned, OwnedReadHalf, OwnedWriteHalf};
use crate::util::check_socket_for_blocking;
use std::fmt;
use std::io;
use std::net::{Shutdown, SocketAddr};
use std::pin::Pin;
use std::task::{ready, Context, Poll};
cfg_io_util! {
    use bytes::BufMut;
}
cfg_net! {
    #[doc = " A TCP stream between a local and a remote socket."] #[doc = ""] #[doc =
    " A TCP stream can either be created by connecting to an endpoint, via the"] #[doc =
    " [`connect`] method, or by [accepting] a connection from a [listener]. A"] #[doc =
    " TCP stream can also be created via the [`TcpSocket`] type."] #[doc = ""] #[doc =
    " Reading and writing to a `TcpStream` is usually done using the"] #[doc =
    " convenience methods found on the [`AsyncReadExt`] and [`AsyncWriteExt`]"] #[doc =
    " traits."] #[doc = ""] #[doc = " [`connect`]: method@TcpStream::connect"] #[doc =
    " [accepting]: method@crate::net::TcpListener::accept"] #[doc =
    " [listener]: struct@crate::net::TcpListener"] #[doc =
    " [`TcpSocket`]: struct@crate::net::TcpSocket"] #[doc =
    " [`AsyncReadExt`]: trait@crate::io::AsyncReadExt"] #[doc =
    " [`AsyncWriteExt`]: trait@crate::io::AsyncWriteExt"] #[doc = ""] #[doc =
    " # Examples"] #[doc = ""] #[doc = " ```no_run"] #[doc =
    " use tokio::net::TcpStream;"] #[doc = " use tokio::io::AsyncWriteExt;"] #[doc =
    " use std::error::Error;"] #[doc = ""] #[doc = " #[tokio::main]"] #[doc =
    " async fn main() -> Result<(), Box<dyn Error>> {"] #[doc =
    "     // Connect to a peer"] #[doc =
    "     let mut stream = TcpStream::connect(\"127.0.0.1:8080\").await?;"] #[doc = ""]
    #[doc = "     // Write some data."] #[doc =
    "     stream.write_all(b\"hello world!\").await?;"] #[doc = ""] #[doc =
    "     Ok(())"] #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc =
    " The [`write_all`] method is defined on the [`AsyncWriteExt`] trait."] #[doc = ""]
    #[doc = " [`write_all`]: fn@crate::io::AsyncWriteExt::write_all"] #[doc =
    " [`AsyncWriteExt`]: trait@crate::io::AsyncWriteExt"] #[doc = ""] #[doc =
    " To shut down the stream in the write direction, you can call the"] #[doc =
    " [`shutdown()`] method. This will cause the other peer to receive a read of"] #[doc
    = " length 0, indicating that no more data will be sent. This only closes"] #[doc =
    " the stream in one direction."] #[doc = ""] #[doc =
    " [`shutdown()`]: fn@crate::io::AsyncWriteExt::shutdown"] pub struct TcpStream { io :
    PollEvented < mio::net::TcpStream >, }
}
impl TcpStream {
    cfg_not_wasip1! {
        #[doc = " Opens a TCP connection to a remote host."] #[doc = ""] #[doc =
        " `addr` is an address of the remote host. Anything which implements the"] #[doc
        = " [`ToSocketAddrs`] trait can be supplied as the address.  If `addr`"] #[doc =
        " yields multiple addresses, connect will be attempted with each of the"] #[doc =
        " addresses until a connection is successful. If none of the addresses"] #[doc =
        " result in a successful connection, the error returned from the last"] #[doc =
        " connection attempt (the last address) is returned."] #[doc = ""] #[doc =
        " To configure the socket before connecting, you can use the [`TcpSocket`]"]
        #[doc = " type."] #[doc = ""] #[doc =
        " [`ToSocketAddrs`]: trait@crate::net::ToSocketAddrs"] #[doc =
        " [`TcpSocket`]: struct@crate::net::TcpSocket"] #[doc = ""] #[doc =
        " # Examples"] #[doc = ""] #[doc = " ```no_run"] #[doc =
        " use tokio::net::TcpStream;"] #[doc = " use tokio::io::AsyncWriteExt;"] #[doc =
        " use std::error::Error;"] #[doc = ""] #[doc = " #[tokio::main]"] #[doc =
        " async fn main() -> Result<(), Box<dyn Error>> {"] #[doc =
        "     // Connect to a peer"] #[doc =
        "     let mut stream = TcpStream::connect(\"127.0.0.1:8080\").await?;"] #[doc =
        ""] #[doc = "     // Write some data."] #[doc =
        "     stream.write_all(b\"hello world!\").await?;"] #[doc = ""] #[doc =
        "     Ok(())"] #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc =
        " The [`write_all`] method is defined on the [`AsyncWriteExt`] trait."] #[doc =
        ""] #[doc = " [`write_all`]: fn@crate::io::AsyncWriteExt::write_all"] #[doc =
        " [`AsyncWriteExt`]: trait@crate::io::AsyncWriteExt"] pub async fn connect < A :
        ToSocketAddrs > (addr : A) -> io::Result < TcpStream > { let addrs =
        to_socket_addrs(addr). await ?; let mut last_err = None; for addr in addrs {
        match TcpStream::connect_addr(addr). await { Ok(stream) => return Ok(stream),
        Err(e) => last_err = Some(e), } } Err(last_err.unwrap_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "could not resolve to any address",)
        })) } #[doc = " Establishes a connection to the specified `addr`."] async fn
        connect_addr(addr : SocketAddr) -> io::Result < TcpStream > { let sys =
        mio::net::TcpStream::connect(addr) ?; TcpStream::connect_mio(sys). await } pub
        (crate) async fn connect_mio(sys : mio::net::TcpStream) -> io::Result < TcpStream
        > { let stream = TcpStream::new(sys) ?; poll_fn(| cx | stream.io.registration()
        .poll_write_ready(cx)). await ?; if let Some(e) = stream.io.take_error() ? {
        return Err(e); } Ok(stream) }
    }
    pub(crate) fn new(connected: mio::net::TcpStream) -> io::Result<TcpStream> {
        panic!("STUB: not implemented");
    }
    /// Creates new `TcpStream` from a `std::net::TcpStream`.
    ///
    /// This function is intended to be used to wrap a TCP stream from the
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
    /// [`set_nonblocking`]: std::net::TcpStream::set_nonblocking
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use std::error::Error;
    /// use tokio::net::TcpStream;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     let std_stream = std::net::TcpStream::connect("127.0.0.1:34254")?;
    ///     std_stream.set_nonblocking(true)?;
    ///     let stream = TcpStream::from_std(std_stream)?;
    ///     Ok(())
    /// }
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
    pub fn from_std(stream: std::net::TcpStream) -> io::Result<TcpStream> {
        panic!("STUB: not implemented");
    }
    /// Turns a [`tokio::net::TcpStream`] into a [`std::net::TcpStream`].
    ///
    /// The returned [`std::net::TcpStream`] will have nonblocking mode set as `true`.
    /// Use [`set_nonblocking`] to change the blocking mode if needed.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::error::Error;
    /// use std::io::Read;
    /// use tokio::net::TcpListener;
    /// # use tokio::net::TcpStream;
    /// # use tokio::io::AsyncWriteExt;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     let mut data = [0u8; 12];
    /// #   if false {
    ///     let listener = TcpListener::bind("127.0.0.1:34254").await?;
    /// #   }
    /// #   let listener = TcpListener::bind("127.0.0.1:0").await?;
    /// #   let addr = listener.local_addr().unwrap();
    /// #   let handle = tokio::spawn(async move {
    /// #       let mut stream: TcpStream = TcpStream::connect(addr).await.unwrap();
    /// #       stream.write_all(b"Hello world!").await.unwrap();
    /// #   });
    ///     let (tokio_tcp_stream, _) = listener.accept().await?;
    ///     let mut std_tcp_stream = tokio_tcp_stream.into_std()?;
    /// #   handle.await.expect("The task being joined has panicked");
    ///     std_tcp_stream.set_nonblocking(false)?;
    ///     std_tcp_stream.read_exact(&mut data)?;
    /// #   assert_eq!(b"Hello world!", &data);
    ///     Ok(())
    /// }
    /// ```
    /// [`tokio::net::TcpStream`]: TcpStream
    /// [`std::net::TcpStream`]: std::net::TcpStream
    /// [`set_nonblocking`]: fn@std::net::TcpStream::set_nonblocking
    pub fn into_std(self) -> io::Result<std::net::TcpStream> {
        panic!("STUB: not implemented");
    }
    /// Returns the local address that this stream is bound to.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::net::TcpStream;
    ///
    /// # async fn dox() -> Result<(), Box<dyn std::error::Error>> {
    /// let stream = TcpStream::connect("127.0.0.1:8080").await?;
    ///
    /// println!("{:?}", stream.local_addr()?);
    /// # Ok(())
    /// # }
    /// ```
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        panic!("STUB: not implemented");
    }
    /// Returns the value of the `SO_ERROR` option.
    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        panic!("STUB: not implemented");
    }
    /// Returns the remote address that this stream is connected to.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::net::TcpStream;
    ///
    /// # async fn dox() -> Result<(), Box<dyn std::error::Error>> {
    /// let stream = TcpStream::connect("127.0.0.1:8080").await?;
    ///
    /// println!("{:?}", stream.peer_addr()?);
    /// # Ok(())
    /// # }
    /// ```
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        panic!("STUB: not implemented");
    }
    /// Attempts to receive data on the socket, without removing that data from
    /// the queue, registering the current task for wakeup if data is not yet
    /// available.
    ///
    /// Note that on multiple calls to `poll_peek`, `poll_read` or
    /// `poll_read_ready`, only the `Waker` from the `Context` passed to the
    /// most recent call is scheduled to receive a wakeup. (However,
    /// `poll_write` retains a second, independent waker.)
    ///
    /// # Return value
    ///
    /// The function returns:
    ///
    /// * `Poll::Pending` if data is not yet available.
    /// * `Poll::Ready(Ok(n))` if data is available. `n` is the number of bytes peeked.
    /// * `Poll::Ready(Err(e))` if an error is encountered.
    ///
    /// # Errors
    ///
    /// This function may encounter any standard I/O error except `WouldBlock`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::io::{self, ReadBuf};
    /// use tokio::net::TcpStream;
    ///
    /// use std::future::poll_fn;
    ///
    /// #[tokio::main]
    /// async fn main() -> io::Result<()> {
    ///     let stream = TcpStream::connect("127.0.0.1:8000").await?;
    ///     let mut buf = [0; 10];
    ///     let mut buf = ReadBuf::new(&mut buf);
    ///
    ///     poll_fn(|cx| {
    ///         stream.poll_peek(cx, &mut buf)
    ///     }).await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn poll_peek(
        &self,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<usize>> {
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
    /// use tokio::net::TcpStream;
    /// use std::error::Error;
    /// use std::io;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     let stream = TcpStream::connect("127.0.0.1:8080").await?;
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
    ///                     continue
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
    /// use tokio::net::TcpStream;
    /// use std::error::Error;
    /// use std::io;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     // Connect to a peer
    ///     let stream = TcpStream::connect("127.0.0.1:8080").await?;
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
    /// If the tcp stream is not currently ready for reading, this method will
    /// store a clone of the `Waker` from the provided `Context`. When the tcp
    /// stream becomes ready for reading, `Waker::wake` will be called on the
    /// waker.
    ///
    /// Note that on multiple calls to `poll_read_ready`, `poll_read` or
    /// `poll_peek`, only the `Waker` from the `Context` passed to the most
    /// recent call is scheduled to receive a wakeup. (However,
    /// `poll_write_ready` retains a second, independent waker.)
    ///
    /// This function is intended for cases where creating and pinning a future
    /// via [`readable`] is not feasible. Where possible, using [`readable`] is
    /// preferred, as this supports polling from multiple tasks at once.
    ///
    /// # Return value
    ///
    /// The function returns:
    ///
    /// * `Poll::Pending` if the tcp stream is not ready for reading.
    /// * `Poll::Ready(Ok(()))` if the tcp stream is ready for reading.
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
    /// Tries to read data from the stream into the provided buffer, returning how
    /// many bytes were read.
    ///
    /// Receives any pending data from the socket but does not wait for new data
    /// to arrive. On success, returns the number of bytes read. Because
    /// `try_read()` is non-blocking, the buffer does not have to be stored by
    /// the async task and can exist entirely on the stack.
    ///
    /// Usually, [`readable()`] or [`ready()`] is used with this function.
    ///
    /// [`readable()`]: TcpStream::readable()
    /// [`ready()`]: TcpStream::ready()
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
    /// use tokio::net::TcpStream;
    /// use std::error::Error;
    /// use std::io;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     // Connect to a peer
    ///     let stream = TcpStream::connect("127.0.0.1:8080").await?;
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
    /// [`try_read()`]: TcpStream::try_read()
    /// [`readable()`]: TcpStream::readable()
    /// [`ready()`]: TcpStream::ready()
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
    /// use tokio::net::TcpStream;
    /// use std::error::Error;
    /// use std::io::{self, IoSliceMut};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     // Connect to a peer
    ///     let stream = TcpStream::connect("127.0.0.1:8080").await?;
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
        ""] #[doc = " [`readable()`]: TcpStream::readable()"] #[doc =
        " [`ready()`]: TcpStream::ready()"] #[doc = ""] #[doc = " # Return"] #[doc = ""]
        #[doc = " If data is successfully read, `Ok(n)` is returned, where `n` is the"]
        #[doc =
        " number of bytes read. `Ok(0)` indicates the stream's read half is closed"]
        #[doc =
        " and will no longer yield data. If the stream is not ready to read data"] #[doc
        = " `Err(io::ErrorKind::WouldBlock)` is returned."] #[doc = ""] #[doc =
        " # Examples"] #[doc = ""] #[doc = " ```no_run"] #[doc =
        " use tokio::net::TcpStream;"] #[doc = " use std::error::Error;"] #[doc =
        " use std::io;"] #[doc = ""] #[doc = " #[tokio::main]"] #[doc =
        " async fn main() -> Result<(), Box<dyn Error>> {"] #[doc =
        "     // Connect to a peer"] #[doc =
        "     let stream = TcpStream::connect(\"127.0.0.1:8080\").await?;"] #[doc = ""]
        #[doc = "     loop {"] #[doc = "         // Wait for the socket to be readable"]
        #[doc = "         stream.readable().await?;"] #[doc = ""] #[doc =
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
    /// use tokio::net::TcpStream;
    /// use std::error::Error;
    /// use std::io;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     // Connect to a peer
    ///     let stream = TcpStream::connect("127.0.0.1:8080").await?;
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
    /// If the tcp stream is not currently ready for writing, this method will
    /// store a clone of the `Waker` from the provided `Context`. When the tcp
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
    /// * `Poll::Pending` if the tcp stream is not ready for writing.
    /// * `Poll::Ready(Ok(()))` if the tcp stream is ready for writing.
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
    /// Try to write a buffer to the stream, returning how many bytes were
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
    /// use tokio::net::TcpStream;
    /// use std::error::Error;
    /// use std::io;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     // Connect to a peer
    ///     let stream = TcpStream::connect("127.0.0.1:8080").await?;
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
    /// from possibly being only partially consumed. This method behaves
    /// equivalently to a single call to [`try_write()`] with concatenated
    /// buffers.
    ///
    /// This function is usually paired with `writable()`.
    ///
    /// [`try_write()`]: TcpStream::try_write()
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
    /// use tokio::net::TcpStream;
    /// use std::error::Error;
    /// use std::io;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     // Connect to a peer
    ///     let stream = TcpStream::connect("127.0.0.1:8080").await?;
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
    pub fn try_write_vectored(&self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
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
    /// defined on the Tokio `TcpStream` type, as this will mess with the
    /// readiness flag and can cause the socket to behave incorrectly.
    ///
    /// This method is not intended to be used with combined interests.
    /// The closure should perform only one type of IO operation, so it should not
    /// require more than one ready state. This method may panic or sleep forever
    /// if it is called with a combined interest.
    ///
    /// Usually, [`readable()`], [`writable()`] or [`ready()`] is used with this function.
    ///
    /// [`readable()`]: TcpStream::readable()
    /// [`writable()`]: TcpStream::writable()
    /// [`ready()`]: TcpStream::ready()
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
    /// defined on the Tokio `TcpStream` type, as this will mess with the
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
    /// Receives data on the socket from the remote address to which it is
    /// connected, without removing that data from the queue. On success,
    /// returns the number of bytes peeked.
    ///
    /// Successive calls return the same data. This is accomplished by passing
    /// `MSG_PEEK` as a flag to the underlying `recv` system call.
    ///
    /// # Cancel safety
    ///
    /// This method is cancel safe. If the method is used as a branch in
    /// [`tokio::select!`](crate::select) and another branch
    /// completes first, then it is guaranteed that no peek was performed, and
    /// that `buf` has not been modified.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::net::TcpStream;
    /// use tokio::io::AsyncReadExt;
    /// use std::error::Error;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     // Connect to a peer
    ///     let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    ///
    ///     let mut b1 = [0; 10];
    ///     let mut b2 = [0; 10];
    ///
    ///     // Peek at the data
    ///     let n = stream.peek(&mut b1).await?;
    ///
    ///     // Read the data
    ///     assert_eq!(n, stream.read(&mut b2[..n]).await?);
    ///     assert_eq!(&b1[..n], &b2[..n]);
    ///
    ///     Ok(())
    /// }
    /// ```
    ///
    /// The [`read`] method is defined on the [`AsyncReadExt`] trait.
    ///
    /// [`read`]: fn@crate::io::AsyncReadExt::read
    /// [`AsyncReadExt`]: trait@crate::io::AsyncReadExt
    pub async fn peek(&self, buf: &mut [u8]) -> io::Result<usize> {
        panic!("STUB: not implemented");
    }
    /// Shuts down the read, write, or both halves of this connection.
    ///
    /// This function will cause all pending and future I/O on the specified
    /// portions to return immediately with an appropriate value (see the
    /// documentation of `Shutdown`).
    ///
    /// Remark: this function transforms `Err(std::io::ErrorKind::NotConnected)` to `Ok(())`.
    /// It does this to abstract away OS specific logic and to prevent a race condition between
    /// this function call and the OS closing this socket because of external events (e.g. TCP reset).
    /// See <https://github.com/tokio-rs/tokio/issues/4665> for more information.
    pub(super) fn shutdown_std(&self, how: Shutdown) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
    /// Gets the value of the `TCP_NODELAY` option on this socket.
    ///
    /// For more information about this option, see [`set_nodelay`].
    ///
    /// [`set_nodelay`]: TcpStream::set_nodelay
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::net::TcpStream;
    ///
    /// # async fn dox() -> Result<(), Box<dyn std::error::Error>> {
    /// let stream = TcpStream::connect("127.0.0.1:8080").await?;
    ///
    /// println!("{:?}", stream.nodelay()?);
    /// # Ok(())
    /// # }
    /// ```
    pub fn nodelay(&self) -> io::Result<bool> {
        panic!("STUB: not implemented");
    }
    /// Sets the value of the `TCP_NODELAY` option on this socket.
    ///
    /// If set, this option disables the Nagle algorithm. This means that
    /// segments are always sent as soon as possible, even if there is only a
    /// small amount of data. When not set, data is buffered until there is a
    /// sufficient amount to send out, thereby avoiding the frequent sending of
    /// small packets.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::net::TcpStream;
    ///
    /// # async fn dox() -> Result<(), Box<dyn std::error::Error>> {
    /// let stream = TcpStream::connect("127.0.0.1:8080").await?;
    ///
    /// stream.set_nodelay(true)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_nodelay(&self, nodelay: bool) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
    /// Gets the value of the `TCP_QUICKACK` option on this socket.
    ///
    /// For more information about this option, see [`TcpStream::set_quickack`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::net::TcpStream;
    ///
    /// # async fn dox() -> Result<(), Box<dyn std::error::Error>> {
    /// let stream = TcpStream::connect("127.0.0.1:8080").await?;
    ///
    /// stream.quickack()?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(
        any(
            target_os = "linux",
            target_os = "android",
            target_os = "fuchsia",
            target_os = "cygwin",
        )
    )]
    #[cfg_attr(
        docsrs,
        doc(
            cfg(
                any(
                    target_os = "linux",
                    target_os = "android",
                    target_os = "fuchsia",
                    target_os = "cygwin"
                )
            )
        )
    )]
    pub fn quickack(&self) -> io::Result<bool> {
        panic!("STUB: not implemented");
    }
    /// Enable or disable `TCP_QUICKACK`.
    ///
    /// This flag causes Linux to eagerly send `ACK`s rather than delaying them.
    /// Linux may reset this flag after further operations on the socket.
    ///
    /// See [`man 7 tcp`](https://man7.org/linux/man-pages/man7/tcp.7.html) and
    /// [TCP delayed acknowledgment](https://en.wikipedia.org/wiki/TCP_delayed_acknowledgment)
    /// for more information.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::net::TcpStream;
    ///
    /// # async fn dox() -> Result<(), Box<dyn std::error::Error>> {
    /// let stream = TcpStream::connect("127.0.0.1:8080").await?;
    ///
    /// stream.set_quickack(true)?;
    /// # Ok(())
    /// # }
    /// ```
    #[cfg(
        any(
            target_os = "linux",
            target_os = "android",
            target_os = "fuchsia",
            target_os = "cygwin",
        )
    )]
    #[cfg_attr(
        docsrs,
        doc(
            cfg(
                any(
                    target_os = "linux",
                    target_os = "android",
                    target_os = "fuchsia",
                    target_os = "cygwin"
                )
            )
        )
    )]
    pub fn set_quickack(&self, quickack: bool) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
    cfg_not_wasi! {
        #[doc = " Reads the linger duration for this socket by getting the `SO_LINGER`"]
        #[doc = " option."] #[doc = ""] #[doc =
        " For more information about this option, see [`set_zero_linger`] and [`set_linger`]."]
        #[doc = ""] #[doc = " [`set_linger`]: TcpStream::set_linger"] #[doc =
        " [`set_zero_linger`]: TcpStream::set_zero_linger"] #[doc = ""] #[doc =
        " # Examples"] #[doc = ""] #[doc = " ```no_run"] #[doc =
        " use tokio::net::TcpStream;"] #[doc = ""] #[doc =
        " # async fn dox() -> Result<(), Box<dyn std::error::Error>> {"] #[doc =
        " let stream = TcpStream::connect(\"127.0.0.1:8080\").await?;"] #[doc = ""] #[doc
        = " println!(\"{:?}\", stream.linger()?);"] #[doc = " # Ok(())"] #[doc = " # }"]
        #[doc = " ```"] pub fn linger(& self) -> io::Result < Option < Duration >> {
        socket2::SockRef::from(self).linger() } #[doc =
        " Sets the linger duration of this socket by setting the `SO_LINGER` option."]
        #[doc = ""] #[doc =
        " This option controls the action taken when a stream has unsent messages and the stream is"]
        #[doc =
        " closed. If `SO_LINGER` is set, the system shall block the process until it can transmit the"]
        #[doc = " data or until the time expires."] #[doc = ""] #[doc =
        " If `SO_LINGER` is not specified, and the stream is closed, the system handles the call in a"]
        #[doc = " way that allows the process to continue as quickly as possible."] #[doc
        = ""] #[doc =
        " This option is deprecated because setting `SO_LINGER` on a socket used with Tokio is"]
        #[doc =
        " always incorrect as it leads to blocking the thread when the socket is closed. For more"]
        #[doc = " details, please see:"] #[doc = ""] #[doc =
        " > Volumes of communications have been devoted to the intricacies of `SO_LINGER` versus"]
        #[doc =
        " > non-blocking (`O_NONBLOCK`) sockets. From what I can tell, the final word is: don't"]
        #[doc =
        " > do it. Rely on the `shutdown()`-followed-by-`read()`-eof technique instead."]
        #[doc = " >"] #[doc =
        " > From [The ultimate `SO_LINGER` page, or: why is my tcp not reliable](https://blog.netherlabs.nl/articles/2009/01/18/the-ultimate-so_linger-page-or-why-is-my-tcp-not-reliable)"]
        #[doc = ""] #[doc =
        " Although this method is deprecated, it will not be removed from Tokio."] #[doc
        = ""] #[doc =
        " Note that the special case of setting `SO_LINGER` to zero does not lead to blocking."]
        #[doc =
        " Tokio provides [`set_zero_linger`](Self::set_zero_linger) for this purpose."]
        #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```no_run"] #[doc =
        " # #![allow(deprecated)]"] #[doc = " use tokio::net::TcpStream;"] #[doc = ""]
        #[doc = " # async fn dox() -> Result<(), Box<dyn std::error::Error>> {"] #[doc =
        " let stream = TcpStream::connect(\"127.0.0.1:8080\").await?;"] #[doc = ""] #[doc
        = " stream.set_linger(None)?;"] #[doc = " # Ok(())"] #[doc = " # }"] #[doc =
        " ```"] #[deprecated =
        "`SO_LINGER` causes the socket to block the thread on drop"] pub fn set_linger(&
        self, dur : Option < Duration >) -> io::Result < () > {
        socket2::SockRef::from(self).set_linger(dur) } #[doc =
        " Sets a linger duration of zero on this socket by setting the `SO_LINGER` option."]
        #[doc = ""] #[doc =
        " This causes the connection to be forcefully aborted (\"abortive close\") when the socket"]
        #[doc =
        " is dropped or closed. Instead of the normal TCP shutdown handshake (`FIN`/`ACK`), a TCP"]
        #[doc =
        " `RST` (reset) segment is sent to the peer, and the socket immediately discards any"]
        #[doc =
        " unsent data residing in the socket send buffer. This prevents the socket from entering"]
        #[doc = " the `TIME_WAIT` state after closing it."] #[doc = ""] #[doc =
        " This is a destructive action. Any data currently buffered by the OS but not yet"]
        #[doc =
        " transmitted will be lost. The peer will likely receive a \"Connection Reset\" error"]
        #[doc = " rather than a clean end-of-stream."] #[doc = ""] #[doc =
        " See the documentation for [`set_linger`](Self::set_linger) for additional details on"]
        #[doc = " how `SO_LINGER` works."] #[doc = ""] #[doc = " # Examples"] #[doc = ""]
        #[doc = " ```no_run"] #[doc = " use std::time::Duration;"] #[doc =
        " use tokio::net::TcpStream;"] #[doc = ""] #[doc =
        " # async fn dox() -> Result<(), Box<dyn std::error::Error>> {"] #[doc =
        " let stream = TcpStream::connect(\"127.0.0.1:8080\").await?;"] #[doc = ""] #[doc
        = " stream.set_zero_linger()?;"] #[doc =
        " assert_eq!(stream.linger()?, Some(Duration::ZERO));"] #[doc = " # Ok(())"]
        #[doc = " # }"] #[doc = " ```"] pub fn set_zero_linger(& self) -> io::Result < ()
        > { socket2::SockRef::from(self).set_linger(Some(Duration::ZERO)) }
    }
    /// Gets the value of the `IP_TTL` option for this socket.
    ///
    /// For more information about this option, see [`set_ttl`].
    ///
    /// [`set_ttl`]: TcpStream::set_ttl
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::net::TcpStream;
    ///
    /// # async fn dox() -> Result<(), Box<dyn std::error::Error>> {
    /// let stream = TcpStream::connect("127.0.0.1:8080").await?;
    ///
    /// println!("{:?}", stream.ttl()?);
    /// # Ok(())
    /// # }
    /// ```
    pub fn ttl(&self) -> io::Result<u32> {
        panic!("STUB: not implemented");
    }
    /// Sets the value for the `IP_TTL` option on this socket.
    ///
    /// This value sets the time-to-live field that is used in every packet sent
    /// from this socket.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::net::TcpStream;
    ///
    /// # async fn dox() -> Result<(), Box<dyn std::error::Error>> {
    /// let stream = TcpStream::connect("127.0.0.1:8080").await?;
    ///
    /// stream.set_ttl(123)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
    #[allow(clippy::needless_lifetimes)]
    /// Splits a `TcpStream` into a read half and a write half, which can be used
    /// to read and write the stream concurrently.
    ///
    /// This method is more efficient than [`into_split`], but the halves cannot be
    /// moved into independently spawned tasks.
    ///
    /// [`into_split`]: TcpStream::into_split()
    pub fn split<'a>(&'a mut self) -> (ReadHalf<'a>, WriteHalf<'a>) {
        panic!("STUB: not implemented");
    }
    /// Splits a `TcpStream` into a read half and a write half, which can be used
    /// to read and write the stream concurrently.
    ///
    /// Unlike [`split`], the owned halves can be moved to separate tasks, however
    /// this comes at the cost of a heap allocation.
    ///
    /// **Note:** Dropping the write half will shut down the write half of the TCP
    /// stream. This is equivalent to calling [`shutdown()`] on the `TcpStream`.
    ///
    /// [`split`]: TcpStream::split()
    /// [`shutdown()`]: fn@crate::io::AsyncWriteExt::shutdown
    pub fn into_split(self) -> (OwnedReadHalf, OwnedWriteHalf) {
        panic!("STUB: not implemented");
    }
    pub(crate) fn poll_read_priv(
        &self,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        panic!("STUB: not implemented");
    }
    pub(super) fn poll_write_priv(
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
impl TryFrom<std::net::TcpStream> for TcpStream {
    type Error = io::Error;
    /// Consumes stream, returning the tokio I/O object.
    ///
    /// This is equivalent to
    /// [`TcpStream::from_std(stream)`](TcpStream::from_std).
    fn try_from(stream: std::net::TcpStream) -> Result<Self, Self::Error> {
        panic!("STUB: not implemented");
    }
}
impl AsyncRead for TcpStream {
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        panic!("STUB: not implemented");
    }
}
impl AsyncWrite for TcpStream {
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
    #[inline]
    fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        panic!("STUB: not implemented");
    }
    fn poll_shutdown(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<io::Result<()>> {
        panic!("STUB: not implemented");
    }
}
impl fmt::Debug for TcpStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl AsRef<Self> for TcpStream {
    fn as_ref(&self) -> &Self {
        panic!("STUB: not implemented");
    }
}
#[cfg(unix)]
mod sys {
    use super::TcpStream;
    use std::os::unix::prelude::*;
    impl AsRawFd for TcpStream {
        fn as_raw_fd(&self) -> RawFd {
            panic!("STUB: not implemented");
        }
    }
    impl AsFd for TcpStream {
        fn as_fd(&self) -> BorrowedFd<'_> {
            panic!("STUB: not implemented");
        }
    }
}
cfg_windows! {
    use crate ::os::windows::io:: { AsRawSocket, RawSocket, AsSocket, BorrowedSocket };
    impl AsRawSocket for TcpStream { fn as_raw_socket(& self) -> RawSocket { self.io
    .as_raw_socket() } } impl AsSocket for TcpStream { fn as_socket(& self) ->
    BorrowedSocket <'_ > { unsafe { BorrowedSocket::borrow_raw(self.as_raw_socket()) } }
    }
}
#[cfg(all(tokio_unstable, target_os = "wasi"))]
mod sys {
    use super::TcpStream;
    use std::os::fd::{AsFd, AsRawFd, BorrowedFd, RawFd};
    impl AsRawFd for TcpStream {
        fn as_raw_fd(&self) -> RawFd {
            panic!("STUB: not implemented");
        }
    }
    impl AsFd for TcpStream {
        fn as_fd(&self) -> BorrowedFd<'_> {
            panic!("STUB: not implemented");
        }
    }
}
