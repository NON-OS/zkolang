/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Whether a value matches a pattern (section 9.1), as a 0 or 1: a variant by its tag and
 * then its fields, a literal by its slots, a range by two comparisons, a tuple, struct or
 * array by all its parts, alternatives by any.
 */

use super::cx::Lower;
use super::error::L;
use super::layout::slots;
use crate::compiler::sema::ty::TyId;
use crate::compiler::ssa::{Inst, V};
use crate::compiler::tir::{TLit, TPat};

impl<'p> Lower<'p> {
    /** 1 if `vals`, a value of type `t`, matches `pat`, else 0. */
    pub(super) fn pat_cond(&mut self, pat: &TPat, vals: &[V], t: TyId) -> L<V> {
        let one = self.one();
        Ok(match pat {
            TPat::Bind(_) | TPat::Wild | TPat::Lit(TLit::Unit, _) => one,
            TPat::Tuple(ps) | TPat::Variant(_, ps) => {
                let mut all = match pat {
                    TPat::Variant(tag, _) => {
                        let k = self.b.konst(i128::from(*tag));
                        let tag = vals.first().copied().unwrap_or(k);
                        self.b.emit(Inst::Eq(tag, k))
                    }
                    _ => one,
                };
                for (p, (at, ty)) in ps.iter().zip(self.parts(pat, t, ps.len())) {
                    let n = slots(&self.p.types, ty);
                    let part = vals.get(at..at + n).unwrap_or(&[]).to_vec();
                    let c = self.pat_cond(p, &part, ty)?;
                    all = self.and(all, c);
                }
                all
            }
            TPat::Lit(TLit::Bool(b), _) => {
                let v = vals.first().copied().unwrap_or(one);
                if *b {
                    v
                } else {
                    self.b.not(v)
                }
            }
            TPat::Lit(TLit::Int(i), _) => {
                let want = self.int_slots(*i, t);
                let mut all = one;
                for (&a, &b) in vals.iter().zip(&want) {
                    let e = self.b.emit(Inst::Eq(a, b));
                    all = self.and(all, e);
                }
                all
            }
            TPat::Range(lo, hi, _) => self.in_range(vals, (*lo, *hi), t),
            TPat::Or(alts) => {
                let mut none = one;
                for a in alts {
                    let c = self.pat_cond(a, vals, t)?;
                    let not_c = self.b.not(c);
                    none = self.and(none, not_c);
                }
                self.b.not(none)
            }
        })
    }
}
