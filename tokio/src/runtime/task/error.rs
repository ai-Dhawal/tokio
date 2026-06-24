use std::any::Any;
use std::fmt;
use std::io;
use super::Id;
use crate::util::SyncWrapper;
cfg_rt! {
    #[doc = " Task failed to execute to completion."] pub struct JoinError { repr : Repr,
    id : Id, }
}
enum Repr {
    Cancelled,
    Panic(SyncWrapper<Box<dyn Any + Send + 'static>>),
}
impl JoinError {
    pub(crate) fn cancelled(id: Id) -> JoinError {
        panic!("STUB: not implemented");
    }
    pub(crate) fn panic(id: Id, err: Box<dyn Any + Send + 'static>) -> JoinError {
        panic!("STUB: not implemented");
    }
    /// Returns true if the error was caused by the task being cancelled.
    ///
    /// See [the module level docs] for more information on cancellation.
    ///
    /// [the module level docs]: crate::task#cancellation
    pub fn is_cancelled(&self) -> bool {
        panic!("STUB: not implemented");
    }
    /// Returns true if the error was caused by the task panicking.
    ///
    /// # Examples
    ///
    /// ```
    /// # #[cfg(not(target_family = "wasm"))]
    /// # {
    /// use std::panic;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let err = tokio::spawn(async {
    ///         panic!("boom");
    ///     }).await.unwrap_err();
    ///
    ///     assert!(err.is_panic());
    /// }
    /// # }
    /// ```
    pub fn is_panic(&self) -> bool {
        panic!("STUB: not implemented");
    }
    /// Consumes the join error, returning the object with which the task panicked.
    ///
    /// # Panics
    ///
    /// `into_panic()` panics if the `Error` does not represent the underlying
    /// task terminating with a panic. Use `is_panic` to check the error reason
    /// or `try_into_panic` for a variant that does not panic.
    ///
    /// # Examples
    ///
    /// ```should_panic,ignore-wasm
    /// use std::panic;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let err = tokio::spawn(async {
    ///         panic!("boom");
    ///     }).await.unwrap_err();
    ///
    ///     if err.is_panic() {
    ///         // Resume the panic on the main task
    ///         panic::resume_unwind(err.into_panic());
    ///     }
    /// }
    /// ```
    #[track_caller]
    pub fn into_panic(self) -> Box<dyn Any + Send + 'static> {
        panic!("STUB: not implemented");
    }
    /// Consumes the join error, returning the object with which the task
    /// panicked if the task terminated due to a panic. Otherwise, `self` is
    /// returned.
    ///
    /// # Examples
    ///
    /// ```should_panic,ignore-wasm
    /// use std::panic;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let err = tokio::spawn(async {
    ///         panic!("boom");
    ///     }).await.unwrap_err();
    ///
    ///     if let Ok(reason) = err.try_into_panic() {
    ///         // Resume the panic on the main task
    ///         panic::resume_unwind(reason);
    ///     }
    /// }
    /// ```
    pub fn try_into_panic(self) -> Result<Box<dyn Any + Send + 'static>, JoinError> {
        panic!("STUB: not implemented");
    }
    /// Returns a [task ID] that identifies the task which errored relative to
    /// other currently spawned tasks.
    ///
    /// [task ID]: crate::task::Id
    pub fn id(&self) -> Id {
        panic!("STUB: not implemented");
    }
}
impl fmt::Display for JoinError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl fmt::Debug for JoinError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl std::error::Error for JoinError {}
impl From<JoinError> for io::Error {
    fn from(src: JoinError) -> io::Error {
        panic!("STUB: not implemented");
    }
}
fn panic_payload_as_str(payload: &SyncWrapper<Box<dyn Any + Send>>) -> Option<&str> {
    panic!("STUB: not implemented");
}
