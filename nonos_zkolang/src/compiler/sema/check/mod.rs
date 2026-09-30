/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Checking bodies: every expression typed, bidirectionally, against the type its position
 * expects, into the typed IR. Integer literals whose type is open are settled at the end.
 */

mod args;
mod assign;
mod block;
mod block_warn;
mod body_const;
mod call;
mod call_fn;
mod call_item;
mod call_report;
mod cast;
mod cast_report;
mod compound;
mod constness;
mod cx;
mod deferred;
mod disjoint;
mod enumerate;
mod exits;
mod expr;
mod field;
mod finish;
mod finish_checks;
mod for_array;
mod for_range;
mod generic_call;
mod generic_value;
mod helpers;
mod if_else;
mod index;
mod infer;
mod int_or_field;
mod labels;
mod let_stmt;
mod lit;
mod loops;
mod matching;
mod methods;
mod mismatch;
mod op_sym;
mod ops;
mod ops_rules;
mod params;
mod params_named;
mod pat;
mod pat_report;
mod path_expr;
mod path_kind;
mod place;
mod post;
mod prim;
mod prim_const;
mod ref_mut;
mod repeat;
mod rewrite;
mod rewrite_block;
mod rewrite_lit;
mod scope;
mod stmt;
mod structs;
mod unary;
mod unused;

pub use cx::FnCx;
