/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Layout (section 6): every value is a fixed list of field-element slots. A `field`, a
 * `bool` and an integer of at most 32 bits take one; a 64-bit integer two, its low and
 * high 32-bit halves; a tuple its fields' in order; an array its elements' in order.
 */

use crate::compiler::sema::ty::{TyId, TyKind, Types};

/** How many slots a value of type `t` takes. */
pub(crate) fn slots(types: &Types, t: TyId) -> usize {
    match types.kind(t) {
        TyKind::Unit | TyKind::Never | TyKind::Error | TyKind::Var(_) => 0,
        TyKind::Bool | TyKind::Field => 1,
        TyKind::Int(i) => {
            if i.bits() > 32 {
                2
            } else {
                1
            }
        }
        TyKind::Tuple(ts) => ts.iter().map(|&e| slots(types, e)).sum(),
        TyKind::Array(e, n) => slots(types, *e).saturating_mul(*n as usize),
    }
}

/** The first slot of field `i` of a tuple of type `t`, and the field's type. */
pub(crate) fn field_at(types: &Types, t: TyId, i: u32) -> Option<(usize, TyId)> {
    let TyKind::Tuple(ts) = types.kind(t) else {
        return None;
    };
    let i = i as usize;
    let at = ts.get(..i)?.iter().map(|&e| slots(types, e)).sum();
    Some((at, *ts.get(i)?))
}

/** An array type's element type, element size in slots, and length. */
pub(crate) fn elements(types: &Types, t: TyId) -> Option<(TyId, usize, usize)> {
    match types.kind(t) {
        TyKind::Array(e, n) => Some((*e, slots(types, *e), *n as usize)),
        _ => None,
    }
}
