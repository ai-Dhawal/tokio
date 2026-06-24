//! Asynchronous signal handling for Tokio.
//!
//! Note that signal handling is in general a very tricky topic and should be
//! used with great care. This crate attempts to implement 'best practice' for
//! signal handling, but it should be evaluated for your own applications' needs
//! to see if it's suitable.
//!
//! There are some fundamental limitations of this crate documented on the OS
//! specific structures, as well.
//!
//! # Examples
//!
//! Print on "ctrl-c" notification.
//!
//! ```rust,no_run
//! use tokio::signal;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     signal::ctrl_c().await?;
//!     println!("ctrl-c received!");
//!     Ok(())
//! }
//! ```
//!
//! Wait for `SIGHUP` on Unix
//!
//! ```rust,no_run
//! # #[cfg(unix)] {
//! use tokio::signal::unix::{signal, SignalKind};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // An infinite stream of hangup signals.
//!     let mut stream = signal(SignalKind::hangup())?;
//!
//!     // Print whenever a HUP signal is received
//!     loop {
//!         stream.recv().await;
//!         println!("got signal HUP");
//!     }
//! }
//! # }
//! ```
use crate::sync::watch::Receiver;
use std::task::{Context, Poll};
#[cfg(feature = "signal")]
mod ctrl_c;
#[cfg(feature = "signal")]
pub use ctrl_c::ctrl_c;
#[cfg(unix)]
pub(crate) mod registry;
pub mod unix;
pub mod windows;
mod reusable_box;
use self::reusable_box::ReusableBoxFuture;
#[derive(Debug)]
struct RxFuture {
    inner: ReusableBoxFuture<Receiver<()>>,
}
async fn make_future(mut rx: Receiver<()>) -> Receiver<()> {
    panic!("STUB: not implemented");
}
impl RxFuture {
    fn new(rx: Receiver<()>) -> Self {
        panic!("STUB: not implemented");
    }
    async fn recv(&mut self) {
        panic!("STUB: not implemented");
    }
    fn poll_recv(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        panic!("STUB: not implemented");
    }
}
