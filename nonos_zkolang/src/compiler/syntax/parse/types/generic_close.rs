/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The end of a generic list: its `>`, which may be the first half of a `>>`, `>=` or
 * `>>=`, and recovery to it after an error inside the list.
 */

use super::super::parser::{PResult, Parser, Reported};
use crate::compiler::syntax::token::{Token, TokenKind};

impl<'a> Parser<'a> {
    /**
     * Whether the current token closes a generic list, possibly as the first half of a
     * `>>`, `>=` or `>>=`.
     */
    pub(in crate::compiler::syntax::parse) fn at_generic_close(&self) -> bool {
        matches!(
            self.kind(),
            TokenKind::Gt | TokenKind::Shr | TokenKind::Ge | TokenKind::ShrEq
        )
    }

    /** Consume one `>`, splitting a `>>`, `>=` or `>>=` and leaving its rest current. */
    pub(in crate::compiler::syntax::parse) fn close_generics(&mut self) -> PResult<()> {
        let t = self.tok();
        let rest = match t.kind {
            TokenKind::Gt => {
                self.bump();
                return Ok(());
            }
            TokenKind::Shr => TokenKind::Gt,
            TokenKind::Ge => TokenKind::Eq,
            TokenKind::ShrEq => TokenKind::Ge,
            _ => return Err(self.unexpected("`>`")),
        };
        self.bump();
        let lo = t.span.lo.saturating_add(1);
        let mut span = t.span;
        span.lo = lo.min(t.span.hi);
        self.split = Some(Token { kind: rest, span });
        Ok(())
    }

    /**
     * Report a token where a generic list goes on or ends. A `;` in an argument list gets
     * a help: `C<u8; 3>` is most likely an array type.
     */
    pub(in crate::compiler::syntax::parse) fn generic_list_end(&mut self, args: bool) -> Reported {
        let semi = args && self.at(TokenKind::Semi);
        if let Some(d) = self.unexpected_diag("`,` or `>`", true) {
            let d = if semi {
                d.with_help("an array type is written `[T; N]`")
            } else {
                d
            };
            self.diags.push(d);
        }
        Reported
    }
}
