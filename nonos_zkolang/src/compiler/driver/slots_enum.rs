/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Reading an enum value from its slots: its tag names a variant, the variant's fields
 * follow, and every payload slot past them is 0.
 */

use alloc::vec::Vec;

use super::slots_read::{from_slots, Slots};
use crate::compiler::interp::Value;
use crate::compiler::lower::layout::slots;
use crate::compiler::sema::ty::{TyId, Types};

/** The enum value of type `t` in the slots next in `it`: a tag, its fields, then zeros. */
pub(super) fn enum_from_slots(types: &Types, t: TyId, it: Slots<'_>) -> Option<Value> {
    let tag = u32::try_from(it.next()?).ok()?;
    let v = types.adt(t)?.variants.get(tag as usize)?;
    let fields = v.fields.iter().map(|f| from_slots(types, f.ty, it));
    let fields: Vec<Value> = fields.collect::<Option<_>>()?;
    let used: usize = v.fields.iter().map(|f| slots(types, f.ty)).sum();
    let rest = slots(types, t).saturating_sub(1 + used);
    (0..rest)
        .all(|_| it.next() == Some(0))
        .then_some(Value::Variant(tag, fields))
}
