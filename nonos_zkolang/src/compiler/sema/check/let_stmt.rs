/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! `let` (section 8.1): an annotation checks the initializer, and labels the pattern. */

use alloc::vec::Vec;

use super::cx::FnCx;
use crate::compiler::syntax::ast::{Expr, Pattern, Type};
use crate::compiler::tir::{Labels, TStmt};

impl<'s, 'a> FnCx<'s, 'a> {
    /** `let pat: ty = init;`: the initializer is checked before the pattern binds. */
    pub(crate) fn let_stmt(
        &mut self,
        pat: &'a Pattern,
        ty: Option<&'a Type>,
        init: &'a Expr,
    ) -> TStmt {
        let (want, labels) = match ty {
            Some(t) => {
                let (t, l) = self.sema.lower_ty(self.module, t);
                (Some(t), l)
            }
            None => (None, Labels::default()),
        };
        let init = self.expr(init, want);
        let ty = want.unwrap_or(init.ty);
        let pat = self.bind_pat(pat, ty, &labels, &mut Vec::new());
        TStmt::Let { pat, init }
    }
}
