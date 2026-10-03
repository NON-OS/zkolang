/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Blocks (section 8.9). A block whose statements leave it with `return`, `break`,
 * `continue` or `assert false` has type `!` when it has no tail, and what follows the
 * leaving statement is unreachable (W0003).
 */

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::cx::FnCx;
use crate::compiler::sema::ty::{TyId, Types};
use crate::compiler::syntax::ast::{Block, StmtKind};
use crate::compiler::tir::TBlock;

impl<'s, 'a> FnCx<'s, 'a> {
    /** The block `b`, where a value of type `want` is expected, and its type. */
    pub(crate) fn block(&mut self, b: &'a Block, want: Option<TyId>) -> (TBlock, TyId) {
        self.push_scope();
        let mut stmts = Vec::with_capacity(b.stmts.len());
        let (mut diverges, mut warned) = (false, false);
        for s in &b.stmts {
            if diverges && !warned && !matches!(s.kind, StmtKind::Empty) {
                self.unreachable(s.span);
                warned = true;
            }
            if let Some(t) = self.stmt(s, &mut diverges) {
                stmts.push(t);
            }
        }
        if let (true, false, Some(t)) = (diverges, warned, &b.tail) {
            self.unreachable(t.span);
        }
        let tail = b.tail.as_ref().map(|t| Box::new(self.infer(t, want)));
        self.pop_scope();
        let ty = match &tail {
            Some(t) => t.ty,
            None if diverges => Types::NEVER,
            None => Types::UNIT,
        };
        let span = b.span;
        (TBlock { stmts, tail, span }, ty)
    }
}
