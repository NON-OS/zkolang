/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The cycles of the call graph: its strongly connected parts that hold one, found by
 * Tarjan's method without recursion on the host stack.
 */

use alloc::vec;
use alloc::vec::Vec;

use crate::compiler::source::Span;
use crate::compiler::tir::FnId;

/** The strongly connected parts of `graph` that hold a cycle, by Tarjan's method. */
pub(super) fn cycles(graph: &[Vec<(FnId, Span)>]) -> Vec<Vec<usize>> {
    let n = graph.len();
    let (mut index, mut low, mut on) = (vec![usize::MAX; n], vec![0; n], vec![false; n]);
    let (mut stack, mut out, mut next) = (Vec::new(), Vec::new(), 0);
    for root in 0..n {
        if index[root] != usize::MAX {
            continue;
        }
        let mut work = vec![(root, 0usize)];
        while let Some(&mut (v, ref mut i)) = work.last_mut() {
            if *i == 0 {
                index[v] = next;
                low[v] = next;
                next += 1;
                stack.push(v);
                on[v] = true;
            }
            if let Some(&(w, _)) = graph[v].get(*i) {
                *i += 1;
                let w = w.0 as usize;
                if w >= n {
                    continue;
                }
                if index[w] == usize::MAX {
                    work.push((w, 0));
                } else if on[w] {
                    low[v] = low[v].min(index[w]);
                }
                continue;
            }
            work.pop();
            if let Some(&(parent, _)) = work.last() {
                low[parent] = low[parent].min(low[v]);
            }
            if low[v] == index[v] {
                let mut part = Vec::new();
                while let Some(w) = stack.pop() {
                    on[w] = false;
                    part.push(w);
                    if w == v {
                        break;
                    }
                }
                let self_loop = graph[v].iter().any(|(g, _)| g.0 as usize == v);
                if part.len() > 1 || self_loop {
                    part.reverse();
                    out.push(part);
                }
            }
        }
    }
    out
}
