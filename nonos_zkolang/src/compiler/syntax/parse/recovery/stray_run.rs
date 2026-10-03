/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Stray tokens between items: a `;`, or a closer no bracket is owed. A run of them, as
 * `;;;`, `)))` or `)]`, is one mistake and is reported once.
 */

use alloc::format;
use alloc::string::String;

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

    /**
     * Report and skip the run of stray tokens that starts at the current one, whatever
     * their kinds, unless the item before it has reported its first token. A `}` joins
     * the run only when `braces`: inside a module or block, it closes that.
     */
    pub(in crate::compiler::syntax::parse) fn skip_stray_run(&mut self, braces: bool) {
        let k = self.kind();
        let reported = self.stray_reported == Some(self.pos);
        let first = self.bump().span;
        let mut mixed = false;
        while self.at_stray_between_items() && (braces || !self.at(TokenKind::RBrace)) {
            mixed |= self.kind() != k;
            self.bump();
        }
        if reported {
            return;
        }
        let (message, label) = match (mixed, k) {
            (false, TokenKind::Semi) => {
                (format!("unexpected {}", k.describe()), "no item ends here")
            }
            (false, _) => (
                format!("unexpected {}", k.describe()),
                "no bracket to close here",
            ),
            (true, _) => (
                String::from("unexpected closing brackets"),
                "no bracket to close here",
            ),
        };
        let at = first.to(self.prev_span());
        let d = Diagnostic::error(Code::UNEXPECTED_TOKEN, message, at, label);
        self.diags.push(d);
    }
}
