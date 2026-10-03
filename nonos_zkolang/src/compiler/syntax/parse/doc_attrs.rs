/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * What stands before an item or field, and at the start of a module: doc comments and
 * attributes, in any order. Every doc comment among them documents what follows (spec
 * section 17.2). An inner attribute after a module's first item is reported and skipped.
 */

use alloc::string::String;
use alloc::vec::Vec;

use super::parser::Parser;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::ast::Attr;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** The doc comments and `#[...]` attributes before an item or field. */
    pub(super) fn doc_and_attrs(&mut self) -> (Option<String>, Vec<Attr>) {
        let mut doc = self.take_doc(self.span().lo);
        let mut attrs = Vec::new();
        while self.at(TokenKind::Pound) {
            let inner = self.peek(1) == TokenKind::Bang;
            let Some(a) = self.attr_or_skip(inner) else {
                doc = join(doc, self.take_doc(self.span().lo));
                continue;
            };
            if inner {
                let d = Diagnostic::error(
                    Code::UNEXPECTED_TOKEN,
                    "an inner attribute comes before the items of its module",
                    a.span,
                    "after an item",
                )
                .with_help("move it to the start of the file or module, or write `#[...]` for the next item");
                self.diags.push(d);
            } else {
                attrs.push(a);
            }
            doc = join(doc, self.take_doc(self.span().lo));
        }
        (doc, attrs)
    }

    /** The inner doc comments and `#![...]` attributes at a module's start, after `after`. */
    pub(super) fn inner_doc_and_attrs(&mut self, after: u32) -> (Option<String>, Vec<Attr>) {
        let mut doc = self.take_inner_doc(after, self.span().lo);
        let mut attrs = Vec::new();
        while self.at(TokenKind::Pound) && self.peek(1) == TokenKind::Bang {
            attrs.extend(self.attr_or_skip(true));
            let more = self.take_inner_doc(self.prev_span().hi, self.span().lo);
            doc = join(doc, more);
        }
        (doc, attrs)
    }
}

/** Two runs of documentation as one. */
fn join(a: Option<String>, b: Option<String>) -> Option<String> {
    match (a, b) {
        (Some(mut a), Some(b)) => {
            a.push('\n');
            a.push_str(&b);
            Some(a)
        }
        (a, None) => a,
        (None, b) => b,
    }
}
