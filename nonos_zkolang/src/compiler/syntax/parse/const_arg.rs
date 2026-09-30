/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Constant arguments, as in an array length or a generic argument list. */

use alloc::boxed::Box;

use super::parser::{PResult, Parser};
use super::paths::PathMode;
use crate::compiler::syntax::ast::ConstArg;
use crate::compiler::syntax::lex::int_literal;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** A constant argument: an integer literal, a path, or `{ expr }`. */
    pub(super) fn const_arg(&mut self) -> PResult<ConstArg> {
        match self.kind() {
            TokenKind::Int => {
                let t = self.bump();
                let value = int_literal(self.text_of(t)).map(|l| l.value).unwrap_or(0);
                Ok(ConstArg::Lit {
                    value,
                    span: t.span,
                })
            }
            TokenKind::LBrace => {
                self.bump();
                let e = self.nested(|p| p.restricted(false, |p| p.expr()))?;
                self.expect(TokenKind::RBrace)?;
                Ok(ConstArg::Expr(Box::new(e)))
            }
            _ if self.at_path_start() => Ok(ConstArg::Path(self.path(PathMode::Plain)?)),
            _ => Err(self.unexpected("a constant: a literal, a name, or `{ expression }`")),
        }
    }
}
