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
 * The deepest the parser nests, counting every edge of the tree it builds. A debug build
 * compiles a chain of about 380 links on a 2 MiB stack, so 256 leaves room; the deepest
 * program in the tree, a hash round written as nested calls, spends a few dozen.
 */
pub(crate) const MAX_NESTING: usize = 256;

/**
 * What one nested expression spends. Parentheses, an argument or a block recurse through
 * every precedence level, which in a debug build takes two to three times the stack of a
 * chain link, so a nested expression costs three links.
 */
pub(crate) const EXPR_COST: usize = 3;

impl<'a> Parser<'a> {
    /** Open one nesting level. */
    pub(crate) fn enter(&mut self) -> Result<(), CompileError> {
        self.spend(1)
    }

    /** Open `n` nesting levels at once. */
    pub(crate) fn spend(&mut self, n: usize) -> Result<(), CompileError> {
        self.depth = self.depth.saturating_add(n);
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
