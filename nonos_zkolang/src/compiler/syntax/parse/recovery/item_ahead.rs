/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The item keyword a run of attributes and `pub` leads to, looked up without parsing. */

use super::super::parser::Parser;
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

/** How many tokens of attributes the look-ahead passes before it gives up. */
const ATTR_TOKENS: usize = 256;

impl<'a> Parser<'a> {
    /**
     * The item keyword at the current token, after any attributes and `pub`, with the
     * token after it; `None` if no item starts here.
     */
    pub(in crate::compiler::syntax::parse) fn item_keyword_ahead(
        &self,
    ) -> Option<(TokenKind, TokenKind)> {
        self.item_keyword_at().map(|(_, k, next)| (k, next))
    }

    /** As `item_keyword_ahead`, with the index of the keyword's token. */
    pub(in crate::compiler::syntax::parse) fn item_keyword_at(
        &self,
    ) -> Option<(usize, TokenKind, TokenKind)> {
        let kind = |i: usize| self.tokens.get(i).map_or(TokenKind::Eof, |t| t.kind);
        let mut i = self.pos;
        let limit = self.pos + ATTR_TOKENS;
        while kind(i) == TokenKind::Pound && i < limit {
            i += 1 + usize::from(kind(i + 1) == TokenKind::Bang);
            let mut depth = 0usize;
            loop {
                match kind(i) {
                    TokenKind::LBracket => depth += 1,
                    TokenKind::RBracket if depth <= 1 => break,
                    TokenKind::RBracket => depth -= 1,
                    TokenKind::Eof => return None,
                    _ if i >= limit => return None,
                    _ => {}
                }
                i += 1;
            }
            i += 1;
        }
        i += usize::from(kind(i) == TokenKind::Kw(Keyword::Pub));
        let (k, next) = (kind(i), kind(i + 1));
        let item = match k {
            TokenKind::Kw(Keyword::Const) => {
                matches!(next, TokenKind::Ident | TokenKind::Kw(Keyword::Fn))
            }
            TokenKind::Kw(
                Keyword::Fn
                | Keyword::Struct
                | Keyword::Enum
                | Keyword::Type
                | Keyword::Mod
                | Keyword::Use
                | Keyword::Impl,
            ) => true,
            _ => false,
        };
        item.then_some((i, k, next))
    }

    /** Whether an item starts at the current token, first on its line. */
    pub(in crate::compiler::syntax::parse) fn item_starts_line(&self) -> bool {
        self.item_keyword_ahead().is_some() && self.indent_before(self.span().lo).is_some()
    }
}
