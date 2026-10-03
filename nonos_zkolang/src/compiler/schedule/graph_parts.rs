/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The parts of the scheduling graph, each worked out from the program. */

use alloc::vec;
use alloc::vec::Vec;

use crate::compiler::ssa::{Inst, Ssa, V};

/** Each instruction's dependences, how many of them it reads, and each value's readers. */
pub(super) fn edges(ssa: &Ssa) -> (Vec<Vec<usize>>, Vec<usize>, Vec<Vec<usize>>) {
    let n = ssa.insts.len();
    let index = |v: V| Some(v.index()).filter(|&d| d < n);
    let mut users = vec![Vec::new(); n];
    let (mut deps, mut reads) = (Vec::with_capacity(n), Vec::with_capacity(n));
    for (i, inst) in ssa.insts.iter().enumerate() {
        let own: Vec<usize> = inst.operands().filter_map(index).collect();
        for &d in &own {
            /* An instruction reading a value twice is one of its readers. */
            if users[d].last() != Some(&i) {
                users[d].push(i);
            }
        }
        reads.push(own.len());
        let hint = inst.hint_operands().filter_map(index);
        deps.push(own.into_iter().chain(hint).collect());
    }
    (deps, reads, users)
}

/** The last of `sinks` that needs each instruction, directly or not. */
pub(super) fn last_sinks(deps: &[Vec<usize>], sinks: &[usize]) -> Vec<usize> {
    let mut last = vec![0usize; deps.len()];
    sinks.iter().enumerate().for_each(|(j, &s)| last[s] = j);
    for i in (0..deps.len()).rev() {
        for &d in &deps[i] {
            last[d] = last[d].max(last[i]);
        }
    }
    last
}

/** How many registers evaluating each operation takes, counted as for a tree. */
pub(super) fn needs(deps: &[Vec<usize>], op: &[bool]) -> Vec<u32> {
    let mut need = vec![1u32; deps.len()];
    for i in (0..deps.len()).filter(|&i| op[i]) {
        let mut ns: Vec<u32> = deps[i].iter().map(|&d| need[d]).collect();
        ns.sort_unstable_by(|a, b| b.cmp(a));
        let k = ns
            .iter()
            .enumerate()
            .map(|(j, &x)| x.saturating_add(j as u32));
        need[i] = k.max().unwrap_or(1).min(1 << 20);
    }
    need
}

/** Whether `i` is read from the inputs or the advice, or written as a constant. */
pub(super) fn is_read(i: &Inst) -> bool {
    matches!(i, Inst::Const(_) | Inst::Input(_) | Inst::Advice(_))
}

/** Whether `i` constrains its operands even when nothing reads its value. */
pub(super) fn needed(i: &Inst) -> bool {
    matches!(i, Inst::Inv(_) | Inst::Sel(..))
}
