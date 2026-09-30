/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The pattern of `for (i, x) in a.enumerate()`: an index and an element. */

use alloc::vec::Vec;

use super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::{TyId, Types};
use crate::compiler::syntax::ast::{PatKind, Pattern};
use crate::compiler::syntax::IntTy;
use crate::compiler::tir::{Labels, LocalId, TPat};

impl<'s, 'a> FnCx<'s, 'a> {
    /** `(i, pat)` of `for (i, pat) in a.enumerate()`: the index and the element's pattern. */
    pub(super) fn enumerate_pat(&mut self, pat: &'a Pattern, el: TyId) -> (Option<LocalId>, TPat) {
        let pair = match &pat.kind {
            PatKind::Tuple(ps) if ps.len() == 2 => ps.first().zip(ps.get(1)),
            _ => None,
        };
        let Some((i, e)) = pair else {
            let d = Diagnostic::error(
                Code::PATTERN_MISMATCH,
                "`enumerate` gives an index and an element",
                pat.span,
                "expected `(index, element)`",
            );
            self.sema.diags.push(d);
            return (
                None,
                self.bind_pat(pat, Types::ERROR, &Labels::default(), &mut Vec::new()),
            );
        };
        let usize_ty = Types::int(IntTy::Usize);
        let index = match self.bind_pat(i, usize_ty, &Labels::default(), &mut Vec::new()) {
            TPat::Bind(l) => Some(l),
            _ => None,
        };
        (
            index,
            self.bind_pat(e, el, &Labels::default(), &mut Vec::new()),
        )
    }
}
