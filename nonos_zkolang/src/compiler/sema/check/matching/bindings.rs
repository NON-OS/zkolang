/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The names a pattern binds (section 9.1): each at most once, and in alternatives
 * `p | q` the same names at the same types and mutability, one local each (E0403).
 */

use alloc::format;
use alloc::string::String;

use super::super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::TyId;
use crate::compiler::syntax::ast::Ident;
use crate::compiler::tir::{Labels, LocalId};

impl<'s, 'a> FnCx<'s, 'a> {
    /** The local the pattern binds to `name`, a value of type `ty`. */
    pub(crate) fn pat_bind(&mut self, name: &Ident, ty: TyId, mutable: bool, l: Labels) -> LocalId {
        if let Some(local) = self.rebind(name, ty, mutable) {
            return local;
        }
        if self.pats.bound.iter().any(|(n, _)| *n == name.name) {
            self.bindings_error(name, "is bound twice in one pattern");
        }
        let local = self.declare(&name.name, ty, mutable, l, name.span);
        self.pats.bound.push((name.name.clone(), local));
        local
    }

    /** In an alternative after the first, the first one's local named `name`, if it has one. */
    fn rebind(&mut self, name: &Ident, ty: TyId, mutable: bool) -> Option<LocalId> {
        let alt = self.pats.reuse.last_mut()?;
        let Some(entry) = alt.iter_mut().find(|e| e.0 == name.name) else {
            self.bindings_error(name, "is not bound in the first alternative");
            return None;
        };
        let (local, twice) = (entry.1, core::mem::replace(&mut entry.2, true));
        if twice {
            self.bindings_error(name, "is bound twice in one pattern");
        }
        let (was, was_mut) = self
            .locals
            .get(local.0 as usize)
            .map(|x| (x.ty, x.mutable))?;
        if !self.unify(was, ty) || was_mut != mutable {
            self.bindings_error(
                name,
                "is bound here otherwise than in the first alternative",
            );
        }
        self.pats.bound.push((name.name.clone(), local));
        Some(local)
    }

    /** Report the binding `name` (E0403), saying `what` of it. */
    pub(super) fn bindings_error(&mut self, name: &Ident, what: &str) {
        let message = format!("`{}` {what}", name.name);
        let d = Diagnostic::error(
            Code::PATTERN_BINDINGS,
            message,
            name.span,
            String::from(what),
        );
        let help = "alternatives `p | q` bind the same names, at the same types and mutability";
        self.sema.diags.push(d.with_help(help));
    }
}
