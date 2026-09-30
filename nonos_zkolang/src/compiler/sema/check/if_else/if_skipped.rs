/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The blocks a constant condition does not take (section 8.4) are not checked; a local
 * a name in one could read counts as read, so W0001 is a fact of the text and not of one
 * instance of it.
 */

use alloc::string::String;
use alloc::vec::Vec;

use super::super::cx::FnCx;
use crate::compiler::syntax::ast::{Block, Expr, ExprKind, IfBranch};

impl<'s, 'a> FnCx<'s, 'a> {
    /** Count as read each local a name in the branches `rest` or the block `last` names. */
    pub(super) fn read_skipped(&mut self, rest: &[IfBranch], last: Option<&Block>) {
        let mut names: Vec<String> = Vec::new();
        let mut note = |e: &Expr| match &e.kind {
            ExprKind::Path(p) => names.extend(p.as_ident().map(|i| i.name.clone())),
            ExprKind::Struct { fields, .. } => names.extend(
                fields
                    .iter()
                    .filter(|x| x.value.is_none())
                    .map(|x| x.name.name.clone()),
            ),
            _ => {}
        };
        for br in rest {
            br.cond.each_expr(&mut note);
            br.block.each_expr(&mut note);
        }
        if let Some(b) = last {
            b.each_expr(&mut note);
        }
        for n in names {
            if let Some(l) = self.lookup(&n) {
                self.read_local(l);
            }
        }
    }
}
