/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The back end against the SSA semantics: for random programs and inputs, the machine
 * program, run by the VM on the witness the hints find, accepts exactly when the SSA
 * program does, with the same outputs; and changing the advice makes the VM reject, since
 * every advice value is pinned (section 21.3).
 */

use nonos_zkolang::compiler::driver::{backend, witness};
use nonos_zkolang::compiler::ssa::eval::eval;
use nonos_zkolang::Vm;

use crate::prop_gen::Rng;
use crate::ssa_gen::program;

#[test]
fn the_machine_program_runs_as_the_ssa_program() {
    let mut r = Rng(0x0DDB_1A5E_5BAD_5EED);
    let (mut accepted, mut spilled, mut problems) = (0, 0, Vec::new());
    for case in 0..300 {
        let ssa = program(&mut r, 20 + (case % 7) * 15);
        let compiled = backend(&ssa).unwrap_or_else(|e| panic!("case {case}: {e:?}"));
        if compiled.machine.advice.len() > compiled.ssa.n_advice() {
            spilled += 1;
        }
        for run in 0..6 {
            let inputs = crate::ssa_gen::inputs(&mut r);
            let want = eval(&ssa, &inputs).map(|run| run.outputs);
            let full = witness(&compiled, &inputs);
            let got = full.as_ref().ok().map(|full| {
                Vm::new()
                    .run(&compiled.machine.ops, full, 4)
                    .map(|t| t.public_outputs)
            });
            match (&want, got) {
                (Ok(outs), Some(Ok(t))) if *outs == t => accepted += 1,
                (Err(_), None) => {}
                (w, g) => problems.push(format!("case {case} on {inputs:?}: ssa {w:?}, vm {g:?}")),
            }
            if let (Ok(_), Ok(full), 0) = (&want, &full, run) {
                problems.extend(crate::ssa_mutate::unpinned(
                    &compiled.machine.ops,
                    full,
                    4,
                    case,
                ));
            }
        }
    }
    let shown: Vec<&String> = problems.iter().take(6).collect();
    assert!(
        problems.is_empty(),
        "{} problems, first {shown:#?}",
        problems.len()
    );
    assert!(accepted > 600, "only {accepted} runs were accepted");
    assert!(spilled > 3, "only {spilled} programs needed a spill");
}
