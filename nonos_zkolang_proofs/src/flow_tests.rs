/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Compiled control flow against the interpreter: random programs with branches, loops,
 * early exits, run-time indices and `&mut` calls, compiled to machine code and run on the
 * VM, must accept exactly the inputs the interpreter accepts, with the same result.
 */

use nonos_zkolang::compiler::interp::Value;

use crate::compile_run::{fp, run, try_compile};
use crate::flow_gen::program;
use crate::prop_gen::Rng;
use crate::sema_check::{call, checked, int};

#[test]
fn compiled_control_flow_runs_as_the_interpreter_does() {
    let mut r = Rng(0xF10E_5EED_0BAD_CAFE);
    let (mut problems, mut accepted, mut large) = (Vec::new(), 0, 0);
    for _ in 0..150 {
        let src = program(&mut r);
        let (p, codes) = checked(&src);
        if codes.iter().any(|c| c.starts_with('E')) {
            problems.push(format!("{src}does not check: {codes:?}"));
            continue;
        }
        let compiled = match try_compile(&p) {
            Ok(c) => c,
            Err(e) if e.contains("TooManySlots") || e.contains("TooLarge") => {
                large += 1;
                continue;
            }
            Err(e) => panic!("{src}{e}"),
        };
        for _ in 0..5 {
            let vals: Vec<i128> = (0..6)
                .map(|_| {
                    let wide = r.below(3) == 0;
                    i128::from(r.below(if wide { 1 << 32 } else { 12 }))
                })
                .collect();
            let c = Value::Array(vals[2..].iter().map(|&v| int(v)).collect());
            let want = call(&p, "main", vec![int(vals[0]), int(vals[1]), c])
                .ok()
                .map(|v| vec![fp(v.int())]);
            let inputs: Vec<_> = vals.iter().map(|&v| fp(v)).collect();
            let got = run(&compiled, &inputs, inputs.len());
            accepted += usize::from(got.is_some());
            if got != want {
                problems.push(format!(
                    "{src}on {vals:?}: interpreter {want:?}, compiled {got:?}"
                ));
            }
        }
    }
    let shown: Vec<&String> = problems.iter().take(3).collect();
    assert!(
        problems.is_empty(),
        "{} disagreements, first {shown:#?}",
        problems.len()
    );
    assert!(accepted > 300, "only {accepted} runs were accepted");
    assert!(large < 8, "{large} programs were too large to compile");
}
