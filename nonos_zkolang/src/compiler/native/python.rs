/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A built program as a Python module exposing `run(values)`. */

use alloc::format;
use alloc::string::String;

use super::python_inst::inst;
use super::python_io::{input, output};
use super::python_prelude::{EXTRA, MAIN};
use crate::backend::PYTHON_PRELUDE;
use crate::compiler::driver::Built;

/**
 * `b` as a Python module exposing `run(values)`. It takes one integer per leaf of
 * `main`'s inputs, the public ones then the secret ones, as `zkolang run` takes them, and
 * returns the leaves of the result. It raises `ValueError` for other inputs and
 * `Unprovable` when a constraint fails, which is when no proof of the run exists. Run as
 * a script, it takes the same command line as the C target and exits as it does.
 */
pub fn to_python(b: &Built) -> String {
    let ssa = &b.compiled.ssa;
    let leaves: alloc::vec::Vec<_> = b.public.iter().chain(&b.secret).copied().collect();
    let mut s = String::from(PYTHON_PRELUDE);
    s.push_str(EXTRA);
    s.push_str("\n\ndef run(values):\n");
    s.push_str(&format!(
        "    if len(values) != {0}:\n        raise ValueError(\"expected {0} inputs\")\n",
        leaves.len()
    ));
    s.push_str(&format!(
        "    inp, out, result = [], [0] * {}, []\n",
        ssa.n_outputs
    ));
    for (i, leaf) in leaves.iter().enumerate() {
        s.push_str(&format!("    {}\n", input(i, *leaf)));
    }
    for (i, x) in ssa.insts.iter().enumerate() {
        s.push_str(&format!("    {}\n", inst(i, *x)));
    }
    s.push_str("    s = 0\n");
    for leaf in &b.output {
        s.push_str(&format!("    {}\n", output(*leaf)));
    }
    s.push_str("    return result\n");
    s.push_str(MAIN);
    s
}
