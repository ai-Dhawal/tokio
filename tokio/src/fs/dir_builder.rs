use crate::fs::asyncify;
use std::io;
use std::path::Path;
/// A builder for creating directories in various manners.
///
/// This is a specialized version of [`std::fs::DirBuilder`] for usage on
/// the Tokio runtime.
#[derive(Debug, Default)]
pub struct DirBuilder {
    /// Indicates whether to create parent directories if they are missing.
    recursive: bool,
    /// Sets the Unix mode for newly created directories.
    #[cfg(unix)]
    pub(super) mode: Option<u32>,
}
impl DirBuilder {
    /// Creates a new set of options with default mode/security settings for all
    /// platforms and also non-recursive.
    ///
    /// This is an async version of [`std::fs::DirBuilder::new`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::fs::DirBuilder;
    ///
    /// let builder = DirBuilder::new();
    /// ```
    pub fn new() -> Self {
        panic!("STUB: not implemented");
    }
    /// Indicates whether to create directories recursively (including all parent directories).
    /// Parents that do not exist are created with the same security and permissions settings.
    ///
    /// This option defaults to `false`.
    ///
    /// This is an async version of [`std::fs::DirBuilder::recursive`].
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::fs::DirBuilder;
    ///
    /// let mut builder = DirBuilder::new();
    /// builder.recursive(true);
    /// ```
    pub fn recursive(&mut self, recursive: bool) -> &mut Self {
        panic!("STUB: not implemented");
    }
    /// Creates the specified directory with the configured options.
    ///
    /// It is considered an error if the directory already exists unless
    /// recursive mode is enabled.
    ///
    /// This is an async version of [`std::fs::DirBuilder::create`].
    ///
    /// # Errors
    ///
    /// An error will be returned under the following circumstances:
    ///
    /// * Path already points to an existing file.
    /// * Path already points to an existing directory and the mode is
    ///   non-recursive.
    /// * The calling process doesn't have permissions to create the directory
    ///   or its missing parents.
    /// * Other I/O error occurred.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::fs::DirBuilder;
    /// use std::io;
    ///
    /// #[tokio::main]
    /// async fn main() -> io::Result<()> {
    ///     DirBuilder::new()
    ///         .recursive(true)
    ///         .create("/tmp/foo/bar/baz")
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn create(&self, path: impl AsRef<Path>) -> io::Result<()> {
        panic!("STUB: not implemented");
    }
}
feature! {
    #![unix] impl DirBuilder { #[doc = " Sets the mode to create new directories with."]
    #[doc = ""] #[doc = " This option defaults to 0o777."] #[doc = ""] #[doc =
    " # Examples"] #[doc = ""] #[doc = ""] #[doc = " ```no_run"] #[doc =
    " use tokio::fs::DirBuilder;"] #[doc = ""] #[doc =
    " let mut builder = DirBuilder::new();"] #[doc = " builder.mode(0o775);"] #[doc =
    " ```"] pub fn mode(& mut self, mode : u32) -> & mut Self { self.mode = Some(mode);
    self } }
}
