use std::num::NonZeroU64;
#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
pub(crate) struct ThreadId(NonZeroU64);
impl ThreadId {
    pub(crate) fn next() -> Self {
        panic!("STUB: not implemented");
    }
}
#[cold]
#[allow(dead_code)]
fn exhausted() -> ! {
    panic!("STUB: not implemented");
}
