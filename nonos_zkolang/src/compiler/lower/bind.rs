/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Binding a pattern's locals to the parts of a value's slots. Where a pattern has
 * alternatives, each alternative's locals are written only where it is the first that
 * matches, so the value bound is the one the reference semantics binds.
 */

use super::cx::Lower;
use super::error::L;
use super::layout::slots;
use crate::compiler::sema::ty::TyId;
use crate::compiler::ssa::V;
use crate::compiler::tir::TPat;

impl<'p> Lower<'p> {
    /** Bind the locals of `pat` to the parts of `vals`, a value of type `t`. */
    pub(super) fn bind(&mut self, pat: &TPat, vals: &[V], t: TyId) -> L<()> {
        self.bind_when(pat, vals, t, None)
    }

    /** Bind the locals of `pat` as `bind` does, but only where `when` is 1, if given. */
    fn bind_when(&mut self, pat: &TPat, vals: &[V], t: TyId, when: Option<V>) -> L<()> {
        match pat {
            TPat::Bind(l) => {
                let new = match when {
                    Some(c) => {
                        let old = self.local(*l);
                        self.pick(c, vals, &old)
                    }
                    None => vals.to_vec(),
                };
                self.set_local(*l, new);
            }
            TPat::Tuple(ps) | TPat::Variant(_, ps) => {
                for (p, (at, ty)) in ps.iter().zip(self.parts(pat, t, ps.len())) {
                    let n = slots(&self.p.types, ty);
                    let part = vals.get(at..at + n).unwrap_or(&[]).to_vec();
                    self.bind_when(p, &part, ty, when)?;
                }
            }
            TPat::Or(alts) => {
                for a in alts.iter().rev() {
                    let c = self.pat_cond(a, vals, t)?;
                    let c = match when {
                        Some(w) => self.and(w, c),
                        None => c,
                    };
                    self.bind_when(a, vals, t, Some(c))?;
                }
            }
            TPat::Wild | TPat::Lit(..) | TPat::Range(..) => {}
        }
        Ok(())
    }
}
