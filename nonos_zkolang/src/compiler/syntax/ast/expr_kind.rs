/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The shapes an expression takes. `RefMut` is `&mut place`, passed to a `&mut` parameter. */

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::{Arm, AssignOp, BinOp, Block, ConstArg, Expr, FieldInit, ForIter, GenericArg};
use super::{Ident, Lit, Path, Pattern, Type, UnOp};

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
    If {
        cond: Box<Expr>,
        then_block: Box<Block>,
        else_branch: Option<Box<Expr>>,
    },
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
    Binary(BinOp, Box<Expr>, Box<Expr>),
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
