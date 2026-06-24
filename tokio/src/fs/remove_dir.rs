use crate::fs::asyncify;
use std::io;
use std::path::Path;
/// Removes an existing, empty directory.
///
/// This is an async version of [`std::fs::remove_dir`].
pub async fn remove_dir(path: impl AsRef<Path>) -> io::Result<()> {
    panic!("STUB: not implemented");
}
