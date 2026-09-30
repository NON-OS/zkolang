/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Typed expressions. */

use alloc::boxed::Box;
use alloc::vec::Vec;

use super::{Builtin, ConstId, FnId, LocalId, TArg, TBlock, TExpr, TLit, TPat, TPlace};
use crate::compiler::syntax::ast::{BinOp, UnOp};

/** The shapes a typed expression takes. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum TExprKind {
    Lit(TLit),
    Local(LocalId),
    Const(ConstId),
    Unary(UnOp, Box<TExpr>),
    /** Operators of one precedence, left to right; `&&` and `||` short-circuit per link. */
    Chain(Box<TExpr>, Vec<(BinOp, TExpr)>),
    /** `e as T`, `T` being the expression's type. */
    Cast(Box<TExpr>),
    Call(FnId, Vec<TArg>),
    Builtin(Builtin, Vec<TExpr>),
    Tuple(Vec<TExpr>),
    Array(Vec<TExpr>),
    /** `[e; n]`, `e` evaluated once. */
    Repeat(Box<TExpr>, u32),
    TupleField(Box<TExpr>, u32),
    Index(Box<TExpr>, Box<TExpr>),
    Block(TBlock),
    /** `if c1 { .. } else if c2 { .. } else { .. }`: the branches, then the last block. */
    If(Vec<(TExpr, TBlock)>, Option<TBlock>),
    /** `for x in lo..hi`, or `..=hi` when `inclusive`; the bounds are constant. */
    ForRange {
        var: LocalId,
        lo: Box<TExpr>,
        hi: Box<TExpr>,
        inclusive: bool,
        body: TBlock,
    },
    /** `for pat in array`, or `for (index, pat) in array.enumerate()`. */
    ForArray {
        index: Option<LocalId>,
        pat: TPat,
        array: Box<TExpr>,
        body: TBlock,
    },
    While {
        cond: Box<TExpr>,
        limit: u32,
        body: TBlock,
    },
    Break,
    Continue,
    Return(Option<Box<TExpr>>),
    /** `place = value`, or `place op= value` with the operator. */
    Assign {
        place: TPlace,
        op: Option<BinOp>,
        value: Box<TExpr>,
    },
    Declassify(Box<TExpr>),
    /** What failed to check; the checker has reported it. */
    Error,
}
