/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Resolving, joining and binding integer literal variables. */

use super::vars::{IntVars, State};
use crate::compiler::sema::ty::{TyId, TyKind, Types};

impl IntVars {
    /** The type `t` stands for: a bound variable's type, or the root of a free one. */
    pub(crate) fn resolve(&mut self, types: &mut Types, t: TyId) -> TyId {
        let TyKind::Var(n) = *types.kind(t) else {
            return t;
        };
        let r = self.root(n);
        match self.states.get(r as usize) {
            Some(State::Bound(b)) => *b,
            _ => types.intern(TyKind::Var(r)),
        }
    }

    /** Join free variables `a` and `b`; the joined one defaults to `usize` if either did. */
    pub(crate) fn join(&mut self, a: u32, b: u32) {
        let (a, b) = (self.root(a), self.root(b));
        if a == b {
            return;
        }
        let d = self.usize_default(a) || self.usize_default(b);
        if let Some(s) = self.states.get_mut(a as usize) {
            *s = State::Link(b);
        }
        if let Some(s) = self.states.get_mut(b as usize) {
            *s = State::Free { usize_default: d };
        }
    }

    /** Bind the free variable `n` to `t`. */
    pub(crate) fn bind(&mut self, n: u32, t: TyId) {
        let r = self.root(n);
        if let Some(s) = self.states.get_mut(r as usize) {
            *s = State::Bound(t);
        }
    }

    /** Whether the free variable `n` defaults to `usize`. */
    pub(crate) fn usize_default(&mut self, n: u32) -> bool {
        let r = self.root(n);
        matches!(
            self.states.get(r as usize),
            Some(State::Free {
                usize_default: true
            })
        )
    }
}
