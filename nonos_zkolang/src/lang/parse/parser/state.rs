/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The parser's state: the tokens, their offsets, the cursor and the nesting depth. */

use crate::lang::lex::Tok;

pub(crate) struct Parser<'a> {
    pub(crate) toks: &'a [Tok],
    pub(crate) spans: &'a [usize],
    pub(crate) eof: usize,
    pub(crate) pos: usize,
    /** Open nesting levels, spent and returned by `enter` and `leave`. */
    pub(crate) depth: usize,
}
