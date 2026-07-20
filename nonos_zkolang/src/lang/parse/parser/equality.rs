/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The comparison level. Equality and the ordered comparisons sit here, above the
//! sums. The ordered operators are expressed through `Lt`: `a > b` is `b < a`, and the
//! inclusive forms are the negations, `a <= b` is `1 - (b < a)`, exact because a
//! comparison yields a bit.

use super::Parser;
use crate::lang::lex::Tok;
use crate::lang::parse::ast::Expr;
use crate::lang::CompileError;
use alloc::boxed::Box;

impl<'a> Parser<'a> {
    /// An optional single comparison of two sums. It is not chainable, because a
    /// comparison yields a bit and comparing that to a third sum is rarely meant.
    pub(crate) fn equality(&mut self) -> Result<Expr, CompileError> {
        let lhs = self.sum()?;
        let op = match self.peek() {
            Some(t @ (Tok::EqEq | Tok::BangEq | Tok::Lt | Tok::Le | Tok::Gt | Tok::Ge)) => {
                t.clone()
            }
            _ => return Ok(lhs),
        };
        self.pos += 1;
        let rhs = self.sum()?;
        let (l, r) = (Box::new(lhs), Box::new(rhs));
        Ok(match op {
            Tok::EqEq => Expr::Eq(l, r),
            Tok::BangEq => Expr::Ne(l, r),
            Tok::Lt => Expr::Lt(l, r),
            Tok::Gt => Expr::Lt(r, l),
            Tok::Le => not(Expr::Lt(r, l)),
            _ => not(Expr::Lt(l, r)),
        })
    }
}

/// The logical complement of a bit, `1 - x`.
fn not(e: Expr) -> Expr {
    Expr::Sub(Box::new(Expr::Num(1)), Box::new(e))
}
