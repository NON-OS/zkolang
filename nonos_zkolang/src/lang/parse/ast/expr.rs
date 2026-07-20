/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! An expression node. Each variant lowers to a small, fixed run of opcodes.
//! Division, negation, and not-equal are sugar; the conditional is the branchless
//! select; an index reads a constant table or an array; an array literal is a vector.

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Clone, Debug)]
pub enum Expr {
    Num(u64),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Neg(Box<Expr>),
    Eq(Box<Expr>, Box<Expr>),
    Ne(Box<Expr>, Box<Expr>),
    Lt(Box<Expr>, Box<Expr>),
    Inv(Box<Expr>),
    Sel(Box<Expr>, Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Call(String, Vec<Expr>),
    Index(Box<Expr>, Box<Expr>),
    Array(Vec<Expr>),
}
