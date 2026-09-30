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
                _ if self.at_stray_between_items() => {
                    self.skip_stray_run(!in_block);
                    continue;
                }
                _ => {}
            }
            let before = self.pos;
            match self.item() {
                Ok(Some(item)) => items.push(item),
                Ok(None) => {}
                Err(_) => {
                    self.recover_item(before);
                    if self.pos == before {
                        self.bump();
                    }
                }
            }
        }
        items
    }
}
