/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Built-in methods (section 18.3). The receiver's type must be known: an integer
 * literal's type is settled first by a suffix or an annotation, or by the expected type
 * when the method gives a value of the receiver's type.
 */

use super::super::cx::FnCx;
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
        let mark = self.sema.diags.mark();
        let recv = self.infer(receiver, want.filter(|_| same));
        let rt = self.resolve(recv.ty);
        if self.sema.types.adt(rt).is_some() {
            let recv = (receiver, recv, rt, mark);
            return self.struct_method(recv, (method, generic), args, at);
        }
        let (b, params, ret) = match self.method_sig(receiver, method, rt, args, at) {
            Ok(sig) => sig,
            Err(done) => return done,
        };
        if generic {
            self.no_generic_args(method);
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
