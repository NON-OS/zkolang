/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Reading a value from the slots it is laid out in (section 6). Each slot is checked
 * against its type, and an enum's payload slots past its variant's fields must be 0, so
 * slots that read back to a value are that value's only layout.
 */

use alloc::vec::Vec;

use nonos_stark::field::P;

use super::slots_enum::enum_from_slots;
use crate::compiler::interp::Value;
use crate::compiler::sema::ty::{TyId, TyKind, Types};
use crate::compiler::syntax::IntTy;

/** The slots of a value, read in order. */
pub(super) type Slots<'i> = &'i mut dyn Iterator<Item = i128>;

/** The value of type `t` laid out in the slots next in `it`, if they hold one. */
pub(super) fn from_slots(types: &Types, t: TyId, it: Slots<'_>) -> Option<Value> {
    if let Some(ts) = types.record(t) {
        let parts = ts.iter().map(|&e| from_slots(types, e, it));
        return parts.collect::<Option<Vec<Value>>>().map(Value::Tuple);
    }
    match types.kind(t) {
        TyKind::Unit => Some(Value::Unit),
        TyKind::Bool => match it.next()? {
            0 => Some(Value::Bool(false)),
            1 => Some(Value::Bool(true)),
            _ => None,
        },
        TyKind::Field => {
            let f = u64::try_from(it.next()?).ok()?;
            (i128::from(f) < i128::from(P)).then_some(Value::Field(f))
        }
        TyKind::Int(i) => int_from_slots(*i, it),
        TyKind::Array(e, n) => {
            let parts = (0..*n).map(|_| from_slots(types, *e, it));
            parts.collect::<Option<Vec<Value>>>().map(Value::Array)
        }
        TyKind::Adt(_) => enum_from_slots(types, t, it),
        _ => None,
    }
}

/** The integer of type `i` in the slots next in `it`: one, or a 64-bit one's two halves. */
fn int_from_slots(i: IntTy, it: Slots<'_>) -> Option<Value> {
    let p = i128::from(P);
    let v = if i.bits() > 32 {
        let half = 0..1i128 << 32;
        let (lo, hi) = (it.next()?, it.next()?);
        let pattern = (half.contains(&lo) && half.contains(&hi)).then_some(lo | hi << 32)?;
        match i.signed() && pattern >= 1i128 << 63 {
            true => pattern - (1i128 << 64),
            false => pattern,
        }
    } else {
        let x = it.next().filter(|x| (0..p).contains(x))?;
        if i.signed() && x > p / 2 {
            x - p
        } else {
            x
        }
    };
    i.contains(v).then_some(Value::Int(v))
}
