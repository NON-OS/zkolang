/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Building a parser over one file's tokens, and numbering the nodes it makes. */

use super::parser::Parser;
use crate::compiler::diag::Diagnostics;
use crate::compiler::source::FileId;
use crate::compiler::syntax::ast::NodeId;
use crate::compiler::syntax::lex::Comment;
use crate::compiler::syntax::token::Token;

impl<'a> Parser<'a> {
    /** A parser at the first token, reporting into `diags` and numbering from `next_id`. */
    pub fn new(
        file: FileId,
        text: &'a str,
        tokens: &'a [Token],
        comments: &'a [Comment],
        diags: &'a mut Diagnostics,
        next_id: &'a mut u32,
    ) -> Parser<'a> {
        Parser {
            file,
            text,
            tokens,
            comments,
            doc_used: alloc::vec![false; comments.len()],
            pos: 0,
            diags,
            next_id,
            depth: 0,
            nesting_reported: false,
            split: None,
            no_struct: false,
        }
    }

    /** A fresh node id. */
    pub(super) fn id(&mut self) -> NodeId {
        let id = NodeId(*self.next_id);
        *self.next_id = self.next_id.saturating_add(1);
        id
    }
}
