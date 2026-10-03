/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A value taken apart into its leaf values, for comparing a reference run's result with
 * the compiled run's outputs. An enum's leaves are its slots.
 */

use alloc::vec::Vec;

use super::slots_write::to_slots;
use crate::compiler::interp::Value;
use crate::compiler::sema::ty::{TyId, TyKind, Types};

/** Push the leaf values of `v`, a value of type `t`, in order. */
pub(super) fn leaves_of(types: &Types, t: TyId, v: &Value, out: &mut Vec<i128>) {
    match (types.kind(t), v) {
        (TyKind::Adt(_), Value::Variant(..)) => to_slots(types, t, v, out),
        (_, Value::Tuple(vs) | Value::Array(vs)) => {
            let parts = types.parts(t, None);
            parts
                .into_iter()
                .zip(vs)
                .for_each(|(e, x)| leaves_of(types, e, x, out));
        }
        (_, Value::Unit) => {}
        (_, x) => out.push(x.int()),
    }
}
