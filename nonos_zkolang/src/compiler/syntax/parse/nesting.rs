/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The nesting budget, and the struct-literal restriction scoped like it. */

use super::parser::{PResult, Parser, Reported, MAX_NESTING};
use crate::compiler::diag::{Code, Diagnostic};

impl<'a> Parser<'a> {
    /** Open one nesting level. */
    pub(super) fn enter(&mut self) -> PResult<()> {
        if self.depth >= MAX_NESTING {
            if !self.nesting_reported {
                self.nesting_reported = true;
                self.diags.push(
                    Diagnostic::error(
                        Code::NESTING_TOO_DEEP,
                        "nested too deep",
                        self.span(),
                        "the parser's nesting budget runs out here",
                    )
                    .with_help("split the expression into `let` bindings"),
                );
            }
            return Err(Reported);
        }
        self.depth += 1;
        Ok(())
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
