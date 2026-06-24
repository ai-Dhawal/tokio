use crate::io::util::poll_proceed_and_make_progress;
use crate::io::AsyncWrite;
use std::fmt;
use std::io;
use std::pin::Pin;
use std::task::{ready, Context, Poll};
cfg_io_util! {
    #[doc = " An async writer which will move data into the void."] #[doc = ""] #[doc =
    " This struct is generally created by calling [`sink`][sink]. Please"] #[doc =
    " see the documentation of `sink()` for more details."] #[doc = ""] #[doc =
    " This is an asynchronous version of [`std::io::Sink`][std]."] #[doc = ""] #[doc =
    " [sink]: sink()"] #[doc = " [std]: std::io::Sink"] pub struct Sink { _p : (), }
    #[doc =
    " Creates an instance of an async writer which will successfully consume all"] #[doc
    = " data."] #[doc = ""] #[doc =
    " All calls to [`poll_write`] on the returned instance will return"] #[doc =
    " `Poll::Ready(Ok(buf.len()))` and the contents of the buffer will not be"] #[doc =
    " inspected."] #[doc = ""] #[doc =
    " This is an asynchronous version of [`std::io::sink`][std]."] #[doc = ""] #[doc =
    " [`poll_write`]: crate::io::AsyncWrite::poll_write()"] #[doc =
    " [std]: std::io::sink"] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc =
    " ```"] #[doc = " use tokio::io::{self, AsyncWriteExt};"] #[doc = ""] #[doc =
    " # #[tokio::main(flavor = \"current_thread\")]"] #[doc =
    " # async fn main() -> io::Result<()> {"] #[doc =
    " let buffer = vec![1, 2, 3, 5, 8];"] #[doc =
    " let num_bytes = io::sink().write(&buffer).await?;"] #[doc =
    " assert_eq!(num_bytes, 5);"] #[doc = " Ok(())"] #[doc = " # }"] #[doc = " ```"] pub
    fn sink() -> Sink { Sink { _p : () } }
}
impl AsyncWrite for Sink {
    #[inline]
    fn poll_write(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &[u8],
    ) -> Poll<Result<usize, io::Error>> {
        panic!("STUB: not implemented");
    }
    #[inline]
    fn poll_flush(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), io::Error>> {
        panic!("STUB: not implemented");
    }
    #[inline]
    fn poll_shutdown(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<(), io::Error>> {
        panic!("STUB: not implemented");
    }
}
impl fmt::Debug for Sink {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn assert_unpin() {
        crate::is_unpin::<Sink>();
    }
}
