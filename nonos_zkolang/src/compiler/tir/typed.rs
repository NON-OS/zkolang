/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A typed expression: its shape, its type and its place in the source. */

use super::TExprKind;
use crate::compiler::sema::ty::TyId;
use crate::compiler::source::Span;

/** An expression, its type, and where it is written. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TExpr {
    pub kind: TExprKind,
    pub ty: TyId,
    pub span: Span,
}
