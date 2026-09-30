/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Constant arguments, as in an array length or a generic argument list. */

use alloc::boxed::Box;

use super::super::parser::{PResult, Parser};
use super::paths::PathMode;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::ast::ConstArg;
use crate::compiler::syntax::lex::int_literal;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** A constant argument: an integer literal, a path, or `{ expr }`. */
    pub(in crate::compiler::syntax::parse) fn const_arg(&mut self) -> PResult<ConstArg> {
        match self.kind() {
            TokenKind::Int => {
                let t = self.bump();
                let lit = int_literal(self.text_of(t)).ok();
                Ok(ConstArg::Lit {
                    value: lit.map_or(0, |l| l.value),
                    suffix: lit.and_then(|l| l.suffix),
                    span: t.span,
                })
            }
            TokenKind::LBrace => {
                self.bump();
                let e = self.nested(|p| p.restricted(false, |p| p.expr()))?;
                self.expect(TokenKind::RBrace)?;
                Ok(ConstArg::Expr(Box::new(e)))
            }
            TokenKind::Ident if self.peek(1) != TokenKind::ColonColon => {
                Ok(ConstArg::Path(self.path(PathMode::Plain)?))
            }
            _ if self.at_path_start() => {
                let p = self.path(PathMode::Plain)?;
                let d = Diagnostic::error(
                    Code::UNEXPECTED_TOKEN,
                    "a constant path here goes in braces",
                    p.span,
                    "a path where a name or `{ expression }` is expected",
                )
                .with_help("write `{ a::B }`");
                self.diags.push(d);
                Ok(ConstArg::Path(p))
            }
            _ => Err(self.unexpected("a constant: a literal, a name, or `{ expression }`")),
        }
    }
}
