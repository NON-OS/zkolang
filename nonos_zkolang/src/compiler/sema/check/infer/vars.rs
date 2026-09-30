/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The type variables of one body. The type of an unsuffixed integer literal is one while
 * it is not yet known (section 5.6): meeting a known integer or `field` type binds it, and
 * two that meet become one. At the end of a body each left free takes its default: `usize`
 * if it arose where a length, index, bound or count stands, and `field` otherwise. A
 * general variable, such as a generic argument to infer, may become any type, and one
 * left free at the end of a body is reported (E0303).
 */

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

use crate::compiler::sema::ty::{TyId, TyKind, Types};
use crate::compiler::source::Span;

/** What one variable is. */
#[derive(Clone, Copy, Debug)]
pub(super) enum State {
    Free {
        usize_default: bool,
        general: bool,
    },
    /** Joined with the variable of this number. */
    Link(u32),
    Bound(TyId),
}

/** The integer literal variables of one body. */
#[derive(Clone, Debug, Default)]
pub struct IntVars {
    pub(super) states: Vec<State>,
    /** Where each general variable arose, and what it stands for. */
    pub(super) origins: BTreeMap<u32, (Span, String)>,
}

impl IntVars {
    /** A new variable, and the type that names it. */
    pub(crate) fn fresh(&mut self, types: &mut Types, usize_default: bool) -> TyId {
        let n = u32::try_from(self.states.len()).unwrap_or(u32::MAX);
        self.states.push(State::Free {
            usize_default,
            general: false,
        });
        types.intern(TyKind::Var(n))
    }

    /** How many variables there are. */
    pub(crate) fn len(&self) -> u32 {
        u32::try_from(self.states.len()).unwrap_or(u32::MAX)
    }
}
