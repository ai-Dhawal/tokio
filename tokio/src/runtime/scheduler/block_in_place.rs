use crate::runtime::scheduler;
#[track_caller]
pub(crate) fn block_in_place<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    panic!("STUB: not implemented");
}
