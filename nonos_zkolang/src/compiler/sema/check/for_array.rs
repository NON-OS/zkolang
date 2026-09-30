/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `for x in arr` and `for (i, x) in arr.enumerate()`: `x` has the element type, and
 * `i` is a `usize` (section 8.6).
 */

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::{TyKind, Types};
use crate::compiler::syntax::ast::{Block, Expr, Pattern};
use crate::compiler::tir::{Labels, TExprKind};

impl<'s, 'a> FnCx<'s, 'a> {
    /** The loop over the elements of `a`, and their indices when `enumerate`. */
    pub(super) fn for_array(
        &mut self,
        pat: &'a Pattern,
        a: &'a Expr,
        enumerate: bool,
        body: &'a Block,
    ) -> TExprKind {
        let array = self.infer(a, None);
        let el = match self.kind(array.ty) {
            TyKind::Array(el, _) => el,
            TyKind::Error => Types::ERROR,
            _ => {
                let shown = self.show(array.ty);
                let d = Diagnostic::error(
                    Code::MISMATCHED_TYPES,
                    alloc::format!("`for` goes over a range or an array, not `{shown}`"),
                    array.span,
                    "not a range or an array",
                );
                self.sema.diags.push(d);
                Types::ERROR
            }
        };
        let (index, pat) = match enumerate {
            true => self.enumerate_pat(pat, el),
            false => (
                None,
                self.bind_pat(pat, el, &Labels::default(), &mut Vec::new()),
            ),
        };
        let body = self.loop_body(body);
        let (array, pat) = (Box::new(array), Box::new(pat));
        TExprKind::ForArray {
            index,
            pat,
            array,
            body,
        }
    }
}
