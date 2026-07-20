/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The syntax layer: the abstract syntax tree and the recursive-descent parser
//! that builds it. The tree types live apart from the parser that produces them,
//! so the compiler can walk the shapes without pulling in the parsing machinery.

mod ast;
mod parser;

pub use ast::{Ast, ConstDef, Expr, FnDef, Stmt};
pub use parser::parse;
