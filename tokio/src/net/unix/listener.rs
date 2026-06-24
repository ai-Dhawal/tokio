use crate::io::{Interest, PollEvented};
use crate::net::unix::{SocketAddr, UnixStream};
use crate::util::check_socket_for_blocking;
use std::fmt;
use std::io;
#[cfg(target_os = "android")]
use std::os::android::net::SocketAddrExt;
#[cfg(target_os = "linux")]
use std::os::linux::net::SocketAddrExt;
#[cfg(any(target_os = "linux", target_os = "android"))]
use std::os::unix::ffi::OsStrExt;
use std::os::unix::io::{AsFd, AsRawFd, BorrowedFd, FromRawFd, IntoRawFd, RawFd};
use std::os::unix::net::{self, SocketAddr as StdSocketAddr};
use std::path::Path;
use std::task::{ready, Context, Poll};
cfg_net_unix! {
    #[doc = " A Unix socket which can accept connections from other Unix sockets."] #[doc
    = ""] #[doc =
    " You can accept a new connection by using the [`accept`](`UnixListener::accept`) method."]
    #[doc = ""] #[doc =
    " A `UnixListener` can be turned into a `Stream` with [`UnixListenerStream`]."] #[doc
    = ""] #[doc =
    " [`UnixListenerStream`]: https://docs.rs/tokio-stream/0.1/tokio_stream/wrappers/struct.UnixListenerStream.html"]
    #[doc = ""] #[doc = " # Errors"] #[doc = ""] #[doc =
    " Note that accepting a connection can lead to various errors and not all"] #[doc =
    " of them are necessarily fatal ‒ for example having too many open file"] #[doc =
    " descriptors or the other side closing the connection while it waits in"] #[doc =
    " an accept queue. These would terminate the stream if not handled in any"] #[doc =
    " way."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc = " ```no_run"] #[doc =
    " use tokio::net::UnixListener;"] #[doc = ""] #[doc = " #[tokio::main]"] #[doc =
    " async fn main() {"] #[doc =
    "     let listener = UnixListener::bind(\"/path/to/the/socket\").unwrap();"] #[doc =
    "     loop {"] #[doc = "         match listener.accept().await {"] #[doc =
    "             Ok((stream, _addr)) => {"] #[doc =
    "                 println!(\"new client!\");"] #[doc = "             }"] #[doc =
    "             Err(e) => { /* connection failed */ }"] #[doc = "         }"] #[doc =
    "     }"] #[doc = " }"] #[doc = " ```"] #[cfg_attr(docsrs, doc(alias = "uds"))] pub
    struct UnixListener { io : PollEvented < mio::net::UnixListener >, }
}
impl UnixListener {
    pub(crate) fn new(listener: mio::net::UnixListener) -> io::Result<UnixListener> {
        panic!("STUB: not implemented");
    }
    /// Creates a new `UnixListener` bound to the specified path.
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
    pub fn bind<P>(path: P) -> io::Result<UnixListener>
    where
        P: AsRef<Path>,
    {
        panic!("STUB: not implemented");
    }
    /// Creates a new `UnixListener` bound to the specified address.
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
    pub fn bind_addr(socket_addr: &SocketAddr) -> io::Result<UnixListener> {
        panic!("STUB: not implemented");
    }
    /// Creates new [`UnixListener`] from a [`std::os::unix::net::UnixListener`].
    ///
    /// This function is intended to be used to wrap a `UnixListener` from the
    /// standard library in the Tokio equivalent.
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
    /// [`set_nonblocking`]: std::os::unix::net::UnixListener::set_nonblocking
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::net::UnixListener;
    /// use std::os::unix::net::UnixListener as StdUnixListener;
    /// # use std::error::Error;
    ///
    /// # async fn dox() -> Result<(), Box<dyn Error>> {
    /// let std_listener = StdUnixListener::bind("/path/to/the/socket")?;
    /// std_listener.set_nonblocking(true)?;
    /// let listener = UnixListener::from_std(std_listener)?;
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
    pub fn from_std(listener: net::UnixListener) -> io::Result<UnixListener> {
        panic!("STUB: not implemented");
    }
    /// Turns a [`tokio::net::UnixListener`] into a [`std::os::unix::net::UnixListener`].
    ///
    /// The returned [`std::os::unix::net::UnixListener`] will have nonblocking mode
    /// set as `true`. Use [`set_nonblocking`] to change the blocking mode if needed.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use std::error::Error;
    /// # async fn dox() -> Result<(), Box<dyn Error>> {
    /// let tokio_listener = tokio::net::UnixListener::bind("/path/to/the/socket")?;
    /// let std_listener = tokio_listener.into_std()?;
    /// std_listener.set_nonblocking(false)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// [`tokio::net::UnixListener`]: UnixListener
    /// [`std::os::unix::net::UnixListener`]: std::os::unix::net::UnixListener
    /// [`set_nonblocking`]: fn@std::os::unix::net::UnixListener::set_nonblocking
    pub fn into_std(self) -> io::Result<std::os::unix::net::UnixListener> {
        panic!("STUB: not implemented");
    }
    /// Returns the local socket address of this listener.
    pub fn local_addr(&self) -> io::Result<SocketAddr> {
        panic!("STUB: not implemented");
    }
    /// Returns the value of the `SO_ERROR` option.
    pub fn take_error(&self) -> io::Result<Option<io::Error>> {
        panic!("STUB: not implemented");
    }
    /// Accepts a new incoming connection to this listener.
    ///
    /// # Cancel safety
    ///
    /// This method is cancel safe. If the method is used as a branch in
    /// [`tokio::select!`](crate::select) and another branch
    /// completes first, then it is guaranteed that no new connections were
    /// accepted by this method.
    pub async fn accept(&self) -> io::Result<(UnixStream, SocketAddr)> {
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
    ) -> Poll<io::Result<(UnixStream, SocketAddr)>> {
        panic!("STUB: not implemented");
    }
}
impl TryFrom<std::os::unix::net::UnixListener> for UnixListener {
    type Error = io::Error;
    /// Consumes stream, returning the tokio I/O object.
    ///
    /// This is equivalent to
    /// [`UnixListener::from_std(stream)`](UnixListener::from_std).
    fn try_from(stream: std::os::unix::net::UnixListener) -> io::Result<Self> {
        panic!("STUB: not implemented");
    }
}
impl fmt::Debug for UnixListener {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl AsRawFd for UnixListener {
    fn as_raw_fd(&self) -> RawFd {
        panic!("STUB: not implemented");
    }
}
impl AsFd for UnixListener {
    fn as_fd(&self) -> BorrowedFd<'_> {
        panic!("STUB: not implemented");
    }
}
