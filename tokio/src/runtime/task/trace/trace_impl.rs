//! Current `backtrace::trace` + collector based backtrace implementation
//!
//! This implementation may eventually be extracted into a separate `tokio-taskdump` crate.
use std::ptr;
use crate::runtime::task::trace::{Trace, TraceMeta};
/// Capture a backtrace via `backtrace::trace` and collect it into `trace`.
pub(crate) fn trace_leaf(meta: &TraceMeta, trace: &mut Trace) {
    panic!("STUB: not implemented");
}
