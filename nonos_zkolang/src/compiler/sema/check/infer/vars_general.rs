/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! General type variables: made where a type is to be inferred, and said where they arose. */

use alloc::string::String;

use super::vars::{IntVars, State};
use crate::compiler::sema::ty::{TyId, TyKind, Types};
use crate::compiler::source::Span;

impl IntVars {
    /** A new general variable for `what`, which arose at `at`, and the type that names it. */
    pub(crate) fn fresh_general(&mut self, types: &mut Types, at: Span, what: String) -> TyId {
        let n = self.len();
        self.states.push(State::Free {
            usize_default: false,
            general: true,
        });
        self.origins.insert(n, (at, what));
        types.intern(TyKind::Infer(n))
    }

    /** Where the variable `n`, or the one it is joined to, arose and what it stands for. */
    pub(crate) fn origin(&mut self, n: u32) -> Option<(Span, String)> {
        let r = self.root(n);
        self.origins
            .get(&r)
            .or_else(|| self.origins.get(&n))
            .cloned()
    }
}
