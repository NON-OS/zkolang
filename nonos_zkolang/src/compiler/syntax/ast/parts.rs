/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The parts some expressions hold: struct literal fields, `match` arms and loop iterators. */

use alloc::boxed::Box;

use super::{Expr, Ident, Pattern};
use crate::compiler::source::Span;

/** A field initializer in a struct literal, `name: value` or the shorthand `name`. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FieldInit {
    pub name: Ident,
    pub value: Option<Expr>,
    pub span: Span,
}

/** One arm of a `match`. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Arm {
    pub pat: Pattern,
    pub guard: Option<Expr>,
    pub body: Expr,
    pub span: Span,
}

/** What a `for` loop iterates. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ForIter {
    /** `lo..hi` or `lo..=hi`. */
    Range {
        lo: Box<Expr>,
        hi: Box<Expr>,
        inclusive: bool,
    },
    /** An array value. */
    Array(Box<Expr>),
    /** `arr.enumerate()`, binding an index and an element. */
    Enumerate(Box<Expr>),
}
