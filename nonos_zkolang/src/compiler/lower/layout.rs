/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Layout (section 6): every value is a fixed list of field-element slots. A `field`, a
 * `bool` and an integer of at most 32 bits take one; a 64-bit integer two, its low and
 * high 32-bit halves; a tuple or struct its fields' in order; an array its elements' in
 * order; an enum a tag and then the most any variant's fields take.
 */

use crate::compiler::sema::ty::{TyId, TyKind, Types};

/** How many slots a value of type `t` takes. */
pub(crate) fn slots(types: &Types, t: TyId) -> usize {
    match types.kind(t) {
        TyKind::Unit | TyKind::Never | TyKind::Error | TyKind::Var(_) | TyKind::Infer(_) => 0,
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
        TyKind::Adt(_) => {
            let Some(a) = types.adt(t) else {
                return 0;
            };
            let payload = a
                .variants
                .iter()
                .map(|v| v.fields.iter().map(|f| slots(types, f.ty)).sum());
            let most: usize = payload.max().unwrap_or(0);
            most + usize::from(a.is_enum)
        }
    }
}

/** The first slot of field `i` of a tuple or struct of type `t`, and the field's type. */
pub(crate) fn field_at(types: &Types, t: TyId, i: u32) -> Option<(usize, TyId)> {
    let ts = types.record(t)?;
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

/** The first slot of field `i` of variant `tag` of the enum `t`, and the field's type. */
pub(crate) fn variant_field_at(types: &Types, t: TyId, tag: u32, i: u32) -> Option<(usize, TyId)> {
    let fields = &types.adt(t)?.variants.get(tag as usize)?.fields;
    let i = i as usize;
    let at: usize = fields.get(..i)?.iter().map(|f| slots(types, f.ty)).sum();
    Some((1 + at, fields.get(i)?.ty))
}
