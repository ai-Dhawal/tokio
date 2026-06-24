use crate::fs::asyncify;
use std::io;
use std::path::Path;
/// Returns `Ok(true)` if the path points at an existing entity.
///
/// This function will traverse symbolic links to query information about the
/// destination file. In case of broken symbolic links this will return `Ok(false)`.
///
/// This is the async equivalent of [`std::path::Path::try_exists`][std].
///
/// [std]: fn@std::path::Path::try_exists
///
/// # Examples
///
/// ```no_run
/// use tokio::fs;
///
/// # async fn dox() -> std::io::Result<()> {
/// fs::try_exists("foo.txt").await?;
/// # Ok(())
/// # }
/// ```
pub async fn try_exists(path: impl AsRef<Path>) -> io::Result<bool> {
    panic!("STUB: not implemented");
}
cfg_io_uring! {
    #[inline] #[cfg(any(target_env = "gnu", target_os = "android"))] async fn
    try_exists_uring(path : & Path) -> io::Result < bool > { use crate
    ::runtime::driver::op::Op; match Op::metadata(path) ?. await { Ok(_) => Ok(true),
    Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false), Err(error) =>
    Err(error), } }
}
async fn try_exists_spawn_blocking(path: &Path) -> io::Result<bool> {
    panic!("STUB: not implemented");
}
