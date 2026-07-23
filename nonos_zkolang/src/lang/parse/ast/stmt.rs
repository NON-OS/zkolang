/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! A statement node.

use alloc::string::String;
use alloc::vec::Vec;

use super::Expr;

#[derive(Clone, Debug)]
pub enum Stmt {
    Let(String, Expr),
    /// Destructure the several values a tuple expression produces into names, `let (x, y) = e;`.
    LetTuple(Vec<String>, Expr),
    Assert(Expr),
    Input(String),
    Secret(String),
    Output(Expr),
    For {
        var: String,
        lo: u64,
        hi: u64,
        body: Vec<Stmt>,
    },
}
