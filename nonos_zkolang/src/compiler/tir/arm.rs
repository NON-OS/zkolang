/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The arms of a typed `match`. */

use super::{TExpr, TPat};
use crate::compiler::source::Span;

/** An arm of a `match`: its pattern, its guard if any, and its value. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TArm {
    pub pat: TPat,
    pub guard: Option<TExpr>,
    pub body: TExpr,
    /** Where the pattern is written. */
    pub span: Span,
}
