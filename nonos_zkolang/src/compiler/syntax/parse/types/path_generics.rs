/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The generic arguments of a path, which follow the whole path (spec section 3). */

use super::super::parser::{PResult, Parser, Reported};
use super::paths::PathMode;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::ast::PathSegment;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** The generic arguments after the path `segments`, if the mode takes them there. */
    pub(in crate::compiler::syntax::parse) fn path_generics(
        &mut self,
        segments: &mut [PathSegment],
        mode: PathMode,
    ) -> PResult<()> {
        let generics = match mode {
            PathMode::Type if self.at(TokenKind::Lt) => self.generic_args()?,
            PathMode::Expr if self.at(TokenKind::ColonColon) && self.peek(1) == TokenKind::Lt => {
                self.bump();
                self.generic_args()?
            }
            _ => return Ok(()),
        };
        let more = self.at(TokenKind::ColonColon) && self.peek(1) == TokenKind::Ident;
        match segments.last_mut() {
            Some(last) if !more => last.generics = Some(generics),
            _ => {
                let d = Diagnostic::error(
                    Code::UNEXPECTED_TOKEN,
                    "generic arguments come after the whole path",
                    self.span(),
                    "the path goes on after its generic arguments",
                )
                .with_help("write the arguments once, after the last name: `a::B<T>`");
                self.diags.push(d);
                return Err(Reported);
            }
        }
        Ok(())
    }
}
