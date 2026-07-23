/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Expression lowering: the dispatcher routes each node to its own lowering file.

mod binary;
mod block;
mod call;
mod compare;
mod decompose;
mod div;
mod index;
mod inline;
mod inv;
mod ne;
mod neg;
mod num;
mod select;
mod var;
use super::compiler::{Compiler, Val};
use crate::isa::Op;
use crate::lang::parse::Expr;
use crate::lang::CompileError;

impl Compiler {
    /// Compile an expression, returning the register that holds its value.
    pub(crate) fn expr(&mut self, e: &Expr) -> Result<Val, CompileError> {
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
        }
    }
}
