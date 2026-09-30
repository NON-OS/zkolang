/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A module's namespace (section 4.4): one name per item, whether declared there or
 * imported. A name imported by two globs from different items is ambiguous, which is an
 * error only where it is used.
 */

use alloc::collections::BTreeMap;
use alloc::string::String;

use super::DefId;
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::Visibility;

/** How a name came into a module. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BindingKind {
    /** Declared by an item of the module. */
    Item,
    /** Imported by a `use` that names it. */
    Import,
    /** Imported by a glob; an item or a named import of the same name takes precedence. */
    Glob,
    /** Imported by globs from two different items: an error where it is used. */
    Ambiguous,
}

/** A name in a module, and the item it stands for. */
#[derive(Clone, Copy, Debug)]
pub struct Binding {
    pub def: DefId,
    pub kind: BindingKind,
    /** Where the name is declared or imported. */
    pub span: Span,
    /** Whether other modules may use the name: `pub` items and `pub use` imports. */
    pub vis: Visibility,
}

/** The names of one module. */
#[derive(Clone, Debug, Default)]
pub struct Module {
    pub names: BTreeMap<String, Binding>,
}
