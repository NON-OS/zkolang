/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Where the parts a tuple, array, struct or variant pattern takes lie among a value's slots. */

use alloc::vec::Vec;

use super::cx::Lower;
use super::layout::{elements, field_at, variant_field_at};
use crate::compiler::sema::ty::{TyId, TyKind};
use crate::compiler::tir::TPat;

impl<'p> Lower<'p> {
    /** The first slot and type of each of the `n` parts a tuple or variant pattern takes. */
    pub(super) fn parts(&self, pat: &TPat, t: TyId, n: usize) -> Vec<(usize, TyId)> {
        let types = &self.p.types;
        let k = (0..n).map(|k| k as u32);
        match (pat, types.kind(t)) {
            (TPat::Variant(tag, _), _) => k
                .filter_map(|k| variant_field_at(types, t, *tag, k))
                .collect(),
            (_, TyKind::Array(..)) => match elements(types, t) {
                Some((e, size, _)) => (0..n).map(|k| (k * size, e)).collect(),
                None => Vec::new(),
            },
            _ => k.filter_map(|k| field_at(types, t, k)).collect(),
        }
    }
}
