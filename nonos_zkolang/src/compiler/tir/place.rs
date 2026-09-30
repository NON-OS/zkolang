/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Places (section 8.2), and the arguments of a call, which pass a value or a place. */

use alloc::vec::Vec;

use super::{LocalId, TExpr};
use crate::compiler::sema::ty::TyId;
use crate::compiler::source::Span;

/** A step from a place to a part of it. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Proj {
    TupleField(u32),
    Index(TExpr),
}

/** A local and a path into it: `x`, `x.0`, `x[i].1`. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TPlace {
    pub root: LocalId,
    pub proj: Vec<Proj>,
    /** The type of the part the place names. */
    pub ty: TyId,
    pub span: Span,
}

/** An argument: a value, or `&mut place` for a `&mut` parameter (section 10.3). */
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TArg {
    Value(TExpr),
    Place(TPlace),
}
