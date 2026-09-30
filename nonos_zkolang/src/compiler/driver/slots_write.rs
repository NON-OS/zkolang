/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Laying a value out in slots (section 6), as the compiled program holds it. */

use alloc::vec::Vec;

use nonos_stark::field::P;

use crate::compiler::interp::Value;
use crate::compiler::lower::layout::slots;
use crate::compiler::sema::ty::{TyId, TyKind, Types};

/** Push the slots the value `v` of type `t` is laid out in. */
pub(super) fn to_slots(types: &Types, t: TyId, v: &Value, out: &mut Vec<i128>) {
    match (types.kind(t), v) {
        (TyKind::Int(i), Value::Int(x)) if i.bits() > 32 => {
            let pattern = x.rem_euclid(1i128 << 64);
            out.push(pattern & 0xFFFF_FFFF);
            out.push(pattern >> 32);
        }
        (TyKind::Adt(_), Value::Variant(tag, vs)) => {
            let start = out.len();
            out.push(i128::from(*tag));
            for (f, x) in types.parts(t, Some(*tag)).into_iter().zip(vs) {
                to_slots(types, f, x, out);
            }
            out.resize(start + slots(types, t), 0);
        }
        (_, Value::Tuple(vs) | Value::Array(vs)) => {
            let parts = types.parts(t, None);
            parts
                .into_iter()
                .zip(vs)
                .for_each(|(e, x)| to_slots(types, e, x, out));
        }
        (_, Value::Unit) => {}
        (_, x) => out.push(x.int().rem_euclid(i128::from(P))),
    }
}
