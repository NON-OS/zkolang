/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The shapes an expression takes. `RefMut` is `&mut place`, passed to a `&mut` parameter. */

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::{Arm, AssignOp, BinOp, Block, ConstArg, Expr, FieldInit, ForIter, GenericArg};
use super::{Ident, IfBranch, Lit, Path, Pattern, Type, UnOp};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum ExprKind {
    Lit(Lit),
    /** A variable, constant, function or unit variant, with optional turbofish generics. */
    Path(Path),
    Struct {
        path: Path,
        fields: Vec<FieldInit>,
    },
    Unit,
    Tuple(Vec<Expr>),
    Array(Vec<Expr>),
    Repeat(Box<Expr>, ConstArg),
    /** `( e )`, kept so the formatter preserves the author's parentheses. */
    Paren(Box<Expr>),
    Block(Box<Block>),
    /** `if a { .. } else if b { .. } else { .. }`: the branches in order and the last block. */
    If(Vec<IfBranch>, Option<Box<Block>>),
    Match {
        scrutinee: Box<Expr>,
        arms: Vec<Arm>,
    },
    For {
        pat: Pattern,
        iter: ForIter,
        body: Box<Block>,
    },
    While {
        cond: Box<Expr>,
        limit: ConstArg,
        body: Box<Block>,
    },
    Return(Option<Box<Expr>>),
    Break,
    Continue,
    Declassify(Box<Expr>),
    Unary(UnOp, Box<Expr>),
    /** Operators of one precedence, left to right: `a + b - c` is `(a + b) - c`. */
    Binary(Box<Expr>, Vec<(BinOp, Expr)>),
    Cast(Box<Expr>, Type),
    Call(Box<Expr>, Vec<Expr>),
    MethodCall {
        receiver: Box<Expr>,
        method: Ident,
        generics: Option<Vec<GenericArg>>,
        args: Vec<Expr>,
    },
    Field(Box<Expr>, Ident),
    TupleField(Box<Expr>, u32, crate::compiler::source::Span),
    Index(Box<Expr>, Box<Expr>),
    /** `place op value`, in statement position only. */
    Assign {
        op: AssignOp,
        place: Box<Expr>,
        value: Box<Expr>,
    },
    RefMut(Box<Expr>),
    /** An expression that failed to parse; the parser has reported it. */
    Error,
}
