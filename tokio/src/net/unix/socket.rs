use std::io;
use std::path::Path;
use std::os::unix::io::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, RawFd};
use crate::net::{UnixDatagram, UnixListener, UnixStream};
cfg_net_unix! {
    #[doc =
    " A Unix socket that has not yet been converted to a [`UnixStream`], [`UnixDatagram`], or"]
    #[doc = " [`UnixListener`]."] #[doc = ""] #[doc =
    " `UnixSocket` wraps an operating system socket and enables the caller to"] #[doc =
    " configure the socket before establishing a connection or accepting"] #[doc =
    " inbound connections. The caller is able to set socket option and explicitly"] #[doc
    = " bind the socket with a socket address."] #[doc = ""] #[doc =
    " The underlying socket is closed when the `UnixSocket` value is dropped."] #[doc =
    ""] #[doc =
    " `UnixSocket` should only be used directly if the default configuration used"] #[doc
    = " by [`UnixStream::connect`], [`UnixDatagram::bind`], and [`UnixListener::bind`]"]
    #[doc = " does not meet the required use case."] #[doc = ""] #[doc =
    " Calling `UnixStream::connect(path)` effectively performs the same function as:"]
    #[doc = ""] #[doc = " ```no_run"] #[doc = " use tokio::net::UnixSocket;"] #[doc =
    " use std::error::Error;"] #[doc = ""] #[doc = " #[tokio::main]"] #[doc =
    " async fn main() -> Result<(), Box<dyn Error>> {"] #[doc =
    "     let dir = tempfile::tempdir().unwrap();"] #[doc =
    "     let path = dir.path().join(\"bind_path\");"] #[doc =
    "     let socket = UnixSocket::new_stream()?;"] #[doc = ""] #[doc =
    "     let stream = socket.connect(path).await?;"] #[doc = ""] #[doc = "     Ok(())"]
    #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc =
    " Calling `UnixDatagram::bind(path)` effectively performs the same function as:"]
    #[doc = ""] #[doc = " ```no_run"] #[doc = " use tokio::net::UnixSocket;"] #[doc =
    " use std::error::Error;"] #[doc = ""] #[doc = " #[tokio::main]"] #[doc =
    " async fn main() -> Result<(), Box<dyn Error>> {"] #[doc =
    "     let dir = tempfile::tempdir().unwrap();"] #[doc =
    "     let path = dir.path().join(\"bind_path\");"] #[doc =
    "     let socket = UnixSocket::new_datagram()?;"] #[doc = "     socket.bind(path)?;"]
    #[doc = ""] #[doc = "     let datagram = socket.datagram()?;"] #[doc = ""] #[doc =
    "     Ok(())"] #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc =
    " Calling `UnixListener::bind(path)` effectively performs the same function as:"]
    #[doc = ""] #[doc = " ```no_run"] #[doc = " use tokio::net::UnixSocket;"] #[doc =
    " use std::error::Error;"] #[doc = ""] #[doc = " #[tokio::main]"] #[doc =
    " async fn main() -> Result<(), Box<dyn Error>> {"] #[doc =
    "     let dir = tempfile::tempdir().unwrap();"] #[doc =
    "     let path = dir.path().join(\"bind_path\");"] #[doc =
    "     let socket = UnixSocket::new_stream()?;"] #[doc = "     socket.bind(path)?;"]
    #[doc = ""] #[doc = "     let listener = socket.listen(1024)?;"] #[doc = ""] #[doc =
    "     Ok(())"] #[doc = " }"] #[doc = " ```"] #[doc = ""] #[doc =
    " Setting socket options not explicitly provided by `UnixSocket` may be done by"]
    #[doc =
    " accessing the [`RawFd`]/[`RawSocket`] using [`AsRawFd`]/[`AsRawSocket`] and"] #[doc
    = " setting the option with a crate like [`socket2`]."] #[doc = ""] #[doc =
    " [`RawFd`]: std::os::fd::RawFd"] #[doc =
    " [`RawSocket`]: https://doc.rust-lang.org/std/os/windows/io/type.RawSocket.html"]
    #[doc = " [`AsRawFd`]: std::os::fd::AsRawFd"] #[doc =
    " [`AsRawSocket`]: https://doc.rust-lang.org/std/os/windows/io/trait.AsRawSocket.html"]
    #[doc = " [`socket2`]: https://docs.rs/socket2/"] #[derive(Debug)] pub struct
    UnixSocket { inner : socket2::Socket, }
}
impl UnixSocket {
    fn ty(&self) -> socket2::Type {
        panic!("STUB: not implemented");
    }
    /// Creates a new Unix datagram socket.
    ///
    /// Calls `socket(2)` with `AF_UNIX` and `SOCK_DGRAM`.
    ///
    /// # Returns
    ///
    /// On success, the newly created [`UnixSocket`] is returned. If an error is
    /// encountered, it is returned instead.
    pub fn new_datagram() -> io::Result<UnixSocket> {
        panic!("STUB: not implemented");
    }
    /// Creates a new Unix stream socket.
    ///
    /// Calls `socket(2)` with `AF_UNIX` and `SOCK_STREAM`.
    ///
    /// # Returns
    ///
    /// On success, the newly created [`UnixSocket`] is returned. If an error is
    /// encountered, it is returned instead.
    pub fn new_stream() -> io::Result<UnixSocket> {
        panic!("STUB: not implemented");
    }
    fn new(ty: socket2::Type) -> io::Result<UnixSocket> {
        panic!("STUB: not implemented");
    }
    /// Binds the socket to the given address.
    ///
    /// This calls the `bind(2)` operating-system function.
    pub fn bind(&self, path: impl AsRef<Path>) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
    /// Converts the socket into a `UnixListener`.
    ///
    /// `backlog` defines the maximum number of pending connections are queued
    /// by the operating system at any given time. Connection are removed from
    /// the queue with [`UnixListener::accept`]. When the queue is full, the
    /// operating-system will start rejecting connections.
    ///
    /// Calling this function on a socket created by [`new_datagram`] will return an error.
    ///
    /// This calls the `listen(2)` operating-system function, marking the socket
    /// as a passive socket.
    ///
    /// [`new_datagram`]: `UnixSocket::new_datagram`
    pub fn listen(self, backlog: u32) -> io::Result<UnixListener> {
        panic!("STUB: not implemented");
    }
    /// Establishes a Unix connection with a peer at the specified socket address.
    ///
    /// The `UnixSocket` is consumed. Once the connection is established, a
    /// connected [`UnixStream`] is returned. If the connection fails, the
    /// encountered error is returned.
    ///
    /// Calling this function on a socket created by [`new_datagram`] will return an error.
    ///
    /// This calls the `connect(2)` operating-system function.
    ///
    /// [`new_datagram`]: `UnixSocket::new_datagram`
    pub async fn connect(self, path: impl AsRef<Path>) -> io::Result<UnixStream> {
        panic!("STUB: not implemented");
    }
    /// Converts the socket into a [`UnixDatagram`].
    ///
    /// Calling this function on a socket created by [`new_stream`] will return an error.
    ///
    /// [`new_stream`]: `UnixSocket::new_stream`
    pub fn datagram(self) -> io::Result<UnixDatagram> {
        panic!("STUB: not implemented");
    }
}
impl AsRawFd for UnixSocket {
    fn as_raw_fd(&self) -> RawFd {
        panic!("STUB: not implemented");
    }
}
impl AsFd for UnixSocket {
    fn as_fd(&self) -> BorrowedFd<'_> {
        panic!("STUB: not implemented");
    }
}
impl FromRawFd for UnixSocket {
    unsafe fn from_raw_fd(fd: RawFd) -> UnixSocket {
        panic!("STUB: not implemented");
    }
}
impl IntoRawFd for UnixSocket {
    fn into_raw_fd(self) -> RawFd {
        panic!("STUB: not implemented");
    }
}
