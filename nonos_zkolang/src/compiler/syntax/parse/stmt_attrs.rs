/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Attributes where no item follows: before a statement or inside an expression. They are
 * parsed, reported once and dropped, and what follows them parses as if they were not
 * there.
 */

use super::parser::{starts_item, PResult, Parser};
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::source::Span;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /**
     * The attributes before a statement. An item after them is reported as out of place,
     * which covers its attributes too.
     */
    pub(super) fn stmt_attrs(&mut self) -> PResult<()> {
        let start = self.span();
        let mut inner = false;
        while self.at(TokenKind::Pound) {
            let bang = self.peek(1) == TokenKind::Bang;
            inner |= bang;
            self.attr(bang)?;
        }
        if start == self.span() || starts_item(self.kind()) {
            return Ok(());
        }
        let (message, help) = if inner {
            (
                "an inner attribute belongs at the start of a file or module",
                "move it before the module's first item",
            )
        } else {
            (
                "a statement takes no attributes",
                "attributes apply to items",
            )
        };
        self.misplaced_attrs(start.to(self.prev_span()), message, help);
        Ok(())
    }

    /** The attributes before an expression, which takes none. */
    pub(super) fn expr_attrs(&mut self) -> PResult<()> {
        let start = self.span();
        while self.at(TokenKind::Pound) {
            let bang = self.peek(1) == TokenKind::Bang;
            self.attr(bang)?;
        }
        let at = start.to(self.prev_span());
        self.misplaced_attrs(
            at,
            "an expression takes no attributes",
            "attributes apply to items",
        );
        Ok(())
    }

    fn misplaced_attrs(&mut self, at: Span, message: &str, help: &str) {
        let d = Diagnostic::error(Code::UNEXPECTED_TOKEN, message, at, "not allowed here")
            .with_help(help);
        self.diags.push(d);
    }
}
