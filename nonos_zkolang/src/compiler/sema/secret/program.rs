/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The secret-flow check of a program (section 13): every function that checked is
 * summarized after the functions it calls, and `main`'s parameters must each be labelled
 * (E0601).
 */

use alloc::vec;
use alloc::vec::Vec;

use super::order::callee_first;
use super::summarize::summarize;
use super::summary::Summary;
use crate::compiler::diag::{Code, Diagnostic, Diagnostics};
use crate::compiler::tir::{FnId, TProgram};

/** Check the secret flow of `p`, whose function `f` checked when `ok[f]`. */
pub fn check_program(p: &TProgram, ok: &[bool], diags: &mut Diagnostics) {
    let mut summaries: Vec<Option<Summary>> = vec![None; p.fns.len()];
    for f in callee_first(p) {
        let (Some(body), Some(true)) = (p.fns.get(f), ok.get(f).copied()) else {
            continue;
        };
        let is_main = p.main == Some(FnId(f as u32));
        if is_main {
            for param in &body.params {
                let Some(local) = body.locals.get(param.local.0 as usize) else {
                    continue;
                };
                if local.labels.whole().is_none() {
                    let d = Diagnostic::error(
                        Code::MAIN_UNLABELLED,
                        "every parameter of `main` is `public` or `secret`",
                        local.span,
                        "no label",
                    )
                    .with_help("write `public T` for a public input or `secret T` for a witness");
                    diags.push(d);
                }
            }
        }
        let (s, found) = summarize(p, &summaries, body, is_main);
        found.into_iter().for_each(|d| diags.push(d));
        if let Some(slot) = summaries.get_mut(f) {
            *slot = Some(s);
        }
    }
}
