/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * One opcode as a Python statement. Wherever the proof would be unprovable the module
 * raises `Unprovable`. Halt has no statement, since the outputs are returned after the
 * loop.
 */

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
        Op::Sel { d, c, a, b } => format!("r[{d}] = _sel(r[{c}], r[{a}], r[{b}])"),
        Op::Eq { d, a, b } => format!("r[{d}] = 1 if r[{a}] == r[{b}] else 0"),
        Op::Bool { a } => format!("_check(r[{a}] in (0, 1), \"boolean constraint\")"),
        Op::Assert { a } => format!("_check(r[{a}] == 0, \"assertion\")"),
        Op::Inp { d, idx } => format!("r[{d}] = inp[{idx}]"),
        Op::Out { a, idx } => format!("out[{idx}] = r[{a}]"),
        Op::Halt => return None,
    };
    Some(line)
}
