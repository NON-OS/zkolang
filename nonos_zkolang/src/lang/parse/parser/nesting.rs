/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The nesting budget. Every construct that nests, an expression inside parentheses, an
 * argument, an index or a block, a prefix operator, a loop body, and every link of an
 * operator chain, spends one level while it is open. The tree the parser builds is then
 * never deeper than a small multiple of the budget, which the compiler, the optimizer and
 * dropping the tree all walk recursively on whatever stack the host gives them. Past the
 * budget the parser stops with an error at the token that went too deep.
 */

use super::Parser;
use crate::lang::CompileError;

/**
 * The deepest the parser nests. Real programs stay far below it: the deepest in the tree,
 * a hash round written as nested calls, opens about a dozen levels.
 */
pub(crate) const MAX_NESTING: usize = 128;

impl<'a> Parser<'a> {
    /** Open one nesting level. */
    pub(crate) fn enter(&mut self) -> Result<(), CompileError> {
        self.depth += 1;
        if self.depth > MAX_NESTING {
            return Err(CompileError::NestingTooDeep { at: self.at() });
        }
        Ok(())
    }

    /** Close `n` nesting levels. */
    pub(crate) fn leave(&mut self, n: usize) {
        self.depth = self.depth.saturating_sub(n);
    }
}
