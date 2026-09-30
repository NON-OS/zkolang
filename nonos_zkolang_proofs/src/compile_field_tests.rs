/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Compiled `field` arithmetic against the model: random expressions over two elements,
 * compiled as `main` and run on the VM, must give the model's value on every input the
 * model accepts, and be rejected on every input it rejects: a zero divisor or inverse.
 */

use nonos_stark::field::Fp;

use crate::compile_run::{compile, run};
use crate::prop_field::eval;
use crate::prop_field_gen::{expr, print, value};
use crate::prop_gen::Rng;
use crate::sema_check::checked;

#[test]
fn compiled_field_programs_run_as_the_model_says() {
    let mut r = Rng(0x5EED_F1E1_D000_0001);
    let mut problems = Vec::new();
    for _ in 0..150 {
        let e = expr(&mut r, 4);
        let src = format!(
            "fn main(a: public field, b: public field) -> public field {{ let z: field = 0; {} }}",
            print(&e)
        );
        let (program, codes) = checked(&src);
        if codes.iter().any(|c| c.starts_with('E')) {
            problems.push(format!("{src} does not check: {codes:?}"));
            continue;
        }
        let compiled = compile(&program);
        for _ in 0..6 {
            let args = (value(&mut r), value(&mut r));
            let want = eval(&e, args).ok().map(|v| vec![Fp::from_u64(v as u64)]);
            let got = run(&compiled, &[Fp::from_u64(args.0), Fp::from_u64(args.1)], 2);
            if got != want {
                problems.push(format!(
                    "{src} on {args:?}: model {want:?}, compiled {got:?}"
                ));
            }
        }
    }
    let shown: Vec<&String> = problems.iter().take(4).collect();
    assert!(
        problems.is_empty(),
        "{} disagreements, first {shown:#?}",
        problems.len()
    );
}
