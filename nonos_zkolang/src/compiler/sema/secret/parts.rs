/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The parts whose labels a value keeps apart (section 13.1). A tuple's or struct's are its
 * fields. An enum's are its tag and then every field of every variant in order, so which
 * variant a value is and what its fields hold are labelled apart; the fields of the
 * variants a value is not hold zeros, which are public.
 */

use alloc::vec::Vec;

use crate::compiler::sema::ty::{TyId, Types};
use crate::compiler::syntax::IntTy;

/** The types of the parts of a value of type `ty`, if it has any. */
pub(super) fn parts(types: &Types, ty: TyId) -> Option<Vec<TyId>> {
    if let Some(ts) = types.record(ty) {
        return Some(ts);
    }
    let adt = types.adt(ty).filter(|a| a.is_enum)?;
    let fields = adt
        .variants
        .iter()
        .flat_map(|v| v.fields.iter().map(|f| f.ty));
    Some(
        core::iter::once(Types::int(IntTy::U32))
            .chain(fields)
            .collect(),
    )
}

/** The index among the parts of the enum `ty` of field `i` of its variant `tag`. */
pub(super) fn variant_part(types: &Types, ty: TyId, tag: u32, i: u32) -> usize {
    let before = types.adt(ty).map_or(0, |a| {
        let earlier = a.variants.iter().take(tag as usize);
        earlier.map(|v| v.fields.len()).sum::<usize>()
    });
    1 + before + i as usize
}
