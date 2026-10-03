/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Resolving, joining and binding type variables. */

use super::vars::{IntVars, State};
use crate::compiler::sema::ty::{TyId, TyKind, Types};

impl IntVars {
    /** The type `t` stands for: a bound variable's type, or the root of a free one. */
    pub(crate) fn resolve(&mut self, types: &mut Types, t: TyId) -> TyId {
        let (TyKind::Var(n) | TyKind::Infer(n)) = *types.kind(t) else {
            return t;
        };
        let r = self.root(n);
        match self.states.get(r as usize) {
            Some(State::Bound(b)) => *b,
            Some(State::Free { general: true, .. }) => types.intern(TyKind::Infer(r)),
            _ => types.intern(TyKind::Var(r)),
        }
    }

    /**
     * Join free variables `a` and `b`. The joined one is general only if both were, and
     * defaults to `usize` if either did.
     */
    pub(crate) fn join(&mut self, a: u32, b: u32) {
        let (a, b) = (self.root(a), self.root(b));
        if a == b {
            return;
        }
        let usize_default = self.usize_default(a) || self.usize_default(b);
        let general = self.general(a) && self.general(b);
        if let Some(s) = self.states.get_mut(a as usize) {
            *s = State::Link(b);
        }
        if let Some(s) = self.states.get_mut(b as usize) {
            *s = State::Free {
                usize_default,
                general,
            };
        }
        if let Some(o) = self.origins.get(&a).cloned() {
            self.origins.entry(b).or_insert(o);
        }
    }

    /** Bind the free variable `n` to `t`. */
    pub(crate) fn bind(&mut self, n: u32, t: TyId) {
        let r = self.root(n);
        if let Some(s) = self.states.get_mut(r as usize) {
            *s = State::Bound(t);
        }
    }

    /** Whether the free variable `n` may become any type. */
    pub(crate) fn general(&mut self, n: u32) -> bool {
        let r = self.root(n);
        matches!(
            self.states.get(r as usize),
            Some(State::Free { general: true, .. })
        )
    }
}
