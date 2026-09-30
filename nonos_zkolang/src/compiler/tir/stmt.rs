/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Blocks, statements, and the irrefutable patterns `let` and parameters bind. */

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

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

/** A pattern that always matches: a binding, `_`, or a tuple of them. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TPat {
    Bind(LocalId),
    Wild,
    Tuple(Vec<TPat>),
}
