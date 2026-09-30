/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Stray tokens between items: a `;`, or a closer no bracket is owed. A run of the same
 * one, as `;;;` or `)))`, is one mistake and is reported once.
 */

use alloc::format;

use super::super::parser::Parser;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** Whether the current token is a stray one between items. */
    pub(in crate::compiler::syntax::parse) fn at_stray_between_items(&self) -> bool {
        matches!(
            self.kind(),
            TokenKind::Semi | TokenKind::RParen | TokenKind::RBracket | TokenKind::RBrace
        )
    }

    /** Report and skip the run of stray tokens that starts at the current one. */
    pub(in crate::compiler::syntax::parse) fn skip_stray_run(&mut self) {
        let k = self.kind();
        let first = self.bump().span;
        while self.at(k) {
            self.bump();
        }
        let at = first.to(self.prev_span());
        let label = if k == TokenKind::Semi {
            "no item ends here"
        } else {
            "no bracket to close here"
        };
        let d = Diagnostic::error(
            Code::UNEXPECTED_TOKEN,
            format!("unexpected {}", k.describe()),
            at,
            label,
        );
        self.diags.push(d);
    }
}
