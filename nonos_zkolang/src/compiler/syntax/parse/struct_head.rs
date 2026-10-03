/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A struct literal in the head of `if`, `while`, `match` or `for`, where a `{` opens the
 * body. When what follows the `{` can only be a literal's fields, the literal is reported
 * and read, so parsing goes on.
 */

use super::parser::Parser;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::source::Span;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** Whether the `{` current opens a struct literal: a block cannot start `name:` or `name,`. */
    pub(super) fn struct_literal_ahead(&self) -> bool {
        self.peek(1) == TokenKind::Ident
            && matches!(self.peek(2), TokenKind::Colon | TokenKind::Comma)
    }

    /** Report the struct literal at `at`, in the head of a control-flow expression. */
    pub(super) fn struct_in_head(&mut self, at: Span) {
        let d = Diagnostic::error(
            Code::STRUCT_LITERAL_HERE,
            "a struct literal in a condition",
            at,
            "read as the start of the body",
        )
        .with_help("wrap the struct literal in parentheses");
        self.diags.push(d);
    }
}
