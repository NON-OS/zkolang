/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The compiler against the reference interpreter: random integer expressions of every
 * type, written as `main`, compiled to machine code and run on the VM, must accept exactly
 * the inputs the interpreter accepts, with the same result.
 */

use crate::compile_run::{compile, fp, run, slots};
use crate::prop_gen::Rng;
use crate::prop_gen_expr::expr;
use crate::prop_model::TYPES;
use crate::prop_print::print;
use crate::sema_check::{call, checked, int};

#[test]
fn compiled_integer_programs_run_as_the_interpreter_does() {
    let mut r = Rng(0xC0FF_EE15_600D_BEEF);
    let mut problems = Vec::new();
    for t in TYPES.iter() {
        for _ in 0..80 {
            let e = expr(&mut r, *t, 3);
            let body = print(&e, *t);
            let src = format!(
                "fn main(a: public {0}, b: public {0}, k: public u32) -> public {0} {{ {body} }}\n",
                t.name
            );
            let (program, codes) = checked(&src);
            if codes.iter().any(|c| c.starts_with('E')) {
                problems.push(format!("{src}does not check: {codes:?}"));
                continue;
            }
            let compiled = compile(&program);
            for _ in 0..6 {
                let args = (
                    r.value(*t),
                    r.value(*t),
                    r.below(u64::from(t.bits) + 2) as i128,
                );
                let argv = vec![int(args.0), int(args.1), int(args.2)];
                let want = call(&program, "main", argv)
                    .ok()
                    .map(|v| slots(v.int(), t.bits));
                let mut inputs = slots(args.0, t.bits);
                inputs.extend(slots(args.1, t.bits));
                inputs.push(fp(args.2));
                let got = run(&compiled, &inputs, inputs.len());
                if got != want {
                    problems.push(format!(
                        "{src}on {args:?}: interpreter {want:?}, compiled {got:?}"
                    ));
                }
            }
        }
    }
    let shown: Vec<&String> = problems.iter().take(6).collect();
    assert!(
        problems.is_empty(),
        "{} disagreements, first {shown:#?}",
        problems.len()
    );
}
