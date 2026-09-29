/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Recursive-descent parser: a token stream to the abstract syntax tree. Precedence
//! is encoded by the call chain, and one grammar rule lives per file: the cursor
//! primitives, the top-level items, the statements, and the expression levels.

mod and;
mod array_expr;
mod atom;
mod block;
mod const_def;
mod cursor;
mod entry;
mod equality;
mod expr;
mod fn_def;
mod for_loop;
mod ident;
mod ident_expr;
mod if_expr;
mod input_secret;
mod inv_expr;
mod match_expr;
mod nesting;
mod number;
mod or;
mod primary;
mod product;
mod program;
mod sel_expr;
mod state;
mod stmt;
mod sum;
mod unary;

pub use entry::parse;
pub(crate) use state::Parser;
