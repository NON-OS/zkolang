/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Every `declassify` of a program (section 13.2), for `zkolang check --declassify`: where
 * each is written, once however many instances of a generic function hold it.
 */

use alloc::vec::Vec;

use crate::compiler::source::Span;
use crate::compiler::tir::{TExpr, TExprKind, TProgram};

/** The span of each `declassify` in the functions of `p`, in source order. */
pub fn declassified(p: &TProgram) -> Vec<Span> {
    let mut out = Vec::new();
    for f in &p.fns {
        f.body.each_expr(&mut |e| walk(e, &mut out));
    }
    out.sort_by_key(|s| (s.file.0, s.lo, s.hi));
    out.dedup();
    out
}

/** Add the span of each `declassify` in `e` to `out`. */
fn walk(e: &TExpr, out: &mut Vec<Span>) {
    if let TExprKind::Declassify(_) = e.kind {
        out.push(e.span);
    }
    e.each_child(&mut |c| walk(c, out));
}
