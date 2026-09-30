/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The parser state. Its primitives, the token cursor, expectation with a diagnostic,
 * recovery, node ids and the nesting budget, are methods of `Parser` in the files beside
 * this one.
 *
 * Every parsing function either returns a node or `Err(Reported)` after pushing a
 * diagnostic.
 */

use alloc::vec::Vec;

use crate::compiler::diag::Diagnostics;
use crate::compiler::source::{FileId, Span};
use crate::compiler::syntax::lex::Comment;
use crate::compiler::syntax::token::Token;

pub(super) use super::recovery::starts_item;

/** A diagnostic has been pushed; the caller recovers. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Reported;

/** The result of a parsing function. */
pub type PResult<T> = Result<T, Reported>;

/**
 * The deepest the source may nest (spec section 1): each bracket, block, argument list,
 * prefix operator, cast, postfix link and operator of another precedence spends one level
 * while open. Every later stage walks the tree recursively, and this bound keeps that
 * within a small host stack, with room to spare in a debug build.
 */
pub const MAX_NESTING: usize = 64;

/** The parser over one file: its tokens and comments, the cursor, and the shared counters. */
pub struct Parser<'a> {
    pub(super) file: FileId,
    pub(super) text: &'a str,
    pub(super) tokens: &'a [Token],
    pub(super) comments: &'a [Comment],
    /** Text the lexer reported and left out of the token stream, in order. */
    pub(super) strays: &'a [Span],
    /** Which comments an item or module has taken as its documentation. */
    pub(super) doc_used: Vec<bool>,
    pub(super) pos: usize,
    pub(super) diags: &'a mut Diagnostics,
    pub(super) next_id: &'a mut u32,
    pub(super) depth: usize,
    pub(super) nesting_reported: bool,
    /** The second half of a `>>`, `>=` or `>>=` split to close a generic argument list. */
    pub(super) split: Option<Token>,
    /**
     * Whether a struct literal is disallowed here, in the head of `if`, `while`, `match`
     * and `for`.
     */
    pub(super) no_struct: bool,
    /** Whether the type about to be parsed is a parameter's, where `&mut T` may stand. */
    pub(super) ref_mut_ok: bool,
    /** The indentation of the line the innermost item being parsed starts on. */
    pub(super) item_indent: usize,
    /** Whether an unclosed block has been reported since the current item began. */
    pub(super) unclosed_reported: bool,
}
