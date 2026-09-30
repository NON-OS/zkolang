/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Translation validation. The machine program is replayed with each register holding the
 * SSA value it stands for: every instruction must compute the value it claims from
 * registers holding the right operands, every read of an input or advice value must read
 * its own slot, no secret input or advice slot may be read twice, and every constraint,
 * output, inverse and selection of the SSA program must appear. What passes enforces
 * exactly the SSA program's constraints.
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
    let layout = m.n_inputs == ssa.n_inputs() && m.advice.len() == ssa.n_advice();
    if !layout || !own.eq(m.advice.iter().copied()) || m.ops.len() != m.origins.len() {
        return Err(VerifyError {
            op: 0,
            why: "the inputs, advice or origins do not match the program",
        });
    }
    let mut rp = Replay {
        ssa,
        m,
        regs: [None; REGS],
        done: vec![false; ssa.insts.len()],
        read: vec![false; m.n_inputs + m.advice.len()],
        own: own_slots(ssa),
    };
    for (k, (&op, &origin)) in m.ops.iter().zip(&m.origins).enumerate() {
        let fail = |why| VerifyError { op: k, why };
        match origin {
            Origin::Def(v) => rp.def(op, v).map_err(fail)?,
            Origin::Reload(v) => rp.reload(op, v).map_err(fail)?,
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
