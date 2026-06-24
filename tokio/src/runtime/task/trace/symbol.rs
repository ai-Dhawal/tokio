use backtrace::BacktraceSymbol;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ptr;
/// A symbol in a backtrace.
///
/// This wrapper type serves two purposes. The first is that it provides a
/// representation of a symbol that can be inserted into hashmaps and hashsets;
/// the [`backtrace`] crate does not define [`Hash`], [`PartialEq`], or [`Eq`]
/// on [`BacktraceSymbol`], and recommends that users define their own wrapper
/// which implements these traits.
///
/// Second, this wrapper includes a `parent_hash` field that uniquely
/// identifies this symbol's position in its trace. Otherwise, e.g., our code
/// would not be able to distinguish between recursive calls of a function at
/// different depths.
#[derive(Clone)]
pub(super) struct Symbol {
    pub(super) symbol: BacktraceSymbol,
    pub(super) parent_hash: u64,
}
impl Hash for Symbol {
    fn hash<H>(&self, state: &mut H)
    where
        H: Hasher,
    {
        panic!("STUB: not implemented");
    }
}
impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        panic!("STUB: not implemented");
    }
}
impl Eq for Symbol {}
impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
