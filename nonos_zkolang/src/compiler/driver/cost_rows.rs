/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Facts of each row of a machine program: the SSA value whose cost it is, and how many
 * registers are live there, those a later row reads before any row writes them again,
 * the row's own reads included.
 */

use alloc::vec;
use alloc::vec::Vec;

use crate::compiler::codegen::Origin;
use crate::compiler::ssa::V;
use crate::isa::Op;

/** The register `op` writes, and those it reads. */
fn regs(op: &Op) -> (Option<u8>, [Option<u8>; 3]) {
    match *op {
        Op::Imm { d, .. } | Op::Inp { d, .. } => (Some(d), [None; 3]),
        Op::Add { d, a, b } | Op::Sub { d, a, b } | Op::Mul { d, a, b } | Op::Eq { d, a, b } => {
            (Some(d), [Some(a), Some(b), None])
        }
        Op::Inv { d, a } => (Some(d), [Some(a), None, None]),
        Op::Sel { d, c, a, b } => (Some(d), [Some(c), Some(a), Some(b)]),
        Op::Bool { a } | Op::Assert { a } | Op::Out { a, .. } => (None, [Some(a), None, None]),
        Op::Halt => (None, [None; 3]),
    }
}

/** How many registers are live at each row of `ops`. */
pub(super) fn live_counts(ops: &[Op]) -> Vec<usize> {
    let mut live = [false; 256];
    let mut out = vec![0; ops.len()];
    for (i, op) in ops.iter().enumerate().rev() {
        let (d, reads) = regs(op);
        if let Some(slot) = d.and_then(|d| live.get_mut(usize::from(d))) {
            *slot = false;
        }
        for r in reads.into_iter().flatten() {
            if let Some(slot) = live.get_mut(usize::from(r)) {
                *slot = true;
            }
        }
        if let Some(n) = out.get_mut(i) {
            *n = live.iter().filter(|&&x| x).count();
        }
    }
    out
}

/**
 * The value whose cost each row is: its own, or for a row that brings a value back into a
 * register, the next row's that needs it; `None` for padding and the final `Halt`.
 */
pub(super) fn owners(origins: &[Origin]) -> Vec<Option<V>> {
    let mut user = None;
    let mut whose = vec![None; origins.len()];
    for (i, origin) in origins.iter().enumerate().rev() {
        user = match origin {
            Origin::Def(v) => Some(*v),
            Origin::Reload(v) => user.or(Some(*v)),
            Origin::Pad | Origin::Halt => None,
        };
        if let Some(w) = whose.get_mut(i) {
            *w = user;
        }
    }
    whose
}
