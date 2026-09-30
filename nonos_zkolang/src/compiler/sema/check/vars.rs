/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The types of unsuffixed integer literals while they are not yet known (section 5.6).
 * Each is a variable that meeting a known integer or `field` type binds, and two that meet
 * become one. At the end of a body each left free takes its default: `usize` if it arose
 * where a length, index, bound or count stands, and `field` otherwise.
 */

use alloc::vec::Vec;

use crate::compiler::sema::ty::{TyId, TyKind, Types};

/** What one variable is. */
#[derive(Clone, Copy, Debug)]
pub(super) enum State {
    Free {
        usize_default: bool,
    },
    /** Joined with the variable of this number. */
    Link(u32),
    Bound(TyId),
}

/** The integer literal variables of one body. */
#[derive(Clone, Debug, Default)]
pub struct IntVars {
    pub(super) states: Vec<State>,
}

impl IntVars {
    /** A new variable, and the type that names it. */
    pub(crate) fn fresh(&mut self, types: &mut Types, usize_default: bool) -> TyId {
        let n = u32::try_from(self.states.len()).unwrap_or(u32::MAX);
        self.states.push(State::Free { usize_default });
        types.intern(TyKind::Var(n))
    }

    /**
     * The variable `n` stands for after following its links; every variable on the way is
     * linked to it directly, so a later search is short.
     */
    pub(crate) fn root(&mut self, n: u32) -> u32 {
        let mut r = n;
        for _ in 0..=self.states.len() {
            match self.states.get(r as usize) {
                Some(State::Link(m)) => r = *m,
                _ => break,
            }
        }
        let mut cur = n;
        while cur != r {
            let Some(s) = self.states.get_mut(cur as usize) else {
                break;
            };
            let State::Link(next) = *s else {
                break;
            };
            *s = State::Link(r);
            cur = next;
        }
        r
    }

    /** How many variables there are. */
    pub(crate) fn len(&self) -> u32 {
        u32::try_from(self.states.len()).unwrap_or(u32::MAX)
    }
}
