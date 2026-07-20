/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The native C back-end. A program becomes a self-contained C source file that
//! computes over the Goldilocks field with 128-bit intermediate products, reads its
//! inputs from the command line, and prints its outputs. Compiled with any C
//! compiler it runs as native code, so a zKolang program has a fast path that needs
//! no prover, and its result matches the proven trace exactly.

use alloc::format;
use alloc::string::String;

use super::{n_inputs, n_outputs};
use crate::isa::{Op, REGS};

// The field prelude: Goldilocks add, subtract, multiply, and inverse, each reducing
// a 128-bit intermediate so operands stay canonical, matching the field the VM and
// the AIR use.
const PRELUDE: &str = "\
#include <stdio.h>
#include <stdlib.h>
typedef unsigned long long u64;
typedef unsigned __int128 u128;
static const u64 P = 0xFFFFFFFF00000001ULL;
static u64 fadd(u64 a, u64 b) { return (u64)(((u128)a + (u128)b) % P); }
static u64 fsub(u64 a, u64 b) { return (u64)(((u128)a + (u128)P - (u128)b) % P); }
static u64 fmul(u64 a, u64 b) { return (u64)(((u128)a * (u128)b) % P); }
static u64 finv(u64 a) {
    u64 r = 1, b = a, e = P - 2;
    while (e) { if (e & 1) r = fmul(r, b); b = fmul(b, b); e >>= 1; }
    return r;
}
";

/// Emit a program as a standalone C source file.
pub fn to_c(program: &[Op]) -> String {
    let n_in = n_inputs(program);
    let n_out = n_outputs(program);

    let mut s = String::from(PRELUDE);
    s.push_str("\nint main(int argc, char **argv) {\n");
    s.push_str(&format!("    u64 r[{REGS}] = {{0}};\n"));
    if n_in > 0 {
        s.push_str(&format!("    u64 in[{n_in}] = {{0}};\n"));
        s.push_str(&format!(
            "    for (int i = 0; i < {n_in}; i++) in[i] = (i + 1 < argc) ? (strtoull(argv[i + 1], 0, 10) % P) : 0;\n"
        ));
    }
    if n_out > 0 {
        s.push_str(&format!("    u64 out[{n_out}] = {{0}};\n"));
    }

    for op in program {
        s.push_str("    ");
        s.push_str(&emit_op(op));
        s.push('\n');
    }

    if n_out > 0 {
        s.push_str(&format!(
            "    for (int i = 0; i < {n_out}; i++) printf(\"%llu \", (unsigned long long)out[i]);\n"
        ));
    }
    s.push_str("    printf(\"\\n\");\n    return 0;\n}\n");
    s
}

// One opcode as a C statement over the register file. A constraint opcode becomes a
// guard that returns a nonzero code, so an emitted program fails exactly where the
// proof would be unprovable.
fn emit_op(op: &Op) -> String {
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
        // The program ends here, but the outputs are printed after the op loop, so
        // halt is a marker rather than an early return.
        Op::Halt => String::from("/* halt */"),
    }
}
