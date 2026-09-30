/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A typed value rebuilt from the leaf values that come next: a scalar from its one leaf,
 * an enum from the leaves of its slots, which must lay out a value of it.
 */

use alloc::vec::Vec;

use super::slots_read::from_slots;
use crate::compiler::interp::Value;
use crate::compiler::lower::layout::slots;
use crate::compiler::sema::ty::{TyId, TyKind, Types};

/** The value of type `t` whose leaves come next in `at`; `Err` the position of an enum's that lay out none. */
pub(super) fn value_of(types: &Types, t: TyId, at: &mut (&[i128], usize)) -> Result<Value, usize> {
    if let Some(ts) = types.record(t) {
        return ts
            .iter()
            .map(|&e| value_of(types, e, at))
            .collect::<Result<_, _>>()
            .map(Value::Tuple);
    }
    Ok(match types.kind(t) {
        TyKind::Bool => Value::Bool(next(at) == 1),
        TyKind::Field => Value::Field(u64::try_from(next(at)).unwrap_or(0)),
        TyKind::Int(_) => Value::Int(next(at)),
        TyKind::Array(e, n) => Value::Array(
            (0..*n)
                .map(|_| value_of(types, *e, at))
                .collect::<Result<_, _>>()?,
        ),
        TyKind::Adt(_) => {
            let (start, n) = (at.1, slots(types, t));
            let leaves: Vec<i128> = (0..n).map(|_| next(at)).collect();
            from_slots(types, t, &mut leaves.into_iter()).ok_or(start)?
        }
        _ => Value::Unit,
    })
}

/** The leaf value next in `at`, which moves past it. */
fn next(at: &mut (&[i128], usize)) -> i128 {
    at.1 += 1;
    at.0.get(at.1 - 1).copied().unwrap_or(0)
}
