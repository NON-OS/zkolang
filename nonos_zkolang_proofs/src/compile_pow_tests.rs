/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A power with a variable exponent, compiled, against the reference interpreter: bases
 * at the edges of each type and exponents from 0 past the width, so that some powers fit
 * and some overflow, and the machine must accept exactly those the interpreter does.
 */

use crate::compile_run::{compile, fp, run, slots};
use crate::sema_check::{call, checked, int};

#[test]
fn a_variable_power_runs_as_the_interpreter_does() {
    let types: [(&str, u32, i128, i128); 5] = [
        ("u8", 8, 0, 255),
        ("i8", 8, -128, 127),
        ("i16", 16, -32768, 32767),
        ("u32", 32, 0, 4294967295),
        ("i32", 32, -2147483648, 2147483647),
    ];
    let exps = [0i128, 1, 2, 3, 5, 7, 8, 15, 16, 31, 32, 33, 4294967295];
    let mut problems = Vec::new();
    for (name, bits, lo, hi) in types {
        let src =
            format!("fn main(a: public {name}, k: public u32) -> public {name} {{ a.pow(k) }}\n");
        let (program, codes) = checked(&src);
        assert!(codes.is_empty(), "{codes:?}");
        let compiled = compile(&program);
        let bases = [-3, -2, -1, 0, 1, 2, 3, 7, 255, lo, hi];
        for a in bases.into_iter().filter(|a| (lo..=hi).contains(a)) {
            for k in exps {
                let want = call(&program, "main", vec![int(a), int(k)])
                    .ok()
                    .map(|v| slots(v.int(), bits));
                let mut inputs = slots(a, bits);
                inputs.push(fp(k));
                let got = run(&compiled, &inputs, inputs.len());
                if got != want {
                    problems.push(format!("{name} {a}.pow({k}): {want:?} vs {got:?}"));
                }
            }
        }
    }
    assert!(problems.is_empty(), "{problems:#?}");
}
