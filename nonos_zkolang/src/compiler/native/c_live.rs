/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Which values the C target declares: those an output, a check that can fail the run, or
 * another declared value reads. A C compiler warns of a local nothing reads, and a value
 * the run never looks at has nothing to compute.
 */

use alloc::vec;
use alloc::vec::Vec;

use super::c_inst::fails;
use crate::compiler::ssa::Inst;

/** For each instruction of `insts`, whether the C target declares its value. */
pub(super) fn live(insts: &[Inst]) -> Vec<bool> {
    let mut live = vec![false; insts.len()];
    for (i, x) in insts.iter().enumerate().rev() {
        let mark = |live: &mut Vec<bool>, x: Inst| {
            x.operands()
                .chain(x.hint_operands())
                .for_each(|o| live[o.0 as usize] = true)
        };
        match *x {
            _ if live[i] || matches!(x, Inst::Output(..)) => mark(&mut live, *x),
            Inst::Sel(c, _, _) => live[c.0 as usize] = true,
            _ if fails(*x) => mark(&mut live, *x),
            _ => {}
        }
    }
    live
}
