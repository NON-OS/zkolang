/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Patterns (section 9): names, `_`, and tuples, arrays, structs and variants of patterns;
 * in a `match` arm also literals, ranges and alternatives. The labels a type annotation
 * writes go with the parts they qualify.
 */

use alloc::vec::Vec;

use super::cx::FnCx;
use super::labels::sub_labels;
use crate::compiler::sema::ty::{TyId, TyKind, Types};
use crate::compiler::syntax::ast::{PatKind, Pattern};
use crate::compiler::tir::{Labels, TPat};

impl<'s, 'a> FnCx<'s, 'a> {
    /** Bind the names of the whole pattern `p`, which takes a value of type `ty`. */
    pub(crate) fn bind_pat(
        &mut self,
        p: &'a Pattern,
        ty: TyId,
        l: &Labels,
        path: &mut Vec<u32>,
    ) -> TPat {
        self.pats.bound.clear();
        let out = self.pat(p, ty, l, path);
        self.pats.bound.clear();
        out
    }

    /** Bind the names of `p`, which takes a value of type `ty` at `path` in the whole. */
    pub(crate) fn pat(
        &mut self,
        p: &'a Pattern,
        ty: TyId,
        labels: &Labels,
        path: &mut Vec<u32>,
    ) -> TPat {
        let parts: Vec<(&'a Pattern, TyId, Option<u32>)> = match (&p.kind, self.kind(ty)) {
            (PatKind::Bind { name, mutable }, _) => {
                return self.lone_name_pat(p, (name, *mutable), ty, sub_labels(labels, path));
            }
            (PatKind::Wild | PatKind::Error, _) => return TPat::Wild,
            (PatKind::Tuple(ps), TyKind::Unit) if ps.is_empty() => Vec::new(),
            (PatKind::Tuple(ps), TyKind::Tuple(ts)) if ps.len() == ts.len() => {
                ps.iter().zip(ts).map(|(p, t)| (p, t, None)).collect()
            }
            (PatKind::Array(ps), TyKind::Array(el, n)) if u32::try_from(ps.len()) == Ok(n) => {
                ps.iter().map(|p| (p, el, Some(Labels::ELEMENT))).collect()
            }
            (PatKind::Struct { .. } | PatKind::TupleStruct(..), _) | (PatKind::Path(_), _) => {
                return self.bind_struct(p, ty, labels, path);
            }
            (PatKind::Tuple(ps) | PatKind::Array(ps), k) => {
                if k != TyKind::Error {
                    self.pattern_mismatch(p, ps.len(), ty);
                }
                ps.iter().map(|p| (p, Types::ERROR, None)).collect()
            }
            _ => return self.refutable_pat(p, ty, labels, path),
        };
        let mut out = Vec::with_capacity(parts.len());
        for (i, (p, t, step)) in parts.into_iter().enumerate() {
            path.push(step.unwrap_or(u32::try_from(i).unwrap_or(u32::MAX)));
            out.push(self.pat(p, t, labels, path));
            path.pop();
        }
        TPat::Tuple(out)
    }
}
