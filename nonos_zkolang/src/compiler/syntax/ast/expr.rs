/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! An expression as written: its node. */

use super::{ExprKind, NodeId};
use crate::compiler::source::Span;

/** An expression. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Expr {
    pub id: NodeId,
    pub kind: ExprKind,
    pub span: Span,
}
