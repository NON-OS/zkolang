/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The nesting budget, and the struct-literal restriction scoped like it. */

use super::parser::{PResult, Parser, Reported, MAX_NESTING};
use crate::compiler::diag::{Code, Diagnostic};
use crate::compiler::syntax::token::TokenKind;

impl<'a> Parser<'a> {
    /**
     * Open one nesting level. Past the bound, the first construct too deep in each
     * top-level item is reported, at the bracket just opened or else at the current token.
     */
    pub(super) fn enter(&mut self) -> PResult<()> {
        if self.depth < MAX_NESTING {
            self.depth += 1;
            return Ok(());
        }
        if !self.nesting_reported {
            self.nesting_reported = true;
            let prev = self.pos.checked_sub(1).and_then(|i| self.tokens.get(i));
            let opens = |k| {
                matches!(
                    k,
                    TokenKind::LParen | TokenKind::LBracket | TokenKind::LBrace | TokenKind::Lt
                )
            };
            let at = match prev {
                Some(t) if self.split.is_none() && opens(t.kind) => t.span,
                _ => self.span(),
            };
            let d = Diagnostic::error(
                Code::NESTING_TOO_DEEP,
                "nested too deep",
                at,
                "this goes past the nesting bound",
            )
            .with_help("nesting is bounded at 64 levels: split the construct into parts with names of their own");
            self.diags.push(d);
        }
        Err(Reported)
    }

    /** Close `n` nesting levels. */
    pub(super) fn leave(&mut self, n: usize) {
        self.depth = self.depth.saturating_sub(n);
    }

    /** Run `f` one nesting level deeper. */
    pub(super) fn nested<T>(&mut self, f: impl FnOnce(&mut Self) -> PResult<T>) -> PResult<T> {
        self.enter()?;
        let r = f(self);
        self.leave(1);
        r
    }

    /** Run `f` with the struct-literal restriction set to `no_struct`. */
    pub(super) fn restricted<T>(
        &mut self,
        no_struct: bool,
        f: impl FnOnce(&mut Self) -> PResult<T>,
    ) -> PResult<T> {
        let saved = self.no_struct;
        self.no_struct = no_struct;
        let r = f(self);
        self.no_struct = saved;
        r
    }
}
