/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Every constructor of a type, and the types of the parts each builds. */

use alloc::vec::Vec;

use super::pat::Ctor;
use crate::compiler::sema::ty::{TyId, TyKind, Types};
use crate::compiler::syntax::IntTy;
use nonos_stark::field::P;

/** The constructors of a type. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Domain {
    /** A tuple, array, struct or `()`: one constructor. */
    Single,
    Bools,
    /** An enum of this many variants; `!` has none. */
    Variants(u32),
    /** The integers from the first to the second, both included. */
    Ints(i128, i128),
    /** A type that failed to check: the check does not look into it. */
    Opaque,
}

/** The constructors of type `ty`. */
pub(super) fn domain(types: &Types, ty: TyId) -> Domain {
    match types.kind(ty) {
        TyKind::Bool => Domain::Bools,
        TyKind::Int(i) => Domain::Ints(IntTy::min(*i), IntTy::max(*i)),
        TyKind::Field => Domain::Ints(0, i128::from(P) - 1),
        TyKind::Never => Domain::Variants(0),
        TyKind::Unit | TyKind::Tuple(_) | TyKind::Array(..) => Domain::Single,
        TyKind::Adt(_) => match types.adt(ty) {
            Some(a) if a.is_enum => {
                Domain::Variants(u32::try_from(a.variants.len()).unwrap_or(u32::MAX))
            }
            Some(_) => Domain::Single,
            None => Domain::Opaque,
        },
        TyKind::Error | TyKind::Var(_) => Domain::Opaque,
    }
}

/** The types of the parts that `c` builds for a value of type `ty`. */
pub(super) fn parts(types: &Types, ty: TyId, c: Ctor) -> Vec<TyId> {
    match c {
        Ctor::Single => types.parts(ty, None),
        Ctor::Variant(t) => types.parts(ty, Some(t)),
        Ctor::Bool(_) | Ctor::Range(..) => Vec::new(),
    }
}
