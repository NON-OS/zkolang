/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The expression dispatcher: route each node to its own lowering. */

use super::super::compiler::{Compiler, Val, MAX_EXPR_DEPTH};
use crate::isa::Op;
use crate::lang::parse::Expr;
use crate::lang::CompileError;

impl Compiler {
    /** Compile an expression, returning the register that holds its value. */
    pub(crate) fn expr(&mut self, e: &Expr) -> Result<Val, CompileError> {
        if self.expr_depth >= MAX_EXPR_DEPTH {
            return Err(CompileError::RecursionTooDeep);
        }
        self.expr_depth += 1;
        let v = self.expr_node(e);
        self.expr_depth -= 1;
        v
    }

    fn expr_node(&mut self, e: &Expr) -> Result<Val, CompileError> {
        match e {
            Expr::Num(v) => self.emit_num(*v),
            Expr::Var(n) => self.emit_var(n),
            Expr::Add(l, r) => self.binary(l, r, |d, a, b| Op::Add { d, a, b }),
            Expr::Sub(l, r) => self.binary(l, r, |d, a, b| Op::Sub { d, a, b }),
            Expr::Mul(l, r) => self.binary(l, r, |d, a, b| Op::Mul { d, a, b }),
            Expr::Eq(l, r) => self.binary(l, r, |d, a, b| Op::Eq { d, a, b }),
            Expr::Div(l, r) => self.div(l, r),
            Expr::Neg(x) => self.neg(x),
            Expr::Ne(l, r) => self.ne(l, r),
            Expr::Lt(l, r) => self.compare(l, r),
            Expr::Inv(x) => self.inv(x),
            Expr::Sel(c, l, r) => self.select(c, l, r),
            Expr::If(c, l, r) => self.select(c, l, r),
            Expr::Call(name, args) => self.call(name, args),
            Expr::Index(base, idx, at) => self.index(base, idx, *at),
            Expr::Array(_) => Err(CompileError::ArrayNotScalar),
            Expr::Block(locals, result) => self.block_expr(locals, result),
            Expr::Tuple(_) => Err(CompileError::TupleNotScalar),
        }
    }
}
