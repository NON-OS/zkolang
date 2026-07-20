/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! One opcode as a Python statement. A constraint opcode raises on violation. Halt
//! has no statement, since the outputs are returned after the loop.

use alloc::format;
use alloc::string::String;

use crate::isa::Op;

pub(super) fn emit_op(op: &Op) -> Option<String> {
    let line = match op {
        Op::Imm { d, v } => format!("r[{d}] = {}", v.value()),
        Op::Add { d, a, b } => format!("r[{d}] = _add(r[{a}], r[{b}])"),
        Op::Sub { d, a, b } => format!("r[{d}] = _sub(r[{a}], r[{b}])"),
        Op::Mul { d, a, b } => format!("r[{d}] = _mul(r[{a}], r[{b}])"),
        Op::Inv { d, a } => format!("r[{d}] = _inv(r[{a}])"),
        Op::Sel { d, c, a, b } => format!("r[{d}] = r[{a}] if r[{c}] else r[{b}]"),
        Op::Eq { d, a, b } => format!("r[{d}] = 1 if r[{a}] == r[{b}] else 0"),
        Op::Bool { a } => format!("assert r[{a}] in (0, 1)"),
        Op::Assert { a } => format!("assert r[{a}] == 0"),
        Op::Inp { d, idx } => format!("r[{d}] = inp[{idx}]"),
        Op::Out { a, idx } => format!("out[{idx}] = r[{a}]"),
        Op::Halt => return None,
    };
    Some(line)
}
