/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The report of an unexpected token. It points at the token found, except where a
 * separator or closer is missing: that belongs after the last token of its line, so when
 * a line ends before the token found, the report points there and marks the token found
 * as a second label. A token missing at the end of the file is reported just after the
 * last token.
 */

use alloc::format;
use alloc::string::String;

use super::parser::{Parser, Reported};
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::source::Span;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** Report the current token where `expected` was; `after_line` if a separator is missing. */
    pub(super) fn report_unexpected(&mut self, expected: &str, after_line: bool) -> Reported {
        if let Some(d) = self.unexpected_diag(expected, after_line) {
            self.diags.push(d);
        }
        Reported
    }

    /**
     * The report of the current token where `expected` was, or `None` when the lexer
     * already reported this text or text just before it.
     */
    pub(super) fn unexpected_diag(&self, expected: &str, after_line: bool) -> Option<Diagnostic> {
        let t = self.tok();
        if t.kind == TokenKind::Error || self.at_reserved() || self.after_stray() {
            return None;
        }
        let found = match t.kind {
            TokenKind::Ident => format!("`{}`", self.text_of(t)),
            k => String::from(k.describe()),
        };
        let message = format!("expected {expected}, found {found}");
        let prev = self.pos.checked_sub(1).and_then(|i| self.tokens.get(i));
        let end = |p: Span| Span::new(p.file, p.hi, p.hi);
        let at_end = match (t.kind, prev) {
            (TokenKind::Eof, Some(p)) => Some(end(p.span)),
            _ if after_line => self.line_end_before(t.span),
            _ => None,
        };
        Some(match at_end {
            Some(at) => {
                let label = format!("expected {expected} after this");
                let d = Diagnostic::error(Code::UNEXPECTED_TOKEN, message, at, label);
                if t.kind == TokenKind::Eof {
                    d
                } else {
                    d.with_label(t.span, format!("found {found}"))
                }
            }
            None => {
                let label = format!("expected {expected}");
                Diagnostic::error(Code::UNEXPECTED_TOKEN, message, t.span, label)
            }
        })
    }
}
