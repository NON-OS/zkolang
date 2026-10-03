/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The edition 2025 textual include, `include "file";`, reported wherever it stands with
 * the edition 2026 form that replaces it (spec section 20.2).
 */

use super::super::parser::Parser;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::ast::{Expr, ExprKind};
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** Whether the current token begins a textual include: `include "file"`. */
    pub(in crate::compiler::syntax::parse) fn at_include(&self) -> bool {
        self.at(TokenKind::Ident)
            && self.text_of(self.tok()) == "include"
            && matches!(self.peek(1), TokenKind::Str | TokenKind::Error)
    }

    /** Report a textual include and skip it with the `;` after it, if any. */
    pub(in crate::compiler::syntax::parse) fn skip_include(&mut self) {
        self.report_include();
        self.eat(TokenKind::Semi);
    }

    /** Report a textual include, `include "file"`, and skip it. */
    pub(in crate::compiler::syntax::parse) fn report_include(&mut self) {
        let start = self.bump().span;
        let file = self.bump().span;
        self.diags.push(
            Diagnostic::error(
                Code::INCLUDE_REMOVED,
                "`include` is not part of edition 2026",
                start.to(file),
                "textual include",
            )
            .with_help(
                "declare the file as a module with `mod name;` and import its items with `use`",
            ),
        );
    }

    /** A textual include where an expression stands: reported, and an error expression. */
    pub(in crate::compiler::syntax::parse) fn include_expr(&mut self) -> Expr {
        let start = self.span();
        self.report_include();
        let span = start.to(self.prev_span());
        self.mk(ExprKind::Error, span)
    }
}
