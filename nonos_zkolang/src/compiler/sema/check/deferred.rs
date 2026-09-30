/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Checks that wait until every type is known: an operator applied to a literal whose type
 * is still open may turn out to be undefined on it once the literal takes its default, and
 * a cast of one is allowed or not by the type it ends with.
 */

use crate::compiler::sema::ty::TyId;
use crate::compiler::source::Span;

/** A check left for the end of a body. */
#[derive(Clone, Debug)]
pub enum Deferred {
    /** An operator defined on integers but not on `field`, such as `%` or `<`. */
    IntOnly {
        ty: TyId,
        span: Span,
        op: &'static str,
    },
    /** Unary `-`, defined on `field` and signed integers. */
    Negatable { ty: TyId, span: Span },
    /** The bounds of a `for` range, which must be integers. */
    RangeInt { ty: TyId, span: Span },
    /** `e as to` where the type `ty` of `e` was still open. */
    Cast { ty: TyId, to: TyId, span: Span },
}
