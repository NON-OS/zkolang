/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Items: functions, structs, enums, aliases, constants, modules, imports and impl blocks,
 * with their attributes, visibility and documentation.
 */

use alloc::vec::Vec;

use super::parser::Parser;
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::ast::Item;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** Items until the end of the file, or until a `}` when `in_block`. */
    pub(super) fn items(&mut self, in_block: bool) -> Vec<Item> {
        let mut items = Vec::new();
        loop {
            match self.kind() {
                TokenKind::Eof => break,
                TokenKind::RBrace if in_block => break,
                TokenKind::RBrace => {
                    let t = self.bump();
                    self.diags.push(Diagnostic::error(
                        Code::UNEXPECTED_TOKEN,
                        "unexpected `}`",
                        t.span,
                        "no block to close here",
                    ));
                    continue;
                }
                _ => {}
            }
            let before = self.pos;
            match self.item() {
                Ok(Some(item)) => items.push(item),
                Ok(None) => {}
                Err(_) => {
                    self.recover_item();
                    if self.pos == before {
                        self.bump();
                    }
                }
            }
        }
        items
    }

    /** Report a textual `include`, the current token, and skip past its `;`. */
    pub(super) fn skip_include(&mut self) {
        self.diags.push(
            Diagnostic::error(
                Code::INCLUDE_REMOVED,
                "`include` is not part of edition 2026",
                self.span(),
                "textual include",
            )
            .with_help(
                "declare the file as a module with `mod name;` and import its items with `use`",
            ),
        );
        self.skip_until(&[TokenKind::Semi]);
        self.eat(TokenKind::Semi);
    }
}
