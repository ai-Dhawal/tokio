use super::{EnterRuntime, CONTEXT};
/// Returns true if in a runtime context.
pub(crate) fn current_enter_context() -> EnterRuntime {
    panic!("STUB: not implemented");
}
/// Forces the current "entered" state to be cleared while the closure
/// is executed.
pub(crate) fn exit_runtime<F: FnOnce() -> R, R>(f: F) -> R {
    panic!("STUB: not implemented");
}
