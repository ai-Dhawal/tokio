use crate::{fs::asyncify, util::as_ref::OwnedBuf};
use std::{io, path::Path};
/// Creates a future that will open a file for writing and write the entire
/// contents of `contents` to it.
///
/// This is the async equivalent of [`std::fs::write`][std].
///
/// This operation is implemented by running the equivalent blocking operation
/// on a separate thread pool using [`spawn_blocking`].
///
/// [`spawn_blocking`]: crate::task::spawn_blocking
/// [std]: fn@std::fs::write
///
/// # Examples
///
/// ```no_run
/// use tokio::fs;
///
/// # async fn dox() -> std::io::Result<()> {
/// fs::write("foo.txt", b"Hello world!").await?;
/// # Ok(())
/// # }
/// ```
pub async fn write(
    path: impl AsRef<Path>,
    contents: impl AsRef<[u8]>,
) -> io::Result<()> {
    panic!("STUB: not implemented");
}
#[cfg(
    all(
        tokio_unstable,
        feature = "io-uring",
        feature = "rt",
        feature = "fs",
        target_os = "linux"
    )
)]
async fn write_uring(path: &Path, mut buf: OwnedBuf) -> io::Result<()> {
    panic!("STUB: not implemented");
}
async fn write_spawn_blocking(path: &Path, contents: OwnedBuf) -> io::Result<()> {
    panic!("STUB: not implemented");
}
