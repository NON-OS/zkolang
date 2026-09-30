/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The parser: tokens to the syntax tree, reporting every error it can find in one run. */

mod attr_arg;
mod attr_lit;
mod attrs;
mod binary;
mod binary_node;
mod binop;
mod block_like;
mod body;
mod call_args;
mod construct;
mod consts;
mod consume;
mod cursor;
mod doc_attrs;
mod doc_text;
mod docs;
mod docs_unused;
mod dot;
mod entry;
mod enums;
mod expect;
mod exprs;
mod fns;
mod for_expr;
mod for_range;
mod if_expr;
mod impl_item;
mod impl_members;
mod impls;
mod item;
mod items;
mod line_end;
mod lists;
mod match_arm;
mod match_expr;
mod modules;
mod nesting;
mod params;
mod parser;
mod patterns;
mod postfix;
mod primary;
mod primary_group;
mod primary_kw;
mod primary_token;
mod recovery;
mod report_unexpected;
mod stmt;
mod stmt_attrs;
mod stmt_expr;
mod stmt_let;
mod stmts;
mod struct_head;
mod struct_lit;
mod structs;
mod tuple_fields;
mod types;
mod unary;
mod uses;
mod while_expr;

pub use entry::{parse_expr, parse_file};
pub use parser::MAX_NESTING;
