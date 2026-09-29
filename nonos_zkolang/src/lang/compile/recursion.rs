/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Find a function that calls itself, directly or through others. Functions inline at
 * every call, so a recursive one never ends; it used to surface as a register or depth
 * error far from its cause, and not at all while nothing called it.
 */

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use super::callees::callees;
use crate::lang::parse::FnDef;

/** A function on a cycle of calls, if there is one. */
pub(super) fn recursive(fns: &[FnDef]) -> Option<&String> {
    let mut index: BTreeMap<&str, usize> = BTreeMap::new();
    for (i, f) in fns.iter().enumerate() {
        index.entry(f.name.as_str()).or_insert(i);
    }
    let calls: Vec<Vec<usize>> = fns
        .iter()
        .map(|f| {
            let mut out = Vec::new();
            callees(&f.body, &index, &mut out);
            out
        })
        .collect();
    /* A depth-first walk on an explicit stack: 0 unseen, 1 on the path, 2 finished. */
    let mut state = vec![0u8; fns.len()];
    for root in 0..fns.len() {
        if state[root] != 0 {
            continue;
        }
        state[root] = 1;
        let mut path = vec![(root, 0usize)];
        while let Some(top) = path.last_mut() {
            let (f, next) = *top;
            let Some(&g) = calls[f].get(next) else {
                state[f] = 2;
                path.pop();
                continue;
            };
            top.1 += 1;
            match state[g] {
                0 => {
                    state[g] = 1;
                    path.push((g, 0));
                }
                1 => return Some(&fns[g].name),
                _ => {}
            }
        }
    }
    None
}
