/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! An assignment to what is not a place. */

use alloc::vec::Vec;

use super::super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::Types;
use crate::compiler::syntax::ast::Expr;
use crate::compiler::tir::{LocalId, TPlace};

impl<'s, 'a> FnCx<'s, 'a> {
    /** Report that `e` is not a place (E0305), and check it for its own errors. */
    pub(super) fn not_a_place(&mut self, e: &'a Expr) -> TPlace {
        let help =
            "a place is a `let mut` variable, a `&mut` parameter, or a field or element of one";
        let d = Diagnostic::error(
            Code::NOT_A_PLACE,
            "this is not a place",
            e.span,
            "cannot be assigned",
        )
        .with_help(help);
        self.sema.diags.push(d);
        self.infer(e, None);
        let (root, proj, ty, span) = (LocalId(0), Vec::new(), Types::ERROR, e.span);
        TPlace {
            root,
            proj,
            ty,
            span,
        }
    }
}
