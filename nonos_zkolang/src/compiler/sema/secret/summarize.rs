/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * One function's secret flow. An unlabelled parameter is label-polymorphic (section 5.4):
 * each of its parts is a slot the summary is written in. A `public` part is public inside,
 * and callers must pass it a public value; a `secret` one is secret. `main`'s parameters
 * are all labelled, and its result must be public (section 12.1).
 */

use alloc::vec::Vec;

use super::flow::Flow;
use super::summary::Summary;
use crate::compiler::diag::Diagnostic;
use crate::compiler::tir::{TFn, TProgram};

/** The summary of `f`, and what its walk found. */
pub(super) fn summarize(
    program: &TProgram,
    summaries: &[Option<Summary>],
    f: &TFn,
    is_main: bool,
) -> (Summary, Vec<Diagnostic>) {
    let mut flow = Flow::new(program, summaries, f);
    let needs = flow.params(is_main);
    let tail = flow.block(&f.body);
    let at = f.body.tail.as_ref().map_or(f.span, |t| t.span);
    let result = tail.join(&flow.returned).raised(flow.ret_guard);
    let result = flow.labelled(result, &f.ret_labels.0, f.ret, at);
    if is_main {
        flow.require_public(result.all(), at, "the result of `main`, which is public");
    }
    let ref_out = f
        .params
        .iter()
        .map(|p| p.by_ref.then(|| flow.ref_final(p.local.0 as usize)));
    let ref_out = ref_out.collect();
    let needs_public = flow.needs_public | needs;
    (
        Summary {
            result,
            needs_public,
            ref_out,
        },
        flow.found,
    )
}
