/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The abstract syntax tree: the shapes the parser produces and the compiler walks.
//! The set stays small on purpose, since each node lowers to a short, fixed run of
//! opcodes. One type per file.

mod const_def;
mod expr;
mod fn_def;
mod program;
mod stmt;

pub use const_def::ConstDef;
pub use expr::Expr;
pub use fn_def::FnDef;
pub use program::Ast;
pub use stmt::Stmt;
