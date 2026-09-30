/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Generic arguments, and the constants written where a type needs one. */

use alloc::boxed::Box;

use super::{Expr, Path, Type};
use crate::compiler::source::Span;

/** A generic argument: a type, or a constant. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum GenericArg {
    Type(Type),
    Const(ConstArg),
}

/**
 * A constant written where a type needs one: an array length or a constant generic
 * argument.
 */
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ConstArg {
    /** An integer literal. */
    Lit { value: u64, span: Span },
    /** A path: a constant or a constant generic parameter. */
    Path(Path),
    /** `{ expr }`, an arbitrary constant expression. */
    Expr(Box<Expr>),
}

impl ConstArg {
    /** Where the argument is. */
    pub fn span(&self) -> Span {
        match self {
            ConstArg::Lit { span, .. } => *span,
            ConstArg::Path(p) => p.span,
            ConstArg::Expr(e) => e.span,
        }
    }
}
