use crate::io::util::read_until::read_until_internal;
use crate::io::AsyncBufRead;
use pin_project_lite::pin_project;
use std::future::Future;
use std::io;
use std::marker::PhantomPinned;
use std::mem;
use std::pin::Pin;
use std::string::FromUtf8Error;
use std::task::{ready, Context, Poll};
pin_project! {
    #[doc =
    " Future for the [`read_line`](crate::io::AsyncBufReadExt::read_line) method."]
    #[derive(Debug)] #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct ReadLine <'a, R : ? Sized > { reader : &'a mut R, output : &'a mut String,
    buf : Vec < u8 >, read : usize, #[pin] _pin : PhantomPinned, }
}
pub(crate) fn read_line<'a, R>(
    reader: &'a mut R,
    string: &'a mut String,
) -> ReadLine<'a, R>
where
    R: AsyncBufRead + ?Sized + Unpin,
{
    panic!("STUB: not implemented");
}
fn put_back_original_data(
    output: &mut String,
    mut vector: Vec<u8>,
    num_bytes_read: usize,
) {
    panic!("STUB: not implemented");
}
/// This handles the various failure cases and puts the string back into `output`.
///
/// The `truncate_on_io_error` `bool` is necessary because `read_to_string` and `read_line`
/// disagree on what should happen when an IO error occurs.
pub(super) fn finish_string_read(
    io_res: io::Result<usize>,
    utf8_res: Result<String, FromUtf8Error>,
    read: usize,
    output: &mut String,
    truncate_on_io_error: bool,
) -> Poll<io::Result<usize>> {
    panic!("STUB: not implemented");
}
pub(super) fn read_line_internal<R: AsyncBufRead + ?Sized>(
    reader: Pin<&mut R>,
    cx: &mut Context<'_>,
    output: &mut String,
    buf: &mut Vec<u8>,
    read: &mut usize,
) -> Poll<io::Result<usize>> {
    panic!("STUB: not implemented");
}
impl<R: AsyncBufRead + ?Sized + Unpin> Future for ReadLine<'_, R> {
    type Output = io::Result<usize>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        panic!("STUB: not implemented");
    }
}
