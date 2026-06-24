use crate::io::{Interest, PollEvented};
use crate::net::tcp::TcpStream;
use crate::util::check_socket_for_blocking;
cfg_not_wasip1! {
    use crate ::net:: { to_socket_addrs, ToSocketAddrs };
}
use std::fmt;
use std::io;
use std::net::{self, SocketAddr};
use std::task::{ready, Context, Poll};
cfg_net! {
    #[doc = " A TCP socket server, listening for connections."] #[doc = ""] #[doc =
    " You can accept a new connection by using the [`accept`](`TcpListener::accept`)"]
    #[doc = " method."] #[doc = ""] #[doc =
    " A `TcpListener` can be turned into a `Stream` with [`TcpListenerStream`]."] #[doc =
    ""] #[doc = " The socket will be closed when the value is dropped."] #[doc = ""]
    #[doc =
    " [`TcpListenerStream`]: https://docs.rs/tokio-stream/0.1/tokio_stream/wrappers/struct.TcpListenerStream.html"]
    #[doc = ""] #[doc = " # Errors"] #[doc = ""] #[doc =
    " Note that accepting a connection can lead to various errors and not all"] #[doc =
    " of them are necessarily fatal ‒ for example having too many open file"] #[doc =
    " descriptors or the other side closing the connection while it waits in"] #[doc =
    " an accept queue. These would terminate the stream if not handled in any"] #[doc =
    " way."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " Using `accept`:"]
    #[doc = " ```no_run"] #[doc = " use tokio::net::TcpListener;"] #[doc = ""] #[doc =
    " use std::io;"] #[doc = ""] #[doc = " async fn process_socket<T>(socket: T) {"]
    #[doc = "     # drop(socket);"] #[doc = "     // do work with socket here"] #[doc =
    " }"] #[doc = ""] #[doc = " #[tokio::main]"] #[doc =
    " async fn main() -> io::Result<()> {"] #[doc =
    "     let listener = TcpListener::bind(\"127.0.0.1:8080\").await?;"] #[doc = ""]
    #[doc = "     loop {"] #[doc =
    "         let (socket, _) = listener.accept().await?;"] #[doc =
    "         process_socket(socket).await;"] #[doc = "     }"] #[doc = " }"] #[doc =
    " ```"] pub struct TcpListener { io : PollEvented < mio::net::TcpListener >, }
}
impl TcpListener {
    cfg_not_wasip1! {
        #[doc =
        " Creates a new `TcpListener`, which will be bound to the specified address."]
        #[doc = ""] #[doc = " The returned listener is ready for accepting connections."]
        #[doc = ""] #[doc =
        " Binding with a port number of 0 will request that the OS assigns a port"] #[doc
        = " to this listener. The port allocated can be queried via the `local_addr`"]
        #[doc = " method."] #[doc = ""] #[doc =
        " The address type can be any implementor of the [`ToSocketAddrs`] trait."] #[doc
        = " If `addr` yields multiple addresses, bind will be attempted with each of"]
        #[doc = " the addresses until one succeeds and returns the listener. If none of"]
        #[doc = " the addresses succeed in creating a listener, the error returned from"]
        #[doc = " the last attempt (the last address) is returned."] #[doc = ""] #[doc =
        " This function sets the `SO_REUSEADDR` option on the socket on Unix."] #[doc =
        ""] #[doc =
        " To configure the socket before binding, you can use the [`TcpSocket`]"] #[doc =
        " type."] #[doc = ""] #[doc =
        " [`ToSocketAddrs`]: trait@crate::net::ToSocketAddrs"] #[doc =
        " [`TcpSocket`]: struct@crate::net::TcpSocket"] #[doc = ""] #[doc =
        " # Examples"] #[doc = ""] #[doc = " ```no_run"] #[doc =
        " use tokio::net::TcpListener;"] #[doc = " use std::io;"] #[doc = ""] #[doc =
        " #[tokio::main]"] #[doc = " async fn main() -> io::Result<()> {"] #[doc =
        "     let listener = TcpListener::bind(\"127.0.0.1:2345\").await?;"] #[doc = ""]
        #[doc = "     // use the listener"] #[doc = ""] #[doc =
        "     # let _ = listener;"] #[doc = "     Ok(())"] #[doc = " }"] #[doc = " ```"]
        pub async fn bind < A : ToSocketAddrs > (addr : A) -> io::Result < TcpListener >
        { let addrs = to_socket_addrs(addr). await ?; let mut last_err = None; for addr
        in addrs { match TcpListener::bind_addr(addr) { Ok(listener) => return
        Ok(listener), Err(e) => last_err = Some(e), } } Err(last_err.unwrap_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "could not resolve to any address",)
        })) } fn bind_addr(addr : SocketAddr) -> io::Result < TcpListener > { let
        listener = mio::net::TcpListener::bind(addr) ?; TcpListener::new(listener) }
    }
    /// Accepts a new incoming connection from this listener.
    ///
    /// This function will yield once a new TCP connection is established. When
    /// established, the corresponding [`TcpStream`] and the remote peer's
    /// address will be returned.
    ///
    /// # Cancel safety
    ///
    /// This method is cancel safe. If the method is used as a branch in
    /// [`tokio::select!`](crate::select) and another branch
    /// completes first, then it is guaranteed that no new connections were
    /// accepted by this method.
    ///
    /// [`TcpStream`]: struct@crate::net::TcpStream
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::net::TcpListener;
    ///
    /// use std::io;
    ///
    /// #[tokio::main]
    /// async fn main() -> io::Result<()> {
    ///     let listener = TcpListener::bind("127.0.0.1:8080").await?;
    ///
    ///     match listener.accept().await {
    ///         Ok((_socket, addr)) => println!("new client: {:?}", addr),
    ///         Err(e) => println!("couldn't get client: {:?}", e),
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn accept(&self) -> io::Result<(TcpStream, SocketAddr)> {
        panic!("STUB: not implemented");
    }
    /// Polls to accept a new incoming connection to this listener.
    ///
    /// If there is no connection to accept, `Poll::Pending` is returned and the
    /// current task will be notified by a waker.  Note that on multiple calls
    /// to `poll_accept`, only the `Waker` from the `Context` passed to the most
    /// recent call is scheduled to receive a wakeup.
    pub fn poll_accept(
        &self,
        cx: &mut Context<'_>,
    ) -> Poll<io::Result<(TcpStream, SocketAddr)>> {
        panic!("STUB: not implemented");
    }
    /// Creates new `TcpListener` from a `std::net::TcpListener`.
    ///
    /// This function is intended to be used to wrap a TCP listener from the
    /// standard library in the Tokio equivalent.
    ///
    /// This API is typically paired with the `socket2` crate and the `Socket`
    /// type to build up and customize a listener before it's shipped off to the
    /// backing event loop. This allows configuration of options like
    /// `SO_REUSEPORT`, binding to multiple addresses, etc.
    ///
    /// # Notes
    ///
    /// The caller is responsible for ensuring that the listener is in
    /// non-blocking mode. Otherwise all I/O operations on the listener
    /// will block the thread, which will cause unexpected behavior.
    /// Non-blocking mode can be set using [`set_nonblocking`].
    ///
    /// Passing a listener in blocking mode is always erroneous,
    /// and the behavior in that case may change in the future.
    /// For example, it could panic.
    ///
    /// [`set_nonblocking`]: std::net::TcpListener::set_nonblocking
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use std::error::Error;
    /// use tokio::net::TcpListener;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     let std_listener = std::net::TcpListener::bind("127.0.0.1:0")?;
    ///     std_listener.set_nonblocking(true)?;
    ///     let listener = TcpListener::from_std(std_listener)?;
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
    pub fn from_std(listener: net::TcpListener) -> io::Result<TcpListener> {
        panic!("STUB: not implemented");
    }
    /// Turns a [`tokio::net::TcpListener`] into a [`std::net::TcpListener`].
    ///
    /// The returned [`std::net::TcpListener`] will have nonblocking mode set as
    /// `true`.  Use [`set_nonblocking`] to change the blocking mode if needed.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use std::error::Error;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn Error>> {
    ///     let tokio_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await?;
    ///     let std_listener = tokio_listener.into_std()?;
    ///     std_listener.set_nonblocking(false)?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [`tokio::net::TcpListener`]: TcpListener
    /// [`std::net::TcpListener`]: std::net::TcpListener
    /// [`set_nonblocking`]: fn@std::net::TcpListener::set_nonblocking
    pub fn into_std(self) -> io::Result<std::net::TcpListener> {
        panic!("STUB: not implemented");
    }
    cfg_not_wasip1! {
        pub (crate) fn new(listener : mio::net::TcpListener) -> io::Result < TcpListener
        > { let io = PollEvented::new(listener) ?; Ok(TcpListener { io }) }
    }
    /// Returns the local address that this listener is bound to.
    ///
    /// This can be useful, for example, when binding to port 0 to figure out
    /// which port was actually bound.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use tokio::net::TcpListener;
    ///
    /// use std::io;
    /// use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
    ///
    /// #[tokio::main]
    /// async fn main() -> io::Result<()> {
    ///     let listener = TcpListener::bind("127.0.0.1:8080").await?;
    ///
    ///     assert_eq!(listener.local_addr()?,
    ///                SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080)));
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        panic!("STUB: not implemented");
    }
    /// Gets the value of the `IP_TTL` option for this socket.
    ///
    /// For more information about this option, see [`set_ttl`].
    ///
    /// [`set_ttl`]: method@Self::set_ttl
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::net::TcpListener;
    ///
    /// use std::io;
    ///
    /// #[tokio::main]
    /// async fn main() -> io::Result<()> {
    ///    let listener = TcpListener::bind("127.0.0.1:0").await?;
    ///
    ///    listener.set_ttl(100).expect("could not set TTL");
    ///    assert_eq!(listener.ttl()?, 100);
    ///
    ///    Ok(())
    /// }
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
    /// use tokio::net::TcpListener;
    ///
    /// use std::io;
    ///
    /// #[tokio::main]
    /// async fn main() -> io::Result<()> {
    ///     let listener = TcpListener::bind("127.0.0.1:0").await?;
    ///
    ///     listener.set_ttl(100).expect("could not set TTL");
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn set_ttl(&self, ttl: u32) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
}
impl TryFrom<net::TcpListener> for TcpListener {
    type Error = io::Error;
    /// Consumes stream, returning the tokio I/O object.
    ///
    /// This is equivalent to
    /// [`TcpListener::from_std(stream)`](TcpListener::from_std).
    fn try_from(stream: net::TcpListener) -> Result<Self, Self::Error> {
        panic!("STUB: not implemented");
    }
}
impl fmt::Debug for TcpListener {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
#[cfg(unix)]
mod sys {
    use super::TcpListener;
    use std::os::unix::prelude::*;
    impl AsRawFd for TcpListener {
        fn as_raw_fd(&self) -> RawFd {
            panic!("STUB: not implemented");
        }
    }
    impl AsFd for TcpListener {
        fn as_fd(&self) -> BorrowedFd<'_> {
            panic!("STUB: not implemented");
        }
    }
}
cfg_unstable! {
    #[cfg(target_os = "wasi")] mod sys { use super::TcpListener; use std::os::fd:: {
    RawFd, BorrowedFd, AsRawFd, AsFd }; impl AsRawFd for TcpListener { fn as_raw_fd(&
    self) -> RawFd { self.io.as_raw_fd() } } impl AsFd for TcpListener { fn as_fd(& self)
    -> BorrowedFd <'_ > { unsafe { BorrowedFd::borrow_raw(self.as_raw_fd()) } } } }
}
cfg_windows! {
    use crate ::os::windows::io:: { AsRawSocket, RawSocket, AsSocket, BorrowedSocket };
    impl AsRawSocket for TcpListener { fn as_raw_socket(& self) -> RawSocket { self.io
    .as_raw_socket() } } impl AsSocket for TcpListener { fn as_socket(& self) ->
    BorrowedSocket <'_ > { unsafe { BorrowedSocket::borrow_raw(self.as_raw_socket()) } }
    }
}
