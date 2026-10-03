/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Patterns as the check sees them: a constructor and its parts, `_`, or alternatives. */

use alloc::vec::Vec;

use crate::compiler::sema::ty::{TyId, Types};
use crate::compiler::tir::{TLit, TPat};

/** What a value is built with at the top, as far as patterns tell values apart. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Ctor {
    /** The one way to build a tuple, array, struct or `()`. */
    Single,
    Variant(u32),
    Bool(bool),
    /** The integer or `field` values from the first to the second, both included. */
    Range(i128, i128),
}

impl Ctor {
    /** Whether every value `c` builds, `self` builds too. */
    pub(super) fn covers(self, c: Ctor) -> bool {
        match (self, c) {
            (Ctor::Range(x, y), Ctor::Range(a, b)) => x <= a && b <= y,
            _ => self == c,
        }
    }
}

/** A pattern as the check sees it. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub(super) enum Pat {
    Wild,
    Ctor(Ctor, Vec<Pat>),
    Or(Vec<Pat>),
}

impl Pat {
    /** The pattern `p`, which takes a value of type `ty`. */
    pub(super) fn of(types: &Types, p: &TPat, ty: TyId) -> Pat {
        let each = |ps: &[TPat], tys: Vec<TyId>| -> Vec<Pat> {
            let at = |i: usize| tys.get(i).copied().unwrap_or(Types::ERROR);
            ps.iter()
                .enumerate()
                .map(|(i, q)| Pat::of(types, q, at(i)))
                .collect()
        };
        match p {
            TPat::Bind(_) | TPat::Wild => Pat::Wild,
            TPat::Lit(TLit::Bool(b), _) => Pat::Ctor(Ctor::Bool(*b), Vec::new()),
            TPat::Lit(TLit::Int(v), _) => Pat::Ctor(Ctor::Range(*v, *v), Vec::new()),
            TPat::Lit(TLit::Unit, _) => Pat::Ctor(Ctor::Single, Vec::new()),
            TPat::Range(lo, hi, _) => Pat::Ctor(Ctor::Range(*lo, *hi), Vec::new()),
            TPat::Tuple(ps) => Pat::Ctor(Ctor::Single, each(ps, types.parts(ty, None))),
            TPat::Variant(t, ps) => {
                Pat::Ctor(Ctor::Variant(*t), each(ps, types.parts(ty, Some(*t))))
            }
            TPat::Or(alts) => Pat::Or(alts.iter().map(|a| Pat::of(types, a, ty)).collect()),
        }
    }
}
