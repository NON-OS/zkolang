/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Where a bracket an error left open is taken to end during recovery. */

use super::super::parser::{starts_item, Parser};
use crate::compiler::syntax::keyword::Keyword;
use crate::compiler::syntax::token::TokenKind;

/** How far ahead a `;` looks for the closer of its bracket. */
const LOOKAHEAD: usize = 64;

impl<'a> Parser<'a> {
    /**
     * Whether `k`, the current token, cannot continue a bracket that `closer` closes: a `let`,
     * an item keyword starting a line, or a `;` unless the closer follows on the same line,
     * as in `[a, b; c]`, where the `;` is the mistake and the `]` still ends the bracket.
     */
    pub(in crate::compiler::syntax::parse) fn ends_bracket(
        &self,
        k: TokenKind,
        closer: TokenKind,
    ) -> bool {
        match k {
            TokenKind::Kw(Keyword::Let) => true,
            TokenKind::Semi => !self.closer_on_line(closer),
            _ => starts_item(k) && self.line_end_before(self.span()).is_some(),
        }
    }

    /** Whether `closer` comes later on the current line, within a few tokens. */
    pub(in crate::compiler::syntax::parse) fn closer_on_line(&self, closer: TokenKind) -> bool {
        let ahead = self
            .tokens
            .get(self.pos..)
            .unwrap_or(&[])
            .windows(2)
            .take(LOOKAHEAD);
        for w in ahead {
            let [a, b] = w else {
                return false;
            };
            let gap = self
                .text
                .get(a.span.hi as usize..b.span.lo as usize)
                .unwrap_or("");
            if gap.contains(['\n', '\r']) || b.kind == TokenKind::Eof {
                return false;
            }
            if b.kind == closer {
                return true;
            }
        }
        false
    }
}
