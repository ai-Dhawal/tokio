use super::copy::CopyBuffer;
use crate::io::{AsyncRead, AsyncWrite};
use std::future::poll_fn;
use std::io;
use std::pin::Pin;
use std::task::{ready, Context, Poll};
enum TransferState {
    Running(CopyBuffer),
    ShuttingDown(u64),
    Done(u64),
}
fn transfer_one_direction<A, B>(
    cx: &mut Context<'_>,
    state: &mut TransferState,
    r: &mut A,
    w: &mut B,
) -> Poll<io::Result<u64>>
where
    A: AsyncRead + AsyncWrite + Unpin + ?Sized,
    B: AsyncRead + AsyncWrite + Unpin + ?Sized,
{
    panic!("STUB: not implemented");
}
/// Copies data in both directions between `a` and `b`.
///
/// This function returns a future that will read from both streams,
/// writing any data read to the opposing stream.
/// This happens in both directions concurrently.
///
/// If an EOF is observed on one stream, [`shutdown()`] will be invoked on
/// the other, and reading from that stream will stop. Copying of data in
/// the other direction will continue.
///
/// The future will complete successfully once both directions of communication has been shut down.
/// A direction is shut down when the reader reports EOF,
/// at which point [`shutdown()`] is called on the corresponding writer. When finished,
/// it will return a tuple of the number of bytes copied from a to b
/// and the number of bytes copied from b to a, in that order.
///
/// It uses two 8 KB buffers for transferring bytes between `a` and `b` by default.
/// To set your own buffers sizes use [`copy_bidirectional_with_sizes()`].
///
/// [`shutdown()`]: crate::io::AsyncWriteExt::shutdown
///
/// # Errors
///
/// The future will immediately return an error if any IO operation on `a`
/// or `b` returns an error. Some data read from either stream may be lost (not
/// written to the other stream) in this case.
///
/// # Return value
///
/// Returns a tuple of bytes copied `a` to `b` and bytes copied `b` to `a`.
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
pub async fn copy_bidirectional<A, B>(a: &mut A, b: &mut B) -> io::Result<(u64, u64)>
where
    A: AsyncRead + AsyncWrite + Unpin + ?Sized,
    B: AsyncRead + AsyncWrite + Unpin + ?Sized,
{
    panic!("STUB: not implemented");
}
/// Copies data in both directions between `a` and `b` using buffers of the specified size.
///
/// This method is the same as the [`copy_bidirectional()`], except that it allows you to set the
/// size of the internal buffers used when copying data.
#[cfg_attr(docsrs, doc(cfg(feature = "io-util")))]
pub async fn copy_bidirectional_with_sizes<A, B>(
    a: &mut A,
    b: &mut B,
    a_to_b_buf_size: usize,
    b_to_a_buf_size: usize,
) -> io::Result<(u64, u64)>
where
    A: AsyncRead + AsyncWrite + Unpin + ?Sized,
    B: AsyncRead + AsyncWrite + Unpin + ?Sized,
{
    panic!("STUB: not implemented");
}
async fn copy_bidirectional_impl<A, B>(
    a: &mut A,
    b: &mut B,
    a_to_b_buffer: CopyBuffer,
    b_to_a_buffer: CopyBuffer,
) -> io::Result<(u64, u64)>
where
    A: AsyncRead + AsyncWrite + Unpin + ?Sized,
    B: AsyncRead + AsyncWrite + Unpin + ?Sized,
{
    panic!("STUB: not implemented");
}
