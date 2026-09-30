/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The patterns `let` and parameters bind (section 9.2): names, `_`, and tuples and arrays
 * of them. The labels a type annotation writes go with the parts they qualify.
 */

use alloc::vec::Vec;

use super::cx::FnCx;
use super::labels::sub_labels;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::{TyId, TyKind, Types};
use crate::compiler::syntax::ast::{PatKind, Pattern};
use crate::compiler::tir::{Labels, TPat};

impl<'s, 'a> FnCx<'s, 'a> {
    /** Bind the names of `p`, which takes a value of type `ty` at `path` in the whole. */
    pub(crate) fn bind_pat(
        &mut self,
        p: &'a Pattern,
        ty: TyId,
        labels: &Labels,
        path: &mut Vec<u32>,
    ) -> TPat {
        let parts: Vec<(&'a Pattern, TyId, Option<u32>)> = match (&p.kind, self.kind(ty)) {
            (PatKind::Bind { name, mutable }, _) => {
                let labels = sub_labels(labels, path);
                return TPat::Bind(self.declare(&name.name, ty, *mutable, labels, name.span));
            }
            (PatKind::Wild | PatKind::Error, _) => return TPat::Wild,
            (PatKind::Tuple(ps), TyKind::Tuple(ts)) if ps.len() == ts.len() => {
                ps.iter().zip(ts).map(|(p, t)| (p, t, None)).collect()
            }
            (PatKind::Array(ps), TyKind::Array(el, n)) if u32::try_from(ps.len()) == Ok(n) => {
                ps.iter().map(|p| (p, el, Some(Labels::ELEMENT))).collect()
            }
            (PatKind::Tuple(ps) | PatKind::Array(ps), k) => {
                if k != TyKind::Error {
                    self.pattern_mismatch(p, ps.len(), ty);
                }
                ps.iter().map(|p| (p, Types::ERROR, None)).collect()
            }
            _ => {
                let d = Diagnostic::error(Code::REFUTABLE_PATTERN, "this pattern may not match", p.span, "a pattern that does not always match")
                    .with_help("`let` and parameters take names, `_`, and tuples or arrays of them; test values with `if`");
                self.sema.diags.push(d);
                return TPat::Wild;
            }
        };
        let mut out = Vec::with_capacity(parts.len());
        for (i, (p, t, step)) in parts.into_iter().enumerate() {
            path.push(step.unwrap_or(u32::try_from(i).unwrap_or(u32::MAX)));
            out.push(self.bind_pat(p, t, labels, path));
            path.pop();
        }
        TPat::Tuple(out)
    }

    /** Report a pattern `p` of `n` parts for a value of type `ty` (E0402). */
    fn pattern_mismatch(&mut self, p: &Pattern, n: usize, ty: TyId) {
        let shown = self.show(ty);
        let what = alloc::format!("a pattern of {n} parts for a value of type `{shown}`");
        let d = Diagnostic::error(
            Code::PATTERN_MISMATCH,
            what,
            p.span,
            "does not fit the value",
        );
        self.sema.diags.push(d);
    }
}
