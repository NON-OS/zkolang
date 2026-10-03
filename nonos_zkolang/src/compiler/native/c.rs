/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A built program as a standalone C source file. */

use alloc::format;
use alloc::string::String;

use super::c_inst::inst;
use super::c_io::{input, output};
use super::c_live::live;
use super::c_prelude::{EXTRA, IO};
use crate::backend::C_PRELUDE;
use crate::compiler::driver::Built;

/**
 * `b` as a C source file that runs as native code. It takes one argument per leaf of
 * `main`'s inputs, the public ones then the secret ones, as `zkolang run` takes them, and
 * prints the leaves of its result on one line. It returns 1 for any other command line
 * and 3 when a constraint fails, which is when no proof of the run exists.
 */
pub fn to_c(b: &Built) -> String {
    let ssa = &b.compiled.ssa;
    let leaves: alloc::vec::Vec<_> = b.public.iter().chain(&b.secret).copied().collect();
    let mut s = String::from(C_PRELUDE);
    s.push_str(EXTRA);
    s.push_str(IO);
    s.push_str("\nint main(int argc, char **argv) {\n");
    s.push_str(&format!(
        "    if (argc != {}) {{ fprintf(stderr, \"expected {} inputs\\n\"); return 1; }}\n",
        leaves.len() + 1,
        leaves.len()
    ));
    s.push_str(&format!(
        "    u64 in[{}] = {{0}}, out[{}] = {{0}};\n    int s = 0;\n",
        ssa.n_inputs().max(1),
        usize::from(ssa.n_outputs).max(1)
    ));
    for (i, leaf) in leaves.iter().enumerate() {
        s.push_str(&format!("    {}\n", input(i, *leaf)));
    }
    let live = live(&ssa.insts);
    for (i, x) in ssa.insts.iter().enumerate() {
        for line in inst(i, *x, live[i]) {
            s.push_str(&format!("    {line}\n"));
        }
    }
    s.push_str("    s = 0;\n");
    for leaf in &b.output {
        s.push_str(&format!("    {}\n", output(*leaf)));
    }
    s.push_str("    printf(\"\\n\");\n    return 0;\n}\n");
    s
}
