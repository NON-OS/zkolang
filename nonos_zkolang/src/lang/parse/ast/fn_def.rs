/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! A function definition. Functions are compile-time inlined, so the body is a single
//! expression substituted at each call site; there is no call stack and no recursion.

use alloc::string::String;
use alloc::vec::Vec;

use super::Expr;

#[derive(Clone, Debug)]
pub struct FnDef {
    pub name: String,
    pub params: Vec<String>,
    pub body: Expr,
}
