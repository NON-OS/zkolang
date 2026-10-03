/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! What may follow a `::` after the last segment of a path. */

use super::super::parser::{PResult, Parser, Reported};
use super::paths::PathMode;
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /**
     * Check the token after a `::` that no segment follows, the `::` current: a `<` in an
     * expression, a `*` or `{` in a `use` tree. Anything else is reported after the `::`,
     * with a help for a word that only begins a path.
     */
    pub(in crate::compiler::syntax::parse) fn after_colons(
        &mut self,
        mode: PathMode,
    ) -> PResult<()> {
        let next = self.peek(1);
        let expected = match mode {
            PathMode::Expr if next == TokenKind::Lt => return Ok(()),
            PathMode::Use if matches!(next, TokenKind::Star | TokenKind::LBrace) => return Ok(()),
            PathMode::Expr => "an identifier or `<`",
            PathMode::Use => "an identifier, `*` or `{`",
            PathMode::Type | PathMode::Plain => "an identifier",
        };
        self.bump();
        let root = matches!(
            self.kind(),
            TokenKind::Kw(Keyword::Crate | Keyword::Super | Keyword::SelfValue | Keyword::SelfType)
        );
        if let Some(d) = self.unexpected_diag(expected, false) {
            let d = if root {
                d.with_help("`crate`, `super`, `self` and `Self` only begin a path")
            } else {
                d
            };
            self.diags.push(d);
        }
        Err(Reported)
    }
}
