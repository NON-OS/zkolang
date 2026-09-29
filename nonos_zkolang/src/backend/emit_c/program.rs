/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Emit a whole program as a standalone C source file.

use alloc::format;
use alloc::string::String;

use super::op::emit_op;
use super::prelude::PRELUDE;
use crate::backend::{n_inputs, n_outputs};
use crate::isa::{Op, REGS};

/** The C statement for an argument that is not a decimal number below the modulus. */
const BAD_INPUT: &str =
    "fprintf(stderr, \"input %d is not a decimal number below the field modulus\\n\", i); return 1;";

/**
 * Emit a program as a standalone C source file that runs as native code. It takes
 * exactly one argument per input, each a decimal number below the field modulus, and
 * returns 1 for any other command line.
 */
pub fn to_c(program: &[Op]) -> String {
    let n_in = n_inputs(program);
    let n_out = n_outputs(program);

    let mut s = String::from(PRELUDE);
    s.push_str("\nint main(int argc, char **argv) {\n");
    s.push_str(&format!("    u64 r[{REGS}] = {{0}};\n"));
    s.push_str(&format!(
        "    if (argc != {}) {{ fprintf(stderr, \"expected {n_in} inputs\\n\"); return 1; }}\n",
        n_in + 1
    ));
    if n_in > 0 {
        s.push_str(&format!("    u64 in[{n_in}] = {{0}};\n"));
        s.push_str(&format!("    for (int i = 0; i < {n_in}; i++) if (!read_input(argv[i + 1], &in[i])) {{ {BAD_INPUT} }}\n"));
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
        s.push_str(&format!("    for (int i = 0; i < {n_out}; i++) printf(\"%llu \", (unsigned long long)out[i]);\n"));
    }
    s.push_str("    printf(\"\\n\");\n    return 0;\n}\n");
    s
}
