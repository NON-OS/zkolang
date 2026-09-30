/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Translation validation. The machine program is replayed with each register holding the
 * SSA value it stands for: every instruction must compute the value it claims from
 * registers holding the right operands, every read of the advice must read the value's
 * own slot or a copy already constrained equal to it, and every constraint, output,
 * inverse and selection of the SSA program must appear. What passes enforces exactly the
 * SSA program's constraints, plus copies equal to their values.
 */

use alloc::vec;

use super::machine::{Machine, Origin};
use super::verify_done::{missing, own_slots};
use super::verify_op::Replay;
use crate::compiler::ssa::{Inst, Ssa};
use crate::isa::{Op, REGS};

/** Where a machine program fails validation, and why. */
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct VerifyError {
    pub op: usize,
    pub why: &'static str,
}

/** Check that `m` implements `ssa`. */
pub fn verify(ssa: &Ssa, m: &Machine) -> Result<(), VerifyError> {
    let own = ssa.insts.iter().filter_map(|i| match i {
        Inst::Advice(h) => Some(*h),
        _ => None,
    });
    if !own.eq(m.advice.iter().copied().take(ssa.n_advice())) || m.ops.len() != m.origins.len() {
        return Err(VerifyError {
            op: 0,
            why: "the advice or the origins do not match the program",
        });
    }
    let mut rp = Replay {
        ssa,
        m,
        regs: [None; REGS],
        done: vec![false; ssa.insts.len()],
        copies: vec![false; m.advice.len()],
        spill: None,
        own: own_slots(ssa),
    };
    for (k, (&op, &origin)) in m.ops.iter().zip(&m.origins).enumerate() {
        let fail = |why| VerifyError { op: k, why };
        match origin {
            Origin::Def(v) => rp.def(op, v).map_err(fail)?,
            Origin::Reload(v) => rp.reload(op, v).map_err(fail)?,
            Origin::Spill(v) => rp.spill_step(op, v).map_err(fail)?,
            Origin::Halt if k + 1 == m.ops.len() && matches!(op, Op::Halt) => {}
            Origin::Halt => return Err(fail("a halt that does not end the program")),
        }
    }
    if !matches!(m.ops.last(), Some(Op::Halt)) {
        return Err(VerifyError {
            op: m.ops.len(),
            why: "the program does not end with halt",
        });
    }
    missing(ssa, &rp.done, m.ops.len())
}
