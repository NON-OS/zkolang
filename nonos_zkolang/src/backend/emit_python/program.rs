/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Emit a whole program as a Python module exposing `run(inputs)`.

use alloc::format;
use alloc::string::String;

use super::op::emit_op;
use super::prelude::PRELUDE;
use crate::backend::{n_inputs, n_outputs};
use crate::isa::{Op, REGS};

/**
 * Emit a program as a Python module exposing `run(inputs)`. It takes exactly one
 * integer per input, each below the field modulus, and raises `ValueError` otherwise.
 */
pub fn to_python(program: &[Op]) -> String {
    let n_in = n_inputs(program);
    let n_out = n_outputs(program);

    let mut s = String::from(PRELUDE);
    s.push_str("\n\ndef run(inputs):\n");
    s.push_str(&format!("    r = [0] * {REGS}\n"));
    s.push_str(&format!("    if len(inputs) != {n_in}:\n"));
    s.push_str(&format!(
        "        raise ValueError(\"expected {n_in} inputs\")\n"
    ));
    if n_in > 0 {
        s.push_str("    inp = [_input(i, v) for i, v in enumerate(inputs)]\n");
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
    s.push_str(if n_out > 0 {
        "    return out\n"
    } else {
        "    return []\n"
    });
    s
}
