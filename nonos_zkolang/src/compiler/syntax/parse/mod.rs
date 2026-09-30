/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The parser: tokens to the syntax tree, reporting every error it can find in one run. */

mod attr_lit;
mod attrs;
mod binary;
mod binop;
mod block_errors;
mod const_arg;
mod construct;
mod consts;
mod consume;
mod cursor;
mod doc_text;
mod docs;
mod dot;
mod entry;
mod enums;
mod expect;
mod exprs;
mod fns;
mod for_expr;
mod generic_args;
mod generic_params;
mod if_expr;
mod impl_item;
mod impls;
mod item;
mod items;
mod line_end;
mod match_expr;
mod modules;
mod nesting;
mod parser;
mod path;
mod paths;
mod pattern_group;
mod pattern_lit;
mod pattern_path;
mod patterns;
mod placement;
mod placement_control;
mod placement_expr;
mod postfix;
mod primary;
mod primary_group;
mod primary_kw;
mod primary_token;
mod recover;
mod recover_item;
mod recover_owed;
mod skip_stmt;
mod stmt_expr;
mod stmt_let;
mod stmts;
mod struct_lit;
mod structs;
mod tuple_fields;
mod type_kinds;
mod types;
mod unary;
mod uses;
mod while_expr;

pub use entry::{parse_expr, parse_file};
pub use parser::MAX_NESTING;
