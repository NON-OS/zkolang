/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The machine instruction of an SSA instruction, given where its operands are. */

use nonos_stark::field::Fp;

use crate::compiler::ssa::{Inst, V};
use crate::isa::Op;

/**
 * The instruction computing `inst` into `d`, its operands in the registers `r` gives,
 * reading advice at `advice`; `None` for a gadget.
 */
pub(super) fn op_of(inst: Inst, d: u8, r: &dyn Fn(V) -> u8, advice: u16) -> Option<Op> {
    Some(match inst {
        Inst::Const(c) => Op::Imm {
            d,
            v: Fp::from_u64(c),
        },
        Inst::Input(idx) => Op::Inp { d, idx },
        Inst::Advice(_) => Op::Inp { d, idx: advice },
        Inst::Add(a, b) => Op::Add {
            d,
            a: r(a),
            b: r(b),
        },
        Inst::Sub(a, b) => Op::Sub {
            d,
            a: r(a),
            b: r(b),
        },
        Inst::Mul(a, b) => Op::Mul {
            d,
            a: r(a),
            b: r(b),
        },
        Inst::Inv(a) => Op::Inv { d, a: r(a) },
        Inst::Sel(c, a, b) => Op::Sel {
            d,
            c: r(c),
            a: r(a),
            b: r(b),
        },
        Inst::Eq(a, b) => Op::Eq {
            d,
            a: r(a),
            b: r(b),
        },
        Inst::AssertBool(a) => Op::Bool { a: r(a) },
        Inst::AssertZero(a) => Op::Assert { a: r(a) },
        Inst::Output(idx, a) => Op::Out { a: r(a), idx },
        _ => return None,
    })
}
