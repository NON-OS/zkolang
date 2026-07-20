/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The Python back-end. A program becomes a Python module with a `run(inputs)`
//! function that returns the outputs, computing over the Goldilocks field with
//! Python's arbitrary-precision integers. This lets a zKolang program be called from
//! Python directly, so the language reaches that ecosystem without a prover, and the
//! result matches the proven trace.

use alloc::format;
use alloc::string::String;

use super::{n_inputs, n_outputs};
use crate::isa::{Op, REGS};

// The field prelude: the modulus and the operations, with inverse by Fermat through
// Python's built-in modular exponentiation.
const PRELUDE: &str = "\
P = 0xFFFFFFFF00000001


def _add(a, b):
    return (a + b) % P


def _sub(a, b):
    return (a - b) % P


def _mul(a, b):
    return (a * b) % P


def _inv(a):
    return pow(a, P - 2, P)
";

/// Emit a program as a Python module exposing `run(inputs)`.
pub fn to_python(program: &[Op]) -> String {
    let n_in = n_inputs(program);
    let n_out = n_outputs(program);

    let mut s = String::from(PRELUDE);
    s.push_str("\n\ndef run(inputs):\n");
    s.push_str(&format!("    r = [0] * {REGS}\n"));
    if n_in > 0 {
        s.push_str(&format!(
            "    inp = [(inputs[i] % P) if i < len(inputs) else 0 for i in range({n_in})]\n"
        ));
    }
    if n_out > 0 {
        s.push_str(&format!("    out = [0] * {n_out}\n"));
    }

    for op in program {
        if let Some(line) = emit_op(op) {
            s.push_str("    ");
            s.push_str(&line);
            s.push('\n');
        }
    }

    if n_out > 0 {
        s.push_str("    return out\n");
    } else {
        s.push_str("    return []\n");
    }
    s
}

// One opcode as a Python statement. A constraint opcode raises on violation, so the
// emitted program fails exactly where the proof would be unprovable. Halt has no
// statement, since the outputs are returned after the loop.
fn emit_op(op: &Op) -> Option<String> {
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
