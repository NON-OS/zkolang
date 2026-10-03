/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! `match` (section 8.5) and the patterns only its arms take (section 9). */

mod arm;
mod bindings;
mod exhaustive;
mod exhaustive_report;
mod int_pat;
mod lit_pat;
mod match_expr;
mod or_pat;
mod pat_cx;
mod rewrite_pat;

pub(crate) use pat_cx::PatCx;
