/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! A parsed program: its constant tables, functions, and statements.

use alloc::vec::Vec;

use super::{ConstDef, FnDef, Stmt};

#[derive(Clone, Debug)]
pub struct Ast {
    pub consts: Vec<ConstDef>,
    pub fns: Vec<FnDef>,
    pub stmts: Vec<Stmt>,
}
