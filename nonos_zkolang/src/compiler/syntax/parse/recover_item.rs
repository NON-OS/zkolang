/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Recovery at module level, and the tokens that can begin an item. */

use super::parser::Parser;
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /** Recover at module level: skip to the next token that can begin an item. */
    pub(super) fn recover_item(&mut self) {
        self.split = None;
        let mut depth: usize = 0;
        let start = self.pos;
        loop {
            let k = self.kind();
            match k {
                TokenKind::Eof => return,
                TokenKind::LParen | TokenKind::LBracket | TokenKind::LBrace => depth += 1,
                TokenKind::RParen | TokenKind::RBracket | TokenKind::RBrace => {
                    if depth == 0 {
                        if self.pos == start {
                            self.bump();
                        }
                        return;
                    }
                    depth -= 1;
                    if depth == 0 && k == TokenKind::RBrace {
                        self.bump();
                        return;
                    }
                }
                TokenKind::Semi if depth == 0 => {
                    self.bump();
                    return;
                }
                _ if depth == 0 && self.pos > start && starts_item(k) => return,
                _ => {}
            }
            self.bump();
        }
    }
}

/** Whether a token can begin an item. */
pub(super) fn starts_item(k: TokenKind) -> bool {
    matches!(
        k,
        TokenKind::Pound
            | TokenKind::Kw(Keyword::Pub)
            | TokenKind::Kw(Keyword::Fn)
            | TokenKind::Kw(Keyword::Struct)
            | TokenKind::Kw(Keyword::Enum)
            | TokenKind::Kw(Keyword::Type)
            | TokenKind::Kw(Keyword::Const)
            | TokenKind::Kw(Keyword::Mod)
            | TokenKind::Kw(Keyword::Use)
            | TokenKind::Kw(Keyword::Impl)
    )
}
