/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The order the functions are summarized in: every function after the functions it calls,
 * found without recursion on the host stack. Functions in a cycle, which is reported
 * elsewhere, come in the order the walk meets them.
 */

use alloc::vec;
use alloc::vec::Vec;

use crate::compiler::tir::{TExpr, TExprKind, TProgram};

/** The functions of `p`, each after those it calls. */
pub(super) fn callee_first(p: &TProgram) -> Vec<usize> {
    let n = p.fns.len();
    let calls: Vec<Vec<usize>> = p
        .fns
        .iter()
        .map(|f| {
            let mut out = Vec::new();
            f.body.each_expr(&mut |e| collect(e, &mut out));
            out.retain(|&g| g < n);
            out
        })
        .collect();
    let (mut state, mut order) = (vec![0u8; n], Vec::with_capacity(n));
    for root in 0..n {
        if state[root] != 0 {
            continue;
        }
        let mut work = vec![(root, 0usize)];
        state[root] = 1;
        while let Some((v, i)) = work.last_mut() {
            let v = *v;
            match calls[v].get(*i) {
                Some(&w) => {
                    *i += 1;
                    if state[w] == 0 {
                        state[w] = 1;
                        work.push((w, 0));
                    }
                }
                None => {
                    state[v] = 2;
                    order.push(v);
                    work.pop();
                }
            }
        }
    }
    order
}

/** The functions `e` calls, anywhere in it. */
fn collect(e: &TExpr, out: &mut Vec<usize>) {
    if let TExprKind::Call(f, _) = &e.kind {
        out.push(f.0 as usize);
    }
    e.each_child(&mut |c| collect(c, out));
}
