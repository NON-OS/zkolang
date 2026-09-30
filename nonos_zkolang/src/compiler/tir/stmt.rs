/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Blocks, statements, and the irrefutable patterns `let` and parameters bind. */

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use super::lit::TLit;
use super::{LocalId, TExpr};
use crate::compiler::source::Span;

/** Statements in order, then the value of the block. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct TBlock {
    pub stmts: Vec<TStmt>,
    pub tail: Option<Box<TExpr>>,
    pub span: Span,
}

/** A statement. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TStmt {
    Let {
        pat: TPat,
        init: TExpr,
    },
    Assert {
        cond: TExpr,
        message: Option<String>,
    },
    Expr(TExpr),
}

/**
 * A pattern. `let` and parameters take only the ones that always match: bindings, `_` and
 * tuples of them (a tuple, struct or array pattern). A `match` arm takes any.
 */
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TPat {
    Bind(LocalId),
    Wild,
    Tuple(Vec<TPat>),
    /** A literal: a `bool`, or an integer's value; the span is where it is written. */
    Lit(TLit, Span),
    /** An inclusive range of integer values, and where it is written. */
    Range(i128, i128, Span),
    /** The variant of the tag, and a pattern for each of its fields in order. */
    Variant(u32, Vec<TPat>),
    /** Any of the alternatives, each binding the same locals. */
    Or(Vec<TPat>),
}
