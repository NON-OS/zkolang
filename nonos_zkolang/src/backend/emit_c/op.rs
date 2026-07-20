/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! One opcode as a C statement over the register file. A constraint opcode becomes a
//! guard that returns a nonzero code, so an emitted program fails exactly where the
//! proof would be unprovable.

use alloc::format;
use alloc::string::String;

use crate::isa::Op;

pub(super) fn emit_op(op: &Op) -> String {
    match op {
        Op::Imm { d, v } => format!("r[{d}] = {}ULL;", v.value()),
        Op::Add { d, a, b } => format!("r[{d}] = fadd(r[{a}], r[{b}]);"),
        Op::Sub { d, a, b } => format!("r[{d}] = fsub(r[{a}], r[{b}]);"),
        Op::Mul { d, a, b } => format!("r[{d}] = fmul(r[{a}], r[{b}]);"),
        Op::Inv { d, a } => format!("r[{d}] = finv(r[{a}]);"),
        Op::Sel { d, c, a, b } => format!("r[{d}] = r[{c}] ? r[{a}] : r[{b}];"),
        Op::Eq { d, a, b } => format!("r[{d}] = (r[{a}] == r[{b}]) ? 1ULL : 0ULL;"),
        Op::Bool { a } => format!("if (r[{a}] != 0ULL && r[{a}] != 1ULL) return 2;"),
        Op::Assert { a } => format!("if (r[{a}] != 0ULL) return 3;"),
        Op::Inp { d, idx } => format!("r[{d}] = in[{idx}];"),
        Op::Out { a, idx } => format!("out[{idx}] = r[{a}];"),
        Op::Halt => String::from("/* halt */"),
    }
}
