use std::collections::{hash_map::DefaultHasher, HashMap, HashSet};
use std::fmt;
use std::hash::{Hash, Hasher};
use super::{Backtrace, Symbol, SymbolTrace, Trace};
/// An adjacency list representation of an execution tree.
///
/// This tree provides a convenient intermediate representation for formatting
/// [`Trace`] as a tree.
pub(super) struct Tree {
    /// The roots of the trees.
    ///
    /// There should only be one root, but the code is robust to multiple roots.
    roots: HashSet<Symbol>,
    /// The adjacency list of symbols in the execution tree(s).
    edges: HashMap<Symbol, HashSet<Symbol>>,
}
impl Tree {
    /// Constructs a [`Tree`] from [`Trace`]
    pub(super) fn from_trace(trace: Trace) -> Self {
        panic!("STUB: not implemented");
    }
    /// Produces the sub-symbols of a given symbol.
    fn consequences(
        &self,
        frame: &Symbol,
    ) -> Option<impl ExactSizeIterator<Item = &Symbol>> {
        panic!("STUB: not implemented");
    }
    /// Format this [`Tree`] as a textual tree.
    fn display<W: fmt::Write>(
        &self,
        f: &mut W,
        root: &Symbol,
        is_last: bool,
        prefix: &str,
    ) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        panic!("STUB: not implemented");
    }
}
/// Resolve a sequence of [`backtrace::BacktraceFrame`]s into a sequence of
/// [`Symbol`]s.
fn to_symboltrace(backtrace: Backtrace) -> SymbolTrace {
    panic!("STUB: not implemented");
}
