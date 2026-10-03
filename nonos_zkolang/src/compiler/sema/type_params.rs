/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The generic parameters an item takes, and a generic argument that fits none (E0701). */

use super::cx::Sema;
use super::defs::DefId;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{GenericParam, ItemKind};

impl<'a> Sema<'a> {
    /** The generic parameters the item `def` takes. */
    pub(crate) fn params_of(&self, def: DefId) -> &'a [GenericParam] {
        match self.defs.get(def).and_then(|d| d.item).map(|i| &i.kind) {
            Some(ItemKind::Struct(s)) => &s.generics,
            Some(ItemKind::Enum(e)) => &e.generics,
            Some(ItemKind::TypeAlias(a)) => &a.generics,
            Some(ItemKind::Fn(f)) => &f.generics,
            _ => &[],
        }
    }

    /** Report the generic argument at `at`, of which `what` is said (E0701). */
    pub(super) fn wrong_arg<T>(&mut self, at: Span, what: &str) -> Option<T> {
        let d = Diagnostic::error(Code::WRONG_GENERICS, what, at, "not this argument");
        self.diags.push(d);
        None
    }
}
