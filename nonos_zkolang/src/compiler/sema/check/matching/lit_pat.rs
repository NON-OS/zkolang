/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The patterns that may not match (section 9.1), which only a `match` arm takes: a `bool`
 * or integer literal, an inclusive range `lo..=hi` of integer literals, and alternatives.
 * Whether a literal fits its type is checked once the type is known.
 */

use alloc::vec::Vec;

use super::super::cx::FnCx;
use crate::compiler::sema::ty::{TyId, Types};
use crate::compiler::syntax::ast::{Lit, PatKind, Pattern};
use crate::compiler::tir::{Labels, TLit, TPat};

impl<'s, 'a> FnCx<'s, 'a> {
    /** The pattern `p`, which may not match, for a value of type `ty`. */
    pub(crate) fn refutable_pat(
        &mut self,
        p: &'a Pattern,
        ty: TyId,
        labels: &Labels,
        path: &mut Vec<u32>,
    ) -> TPat {
        if !self.pats.arm {
            self.refutable(p);
            return TPat::Wild;
        }
        match &p.kind {
            PatKind::Lit {
                lit: Lit::Bool { value, .. },
                ..
            } => match self.unify(ty, Types::BOOL) {
                true => TPat::Lit(TLit::Bool(*value), p.span),
                false => {
                    self.mismatch(p.span, ty, Types::BOOL);
                    TPat::Wild
                }
            },
            PatKind::Lit { .. } => match self.int_end(p, ty) {
                Some(v) => TPat::Lit(TLit::Int(v), p.span),
                None => TPat::Wild,
            },
            PatKind::Range { lo, hi } => match (self.int_end(lo, ty), self.int_end(hi, ty)) {
                (Some(lo), Some(hi)) => TPat::Range(lo, hi, p.span),
                _ => TPat::Wild,
            },
            PatKind::Or(alts) => self.or_pat(alts, ty, labels, path),
            _ => TPat::Wild,
        }
    }
}
