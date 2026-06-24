use crate::fs::{asyncify, File};
use std::io;
use std::path::Path;
cfg_io_uring! {
    mod uring_open_options; pub (crate) use uring_open_options::UringOpenOptions; use
    crate ::runtime::driver::op::Op;
}
#[cfg(test)]
mod mock_open_options;
#[cfg(test)]
use mock_open_options::MockOpenOptions as StdOpenOptions;
#[cfg(not(test))]
use std::fs::OpenOptions as StdOpenOptions;
#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;
#[cfg(windows)]
use std::os::windows::fs::OpenOptionsExt;
/// Options and flags which can be used to configure how a file is opened.
///
/// This builder exposes the ability to configure how a [`File`] is opened and
/// what operations are permitted on the open file. The [`File::open`] and
/// [`File::create`] methods are aliases for commonly used options using this
/// builder.
///
/// Generally speaking, when using `OpenOptions`, you'll first call [`new`],
/// then chain calls to methods to set each option, then call [`open`], passing
/// the path of the file you're trying to open. This will give you a
/// [`io::Result`] with a [`File`] inside that you can further operate
/// on.
///
/// This is a specialized version of [`std::fs::OpenOptions`] for usage from
/// the Tokio runtime.
///
/// `From<std::fs::OpenOptions>` is implemented for more advanced configuration
/// than the methods provided here.
///
/// [`new`]: OpenOptions::new
/// [`open`]: OpenOptions::open
/// [`File`]: File
/// [`File::open`]: File::open
/// [`File::create`]: File::create
///
/// # Examples
///
/// Opening a file to read:
///
/// ```no_run
/// use tokio::fs::OpenOptions;
/// use std::io;
///
/// #[tokio::main]
/// async fn main() -> io::Result<()> {
///     let file = OpenOptions::new()
///         .read(true)
///         .open("foo.txt")
///         .await?;
///
///     Ok(())
/// }
/// ```
///
/// Opening a file for both reading and writing, as well as creating it if it
/// doesn't exist:
///
/// ```no_run
/// use tokio::fs::OpenOptions;
/// use std::io;
///
/// #[tokio::main]
/// async fn main() -> io::Result<()> {
///     let file = OpenOptions::new()
///         .read(true)
///         .write(true)
///         .create(true)
///         .open("foo.txt")
///         .await?;
///
///     Ok(())
/// }
/// ```
#[derive(Clone, Debug)]
pub struct OpenOptions {
    inner: Kind,
}
#[derive(Debug, Clone)]
enum Kind {
    Std(StdOpenOptions),
    #[cfg(
        all(
            tokio_unstable,
            feature = "io-uring",
            feature = "rt",
            feature = "fs",
            target_os = "linux"
        )
    )]
    Uring(UringOpenOptions),
}
impl OpenOptions {
    /// Creates a blank new set of options ready for configuration.
    ///
    /// All options are initially set to `false`.
    ///
    /// This is an async version of [`std::fs::OpenOptions::new`][std]
    ///
    /// [std]: std::fs::OpenOptions::new
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::fs::OpenOptions;
    ///
    /// let mut options = OpenOptions::new();
    /// let future = options.read(true).open("foo.txt");
    /// ```
    pub fn new() -> OpenOptions {
        panic!("STUB: not implemented");
    }
    /// Sets the option for read access.
    ///
    /// This option, when true, will indicate that the file should be
    /// `read`-able if opened.
    ///
    /// This is an async version of [`std::fs::OpenOptions::read`][std]
    ///
    /// [std]: std::fs::OpenOptions::read
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::fs::OpenOptions;
    /// use std::io;
    ///
    /// #[tokio::main]
    /// async fn main() -> io::Result<()> {
    ///     let file = OpenOptions::new()
    ///         .read(true)
    ///         .open("foo.txt")
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn read(&mut self, read: bool) -> &mut OpenOptions {
        panic!("STUB: not implemented");
    }
    /// Sets the option for write access.
    ///
    /// This option, when true, will indicate that the file should be
    /// `write`-able if opened.
    ///
    /// This is an async version of [`std::fs::OpenOptions::write`][std]
    ///
    /// [std]: std::fs::OpenOptions::write
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::fs::OpenOptions;
    /// use std::io;
    ///
    /// #[tokio::main]
    /// async fn main() -> io::Result<()> {
    ///     let file = OpenOptions::new()
    ///         .write(true)
    ///         .open("foo.txt")
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn write(&mut self, write: bool) -> &mut OpenOptions {
        panic!("STUB: not implemented");
    }
    /// Sets the option for the append mode.
    ///
    /// This option, when true, means that writes will append to a file instead
    /// of overwriting previous contents.  Note that setting
    /// `.write(true).append(true)` has the same effect as setting only
    /// `.append(true)`.
    ///
    /// For most filesystems, the operating system guarantees that all writes are
    /// atomic: no writes get mangled because another process writes at the same
    /// time.
    ///
    /// One maybe obvious note when using append-mode: make sure that all data
    /// that belongs together is written to the file in one operation. This
    /// can be done by concatenating strings before passing them to [`write()`],
    /// or using a buffered writer (with a buffer of adequate size),
    /// and calling [`flush()`] when the message is complete.
    ///
    /// If a file is opened with both read and append access, beware that after
    /// opening, and after every write, the position for reading may be set at the
    /// end of the file. So, before writing, save the current position (using
    /// [`seek`]`(`[`SeekFrom`]`::`[`Current`]`(0))`), and restore it before the next read.
    ///
    /// This is an async version of [`std::fs::OpenOptions::append`][std]
    ///
    /// [std]: std::fs::OpenOptions::append
    ///
    /// ## Note
    ///
    /// This function doesn't create the file if it doesn't exist. Use the [`create`]
    /// method to do so.
    ///
    /// [`write()`]: crate::io::AsyncWriteExt::write
    /// [`flush()`]: crate::io::AsyncWriteExt::flush
    /// [`seek`]: crate::io::AsyncSeekExt::seek
    /// [`SeekFrom`]: std::io::SeekFrom
    /// [`Current`]: std::io::SeekFrom::Current
    /// [`create`]: OpenOptions::create
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::fs::OpenOptions;
    /// use std::io;
    ///
    /// #[tokio::main]
    /// async fn main() -> io::Result<()> {
    ///     let file = OpenOptions::new()
    ///         .append(true)
    ///         .open("foo.txt")
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn append(&mut self, append: bool) -> &mut OpenOptions {
        panic!("STUB: not implemented");
    }
    /// Sets the option for truncating a previous file.
    ///
    /// If a file is successfully opened with this option set it will truncate
    /// the file to 0 length if it already exists.
    ///
    /// The file must be opened with write access for truncate to work.
    ///
    /// This is an async version of [`std::fs::OpenOptions::truncate`][std]
    ///
    /// [std]: std::fs::OpenOptions::truncate
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::fs::OpenOptions;
    /// use std::io;
    ///
    /// #[tokio::main]
    /// async fn main() -> io::Result<()> {
    ///     let file = OpenOptions::new()
    ///         .write(true)
    ///         .truncate(true)
    ///         .open("foo.txt")
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn truncate(&mut self, truncate: bool) -> &mut OpenOptions {
        panic!("STUB: not implemented");
    }
    /// Sets the option for creating a new file.
    ///
    /// This option indicates whether a new file will be created if the file
    /// does not yet already exist.
    ///
    /// In order for the file to be created, [`write`] or [`append`] access must
    /// be used.
    ///
    /// This is an async version of [`std::fs::OpenOptions::create`][std]
    ///
    /// [std]: std::fs::OpenOptions::create
    /// [`write`]: OpenOptions::write
    /// [`append`]: OpenOptions::append
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::fs::OpenOptions;
    /// use std::io;
    ///
    /// #[tokio::main]
    /// async fn main() -> io::Result<()> {
    ///     let file = OpenOptions::new()
    ///         .write(true)
    ///         .create(true)
    ///         .open("foo.txt")
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn create(&mut self, create: bool) -> &mut OpenOptions {
        panic!("STUB: not implemented");
    }
    /// Sets the option to always create a new file.
    ///
    /// This option indicates whether a new file will be created.  No file is
    /// allowed to exist at the target location, also no (dangling) symlink.
    ///
    /// This option is useful because it is atomic. Otherwise between checking
    /// whether a file exists and creating a new one, the file may have been
    /// created by another process (a TOCTOU race condition / attack).
    ///
    /// If `.create_new(true)` is set, [`.create()`] and [`.truncate()`] are
    /// ignored.
    ///
    /// The file must be opened with write or append access in order to create a
    /// new file.
    ///
    /// This is an async version of [`std::fs::OpenOptions::create_new`][std]
    ///
    /// [std]: std::fs::OpenOptions::create_new
    /// [`.create()`]: OpenOptions::create
    /// [`.truncate()`]: OpenOptions::truncate
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::fs::OpenOptions;
    /// use std::io;
    ///
    /// #[tokio::main]
    /// async fn main() -> io::Result<()> {
    ///     let file = OpenOptions::new()
    ///         .write(true)
    ///         .create_new(true)
    ///         .open("foo.txt")
    ///         .await?;
    ///
    ///     Ok(())
    /// }
    /// ```
    pub fn create_new(&mut self, create_new: bool) -> &mut OpenOptions {
        panic!("STUB: not implemented");
    }
    /// Opens a file at `path` with the options specified by `self`.
    ///
    /// This is an async version of [`std::fs::OpenOptions::open`][std]
    ///
    /// [std]: std::fs::OpenOptions::open
    ///
    /// # Errors
    ///
    /// This function will return an error under a number of different
    /// circumstances. Some of these error conditions are listed here, together
    /// with their [`ErrorKind`]. The mapping to [`ErrorKind`]s is not part of
    /// the compatibility contract of the function, especially the `Other` kind
    /// might change to more specific kinds in the future.
    ///
    /// * [`NotFound`]: The specified file does not exist and neither `create`
    ///   or `create_new` is set.
    /// * [`NotFound`]: One of the directory components of the file path does
    ///   not exist.
    /// * [`PermissionDenied`]: The user lacks permission to get the specified
    ///   access rights for the file.
    /// * [`PermissionDenied`]: The user lacks permission to open one of the
    ///   directory components of the specified path.
    /// * [`AlreadyExists`]: `create_new` was specified and the file already
    ///   exists.
    /// * [`InvalidInput`]: Invalid combinations of open options (truncate
    ///   without write access, no access mode set, etc.).
    /// * [`Other`]: One of the directory components of the specified file path
    ///   was not, in fact, a directory.
    /// * [`Other`]: Filesystem-level errors: full disk, write permission
    ///   requested on a read-only file system, exceeded disk quota, too many
    ///   open files, too long filename, too many symbolic links in the
    ///   specified path (Unix-like systems only), etc.
    ///
    /// # io_uring support
    ///
    /// On Linux, you can also use `io_uring` for executing system calls.
    /// To enable `io_uring`, you need to specify the `--cfg tokio_unstable`
    /// flag at compile time, enable the `io-uring` cargo feature, and set the
    /// `Builder::enable_io_uring` runtime option.
    ///
    /// Support for `io_uring` is currently experimental, so its behavior may
    /// change or it may be removed in future versions.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use tokio::fs::OpenOptions;
    /// use std::io;
    ///
    /// #[tokio::main]
    /// async fn main() -> io::Result<()> {
    ///     let file = OpenOptions::new().open("foo.txt").await?;
    ///     Ok(())
    /// }
    /// ```
    ///
    /// [`ErrorKind`]: std::io::ErrorKind
    /// [`AlreadyExists`]: std::io::ErrorKind::AlreadyExists
    /// [`InvalidInput`]: std::io::ErrorKind::InvalidInput
    /// [`NotFound`]: std::io::ErrorKind::NotFound
    /// [`Other`]: std::io::ErrorKind::Other
    /// [`PermissionDenied`]: std::io::ErrorKind::PermissionDenied
    pub async fn open(&self, path: impl AsRef<Path>) -> io::Result<File> {
        panic!("STUB: not implemented");
    }
    async fn open_inner(&self, path: &Path) -> io::Result<File> {
        panic!("STUB: not implemented");
    }
    async fn std_open(opts: &StdOpenOptions, path: &Path) -> io::Result<File> {
        panic!("STUB: not implemented");
    }
    #[cfg(windows)]
    pub(super) fn as_inner_mut(&mut self) -> &mut StdOpenOptions {
        panic!("STUB: not implemented");
    }
}
feature! {
    #![unix] impl OpenOptions { #[doc =
    " Sets the mode bits that a new file will be created with."] #[doc = ""] #[doc =
    " If a new file is created as part of an `OpenOptions::open` call then this"] #[doc =
    " specified `mode` will be used as the permission bits for the new file."] #[doc =
    " If no `mode` is set, the default of `0o666` will be used."] #[doc =
    " The operating system masks out bits with the system's `umask`, to produce"] #[doc =
    " the final permissions."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc =
    " ```no_run"] #[doc = " use tokio::fs::OpenOptions;"] #[doc = " use std::io;"] #[doc
    = ""] #[doc = " #[tokio::main]"] #[doc = " async fn main() -> io::Result<()> {"]
    #[doc = "     let mut options = OpenOptions::new();"] #[doc =
    "     options.mode(0o644); // Give read/write for owner and read for others."] #[doc
    = "     let file = options.open(\"foo.txt\").await?;"] #[doc = ""] #[doc =
    "     Ok(())"] #[doc = " }"] #[doc = " ```"] pub fn mode(& mut self, mode : u32) -> &
    mut OpenOptions { match & mut self.inner { Kind::Std(opts) => { opts.mode(mode); }
    #[cfg(all(tokio_unstable, feature = "io-uring", feature = "rt", feature = "fs",
    target_os = "linux"))] Kind::Uring(opts) => { opts.mode(mode); } } self } #[doc =
    " Passes custom flags to the `flags` argument of `open`."] #[doc = ""] #[doc =
    " The bits that define the access mode are masked out with `O_ACCMODE`, to"] #[doc =
    " ensure they do not interfere with the access mode set by Rusts options."] #[doc =
    ""] #[doc =
    " Custom flags can only set flags, not remove flags set by Rusts options."] #[doc =
    " This options overwrites any previously set custom flags."] #[doc = ""] #[doc =
    " # Examples"] #[doc = ""] #[doc = " ```no_run"] #[doc =
    " use tokio::fs::OpenOptions;"] #[doc = " use std::io;"] #[doc = ""] #[doc =
    " #[tokio::main]"] #[doc = " async fn main() -> io::Result<()> {"] #[doc =
    "     let mut options = OpenOptions::new();"] #[doc = "     options.write(true);"]
    #[doc = "     if cfg!(unix) {"] #[doc =
    "         options.custom_flags(libc::O_NOFOLLOW);"] #[doc = "     }"] #[doc =
    "     let file = options.open(\"foo.txt\").await?;"] #[doc = ""] #[doc =
    "     Ok(())"] #[doc = " }"] #[doc = " ```"] pub fn custom_flags(& mut self, flags :
    i32) -> & mut OpenOptions { match & mut self.inner { Kind::Std(opts) => { opts
    .custom_flags(flags); } #[cfg(all(tokio_unstable, feature = "io-uring", feature =
    "rt", feature = "fs", target_os = "linux"))] Kind::Uring(opts) => { opts
    .custom_flags(flags); } } self } }
}
cfg_windows! {
    impl OpenOptions { #[doc =
    " Overrides the `dwDesiredAccess` argument to the call to [`CreateFile`]"] #[doc =
    " with the specified value."] #[doc = ""] #[doc =
    " This will override the `read`, `write`, and `append` flags on the"] #[doc =
    " `OpenOptions` structure. This method provides fine-grained control over"] #[doc =
    " the permissions to read, write and append data, attributes (like hidden"] #[doc =
    " and system), and extended attributes."] #[doc = ""] #[doc = " # Examples"] #[doc =
    ""] #[doc = " ```no_run"] #[doc = " use tokio::fs::OpenOptions;"] #[doc = ""] #[doc =
    " # #[tokio::main]"] #[doc = " # async fn main() -> std::io::Result<()> {"] #[doc =
    " // Open without read and write permission, for example if you only need"] #[doc =
    " // to call `stat` on the file"] #[doc =
    " let file = OpenOptions::new().access_mode(0).open(\"foo.txt\").await?;"] #[doc =
    " # Ok(())"] #[doc = " # }"] #[doc = " ```"] #[doc = ""] #[doc =
    " [`CreateFile`]: https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea"]
    pub fn access_mode(& mut self, access : u32) -> & mut OpenOptions { self
    .as_inner_mut().access_mode(access); self } #[doc =
    " Overrides the `dwShareMode` argument to the call to [`CreateFile`] with"] #[doc =
    " the specified value."] #[doc = ""] #[doc = " By default `share_mode` is set to"]
    #[doc = " `FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE`. This allows"]
    #[doc = " other processes to read, write, and delete/rename the same file"] #[doc =
    " while it is open. Removing any of the flags will prevent other"] #[doc =
    " processes from performing the corresponding operation until the file"] #[doc =
    " handle is closed."] #[doc = ""] #[doc = " # Examples"] #[doc = ""] #[doc =
    " ```no_run"] #[doc = " use tokio::fs::OpenOptions;"] #[doc = ""] #[doc =
    " # #[tokio::main]"] #[doc = " # async fn main() -> std::io::Result<()> {"] #[doc =
    " // Do not allow others to read or modify this file while we have it open"] #[doc =
    " // for writing."] #[doc = " let file = OpenOptions::new()"] #[doc =
    "     .write(true)"] #[doc = "     .share_mode(0)"] #[doc =
    "     .open(\"foo.txt\").await?;"] #[doc = " # Ok(())"] #[doc = " # }"] #[doc =
    " ```"] #[doc = ""] #[doc =
    " [`CreateFile`]: https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea"]
    pub fn share_mode(& mut self, share : u32) -> & mut OpenOptions { self.as_inner_mut()
    .share_mode(share); self } #[doc =
    " Sets extra flags for the `dwFileFlags` argument to the call to"] #[doc =
    " [`CreateFile2`] to the specified value (or combines it with"] #[doc =
    " `attributes` and `security_qos_flags` to set the `dwFlagsAndAttributes`"] #[doc =
    " for [`CreateFile`])."] #[doc = ""] #[doc =
    " Custom flags can only set flags, not remove flags set by Rust's options."] #[doc =
    " This option overwrites any previously set custom flags."] #[doc = ""] #[doc =
    " # Examples"] #[doc = ""] #[doc = " ```no_run"] #[doc =
    " use windows_sys::Win32::Storage::FileSystem::FILE_FLAG_DELETE_ON_CLOSE;"] #[doc =
    " use tokio::fs::OpenOptions;"] #[doc = ""] #[doc = " # #[tokio::main]"] #[doc =
    " # async fn main() -> std::io::Result<()> {"] #[doc =
    " let file = OpenOptions::new()"] #[doc = "     .create(true)"] #[doc =
    "     .write(true)"] #[doc = "     .custom_flags(FILE_FLAG_DELETE_ON_CLOSE)"] #[doc =
    "     .open(\"foo.txt\").await?;"] #[doc = " # Ok(())"] #[doc = " # }"] #[doc =
    " ```"] #[doc = ""] #[doc =
    " [`CreateFile`]: https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea"]
    #[doc =
    " [`CreateFile2`]: https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfile2"]
    pub fn custom_flags(& mut self, flags : u32) -> & mut OpenOptions { self
    .as_inner_mut().custom_flags(flags); self } #[doc =
    " Sets the `dwFileAttributes` argument to the call to [`CreateFile2`] to"] #[doc =
    " the specified value (or combines it with `custom_flags` and"] #[doc =
    " `security_qos_flags` to set the `dwFlagsAndAttributes` for"] #[doc =
    " [`CreateFile`])."] #[doc = ""] #[doc =
    " If a _new_ file is created because it does not yet exist and"] #[doc =
    " `.create(true)` or `.create_new(true)` are specified, the new file is"] #[doc =
    " given the attributes declared with `.attributes()`."] #[doc = ""] #[doc =
    " If an _existing_ file is opened with `.create(true).truncate(true)`, its"] #[doc =
    " existing attributes are preserved and combined with the ones declared"] #[doc =
    " with `.attributes()`."] #[doc = ""] #[doc =
    " In all other cases the attributes get ignored."] #[doc = ""] #[doc = " # Examples"]
    #[doc = ""] #[doc = " ```no_run"] #[doc =
    " use windows_sys::Win32::Storage::FileSystem::FILE_ATTRIBUTE_HIDDEN;"] #[doc =
    " use tokio::fs::OpenOptions;"] #[doc = ""] #[doc = " # #[tokio::main]"] #[doc =
    " # async fn main() -> std::io::Result<()> {"] #[doc =
    " let file = OpenOptions::new()"] #[doc = "     .write(true)"] #[doc =
    "     .create(true)"] #[doc = "     .attributes(FILE_ATTRIBUTE_HIDDEN)"] #[doc =
    "     .open(\"foo.txt\").await?;"] #[doc = " # Ok(())"] #[doc = " # }"] #[doc =
    " ```"] #[doc = ""] #[doc =
    " [`CreateFile`]: https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea"]
    #[doc =
    " [`CreateFile2`]: https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfile2"]
    pub fn attributes(& mut self, attributes : u32) -> & mut OpenOptions { self
    .as_inner_mut().attributes(attributes); self } #[doc =
    " Sets the `dwSecurityQosFlags` argument to the call to [`CreateFile2`] to"] #[doc =
    " the specified value (or combines it with `custom_flags` and `attributes`"] #[doc =
    " to set the `dwFlagsAndAttributes` for [`CreateFile`])."] #[doc = ""] #[doc =
    " By default `security_qos_flags` is not set. It should be specified when"] #[doc =
    " opening a named pipe, to control to which degree a server process can"] #[doc =
    " act on behalf of a client process (security impersonation level)."] #[doc = ""]
    #[doc = " When `security_qos_flags` is not set, a malicious program can gain the"]
    #[doc = " elevated privileges of a privileged Rust process when it allows opening"]
    #[doc = " user-specified paths, by tricking it into opening a named pipe. So"] #[doc
    = " arguably `security_qos_flags` should also be set when opening arbitrary"] #[doc =
    " paths. However the bits can then conflict with other flags, specifically"] #[doc =
    " `FILE_FLAG_OPEN_NO_RECALL`."] #[doc = ""] #[doc =
    " For information about possible values, see [Impersonation Levels] on the"] #[doc =
    " Windows Dev Center site. The `SECURITY_SQOS_PRESENT` flag is set"] #[doc =
    " automatically when using this method."] #[doc = ""] #[doc = " # Examples"] #[doc =
    ""] #[doc = " ```no_run"] #[doc =
    " use windows_sys::Win32::Storage::FileSystem::SECURITY_IDENTIFICATION;"] #[doc =
    " use tokio::fs::OpenOptions;"] #[doc = ""] #[doc = " # #[tokio::main]"] #[doc =
    " # async fn main() -> std::io::Result<()> {"] #[doc =
    " let file = OpenOptions::new()"] #[doc = "     .write(true)"] #[doc =
    "     .create(true)"] #[doc = ""] #[doc =
    "     // Sets the flag value to `SecurityIdentification`."] #[doc =
    "     .security_qos_flags(SECURITY_IDENTIFICATION)"] #[doc = ""] #[doc =
    "     .open(r\"\\\\.\\pipe\\MyPipe\").await?;"] #[doc = " # Ok(())"] #[doc = " # }"]
    #[doc = " ```"] #[doc = ""] #[doc =
    " [`CreateFile`]: https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea"]
    #[doc =
    " [`CreateFile2`]: https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfile2"]
    #[doc = " [Impersonation Levels]:"] #[doc =
    "     https://docs.microsoft.com/en-us/windows/win32/api/winnt/ne-winnt-security_impersonation_level"]
    pub fn security_qos_flags(& mut self, flags : u32) -> & mut OpenOptions { self
    .as_inner_mut().security_qos_flags(flags); self } }
}
impl From<StdOpenOptions> for OpenOptions {
    fn from(options: StdOpenOptions) -> OpenOptions {
        panic!("STUB: not implemented");
    }
}
impl Default for OpenOptions {
    fn default() -> Self {
        panic!("STUB: not implemented");
    }
}
