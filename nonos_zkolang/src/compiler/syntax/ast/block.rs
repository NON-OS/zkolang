/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Blocks and statements as written. */

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

use super::{Expr, NodeId, Pattern, Type};
use crate::compiler::source::Span;

/** A block: statements, then an optional tail expression that is the block's value. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Block {
    pub id: NodeId,
    pub stmts: Vec<Stmt>,
    pub tail: Option<Box<Expr>>,
    pub span: Span,
}

/** A statement. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Stmt {
    pub id: NodeId,
    pub kind: StmtKind,
    pub span: Span,
}

/** The shapes a statement takes. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum StmtKind {
    /** `let pat: ty = init;` */
    Let {
        pat: Pattern,
        ty: Option<Type>,
        init: Expr,
    },
    /** `assert cond, "message";` */
    Assert { cond: Expr, message: Option<String> },
    /** An expression statement; `semi` records whether it ended in `;`. */
    Expr { expr: Expr, semi: bool },
    /** A lone `;`. */
    Empty,
}
