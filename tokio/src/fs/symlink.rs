use crate::fs::asyncify;
use std::io;
use std::path::Path;
/// Creates a new symbolic link on the filesystem.
///
/// The `link` path will be a symbolic link pointing to the `original` path.
///
/// This is an async version of [`std::os::unix::fs::symlink`].
pub async fn symlink(
    original: impl AsRef<Path>,
    link: impl AsRef<Path>,
) -> io::Result<()> {
    panic!("STUB: not implemented");
}
