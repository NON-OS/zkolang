/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Locals and the scopes that name them: a `let` or parameter declares a local in the
 * innermost scope, where it shadows an earlier one of its name until the scope closes.
 */

use alloc::string::String;
use alloc::vec::Vec;

use super::cx::FnCx;
use crate::compiler::sema::ty::TyId;
use crate::compiler::source::Span;
use crate::compiler::tir::{Labels, LocalId, TLocal};

impl<'s, 'a> FnCx<'s, 'a> {
    /** Declare a local named `name` in the innermost scope. */
    pub(crate) fn declare(
        &mut self,
        name: &str,
        ty: TyId,
        mutable: bool,
        labels: Labels,
        span: Span,
    ) -> LocalId {
        let id = LocalId(u32::try_from(self.locals.len()).unwrap_or(u32::MAX));
        self.locals.push(TLocal {
            name: String::from(name),
            ty,
            mutable,
            labels,
            span,
        });
        if name != "_" {
            self.names.entry(String::from(name)).or_default().push(id);
            if let Some(s) = self.scopes.last_mut() {
                s.push(String::from(name));
            }
        }
        id
    }

    /** The local `name` names here, if any. */
    pub(crate) fn lookup(&self, name: &str) -> Option<LocalId> {
        self.names.get(name).and_then(|v| v.last().copied())
    }

    /** Open a block's scope. */
    pub(crate) fn push_scope(&mut self) {
        self.scopes.push(Vec::new());
    }

    /** Close the innermost scope: its names name what they named before it. */
    pub(crate) fn pop_scope(&mut self) {
        for name in self.scopes.pop().unwrap_or_default() {
            if let Some(v) = self.names.get_mut(&name) {
                v.pop();
            }
        }
    }
}
