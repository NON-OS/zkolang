/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The scalar leaves of a type, and the slots they take. */

use alloc::vec::Vec;

use crate::compiler::sema::ty::{TyId, TyKind, Types};
use crate::compiler::syntax::IntTy;

/** A scalar leaf of a value's type. */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Leaf {
    Bool,
    Field,
    Int(IntTy),
}

impl Leaf {
    /** How many slots a value of this leaf takes: two for a 64-bit integer, else one. */
    pub fn width(self) -> usize {
        match self {
            Leaf::Int(t) if t.bits() > 32 => 2,
            _ => 1,
        }
    }
}

/** Why typed values do not fit the ABI. */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AbiError {
    /** `expected` values are needed and `got` were given. */
    Count { expected: usize, got: usize },
    /** The value at `position` is not a value of its leaf's type. */
    Range { position: usize },
}

/** The leaves of type `t`, in order. */
pub fn leaves(types: &Types, t: TyId, out: &mut Vec<Leaf>) {
    match types.kind(t) {
        TyKind::Bool => out.push(Leaf::Bool),
        TyKind::Field => out.push(Leaf::Field),
        TyKind::Int(i) => out.push(Leaf::Int(*i)),
        TyKind::Tuple(ts) => ts.iter().for_each(|&e| leaves(types, e, out)),
        TyKind::Array(e, n) => (0..*n).for_each(|_| leaves(types, *e, out)),
        _ => {}
    }
}

/** How many slots values of `leaves` take. */
pub fn slots(leaves: &[Leaf]) -> usize {
    leaves.iter().map(|l| l.width()).sum()
}
