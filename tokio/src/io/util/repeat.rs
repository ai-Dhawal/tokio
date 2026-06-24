use bytes::BufMut;
use crate::io::util::poll_proceed_and_make_progress;
use crate::io::{AsyncRead, ReadBuf};
use std::io;
use std::pin::Pin;
use std::task::{ready, Context, Poll};
cfg_io_util! {
    #[doc = " An async reader which yields one byte over and over and over and over and"]
    #[doc = " over and..."] #[doc = ""] #[doc =
    " This struct is generally created by calling [`repeat`][repeat]. Please"] #[doc =
    " see the documentation of `repeat()` for more details."] #[doc = ""] #[doc =
    " This is an asynchronous version of [`std::io::Repeat`][std]."] #[doc = ""] #[doc =
    " [repeat]: fn@repeat"] #[doc = " [std]: std::io::Repeat"] #[derive(Debug)] pub
    struct Repeat { byte : u8, } #[doc =
    " Creates an instance of an async reader that infinitely repeats one byte."] #[doc =
    ""] #[doc =
    " All reads from this reader will succeed by filling the specified buffer with"]
    #[doc = " the given byte."] #[doc = ""] #[doc =
    " This is an asynchronous version of [`std::io::repeat`][std]."] #[doc = ""] #[doc =
    " [std]: std::io::repeat"] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc =
    " ```"] #[doc = " use tokio::io::{self, AsyncReadExt};"] #[doc = ""] #[doc =
    " # #[tokio::main(flavor = \"current_thread\")]"] #[doc = " # async fn main() {"]
    #[doc = " let mut buffer = [0; 3];"] #[doc =
    " io::repeat(0b101).read_exact(&mut buffer).await.unwrap();"] #[doc =
    " assert_eq!(buffer, [0b101, 0b101, 0b101]);"] #[doc = " # }"] #[doc = " ```"] pub fn
    repeat(byte : u8) -> Repeat { Repeat { byte } }
}
impl AsyncRead for Repeat {
    #[inline]
    fn poll_read(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        panic!("STUB: not implemented");
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn assert_unpin() {
        crate::is_unpin::<Repeat>();
    }
}
