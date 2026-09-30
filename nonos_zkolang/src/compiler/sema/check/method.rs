/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Built-in methods (section 18.3). The receiver's type must be known: an integer
 * literal's type is settled first by a suffix or an annotation, or by the expected type
 * when the method gives a value of the receiver's type.
 */

use alloc::format;

use super::cx::FnCx;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::sema::ty::TyId;
use crate::compiler::source::Span;
use crate::compiler::syntax::ast::{Expr, Ident};
use crate::compiler::tir::{TExpr, TExprKind};

/** The methods whose value has the receiver's type. */
const SAME_TYPE: [&str; 8] = [
    "wrapping_add",
    "wrapping_sub",
    "wrapping_mul",
    "wrapping_neg",
    "min",
    "max",
    "pow",
    "inv",
];

impl<'s, 'a> FnCx<'s, 'a> {
    /** `receiver.method(args)`. */
    pub(crate) fn method_call(
        &mut self,
        receiver: &'a Expr,
        method: &Ident,
        generic: bool,
        args: &'a [Expr],
        want: Option<TyId>,
        at: Span,
    ) -> TExpr {
        let same = SAME_TYPE.contains(&method.name.as_str());
        let recv = self.infer(receiver, want.filter(|_| same));
        let rt = self.resolve(recv.ty);
        let (b, params, ret) = match self.method_sig(receiver, method, rt, args, at) {
            Ok(sig) => sig,
            Err(done) => return done,
        };
        if generic {
            let d = Diagnostic::error(
                Code::WRONG_GENERICS,
                format!("`{}` takes no generic arguments", method.name),
                method.span,
                "generic arguments given",
            );
            self.sema.diags.push(d);
        }
        self.arity(method, params.len(), args, at);
        let mut operands = alloc::vec![recv];
        for (i, a) in args.iter().enumerate() {
            operands.push(self.expr(a, params.get(i).copied()));
        }
        TExpr {
            kind: TExprKind::Builtin(b, operands),
            ty: ret,
            span: at,
        }
    }
}
