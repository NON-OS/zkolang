/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The invariant of an enum input (section 6): its tag names one of its variants, the
 * fields of that variant hold their invariants, and every other payload slot is 0. Each
 * variant's fields are checked on their slots times whether the tag names it, so the
 * checks of the variants the tag does not name see only zeros, which every type allows.
 */

use alloc::vec;
use alloc::vec::Vec;

use super::cx::Lower;
use super::layout::slots;
use crate::compiler::sema::ty::TyId;
use crate::compiler::ssa::{Inst, V};

impl<'p> Lower<'p> {
    /** Check that `vals`, the slots of a value of the enum `t`, hold its invariant. */
    pub(super) fn check_enum(&mut self, t: TyId, vals: &[V]) {
        let types = &self.p.types;
        let Some(adt) = types.adt(t) else {
            return;
        };
        let variants: Vec<Vec<TyId>> = adt
            .variants
            .iter()
            .map(|v| v.fields.iter().map(|f| f.ty).collect())
            .collect();
        let zero = self.b.konst(0);
        let tag = vals.first().copied().unwrap_or(zero);
        let mut named = vec![zero; vals.len()];
        let mut any = zero;
        for (k, fields) in variants.iter().enumerate() {
            let k = self.b.konst(k as i128);
            let is = self.b.emit(Inst::Eq(tag, k));
            any = self.b.add(any, is);
            let mut at = 1;
            for &f in fields {
                let n = slots(&self.p.types, f);
                let part: Vec<V> = vals.get(at..at + n).unwrap_or(&[]).to_vec();
                let masked: Vec<V> = part.iter().map(|&x| self.b.mul(is, x)).collect();
                self.check_input(f, &masked);
                for s in named.iter_mut().skip(at).take(n) {
                    *s = self.b.add(*s, is);
                }
                at += n;
            }
        }
        self.require(any);
        for (&x, &s) in vals.iter().zip(&named).skip(1) {
            let off = self.b.not(s);
            let stray = self.b.mul(off, x);
            self.require_zero(stray);
        }
    }
}
