/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The patterns of a `match` once every type is known: each literal fits its type (E0302),
 * a `field` literal taken to its canonical value, and each range is of an integer type
 * and not empty (E0404).
 */

use alloc::format;

use super::super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::{TyId, TyKind, Types};
use crate::compiler::tir::{TLit, TPat};

impl<'s, 'a> FnCx<'s, 'a> {
    /** Rewrite the pattern `p`, which takes a value of type `ty`. */
    pub(super) fn rewrite_pat(&mut self, p: &mut TPat, ty: TyId) {
        match p {
            TPat::Lit(TLit::Int(v), at) => *v = self.check_fits(*v, ty, *at),
            TPat::Range(lo, hi, at) => {
                let at = *at;
                if !matches!(self.kind(ty), TyKind::Int(_) | TyKind::Error) {
                    let what = format!("a range pattern for a value of type `{}`", self.show(ty));
                    let d = Diagnostic::error(Code::PATTERN_MISMATCH, what, at, "not an integer")
                        .with_help(
                            "a range pattern takes an integer; test other values one by one",
                        );
                    self.sema.diags.push(d);
                    return;
                }
                *lo = self.check_fits(*lo, ty, at);
                *hi = self.check_fits(*hi, ty, at);
                if *lo > *hi {
                    let what = format!("the range `{lo}..={hi}` is empty");
                    let d = Diagnostic::error(Code::EMPTY_RANGE, what, at, "matches nothing");
                    self.sema.diags.push(d);
                }
            }
            TPat::Tuple(ps) => self.rewrite_parts(ps, ty, None),
            TPat::Variant(t, ps) => self.rewrite_parts(ps, ty, Some(*t)),
            TPat::Or(alts) => alts.iter_mut().for_each(|a| self.rewrite_pat(a, ty)),
            TPat::Bind(_) | TPat::Wild | TPat::Lit(..) => {}
        }
    }

    /** Rewrite the patterns `ps` of the parts of a value of type `ty`, or of its variant `tag`. */
    fn rewrite_parts(&mut self, ps: &mut [TPat], ty: TyId, tag: Option<u32>) {
        let tys = self.sema.types.parts(ty, tag);
        for (i, q) in ps.iter_mut().enumerate() {
            self.rewrite_pat(q, tys.get(i).copied().unwrap_or(Types::ERROR));
        }
    }
}
