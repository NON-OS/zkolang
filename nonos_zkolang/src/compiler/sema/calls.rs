/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The call graph: which functions each checked function calls, and where. */

use alloc::vec::Vec;

use super::cx::{Sema, State};
use crate::compiler::source::Span;
use crate::compiler::tir::{FnId, TExpr, TExprKind};

impl<'a> Sema<'a> {
    /** For each function, the functions it calls and the first call of each. */
    pub(crate) fn call_graph(&self) -> Vec<Vec<(FnId, Span)>> {
        self.fns
            .iter()
            .map(|info| {
                let mut out: Vec<(FnId, Span)> = Vec::new();
                if let State::Done(body) = &info.body {
                    body.body.each_expr(&mut |e| calls(e, &mut out));
                }
                out
            })
            .collect()
    }
}

/** Record each function `e` calls, anywhere in it, once. */
fn calls(e: &TExpr, out: &mut Vec<(FnId, Span)>) {
    if let TExprKind::Call(f, _) = &e.kind {
        if !out.iter().any(|(g, _)| g == f) {
            out.push((*f, e.span));
        }
    }
    e.each_child(&mut |c| calls(c, out));
}
