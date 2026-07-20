/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Property-based fuzzing of the optimizer. Thousands of random arithmetic programs are
//! generated, each compiled with the optimizer and without, run on random inputs, and
//! required to agree. Where the curated equivalence test covers the shapes a bug is known
//! to hide in, this covers the shapes nobody thought of. The generated grammar uses only
//! add, subtract, multiply, bindings, and small constants, so every program always runs
//! and stays inside the register file, which isolates the optimizer as the only variable.

use nonos_zkolang::{compile_source, compile_source_unoptimized, evaluate};

// A small deterministic generator, so a failure reproduces from the seed.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0 >> 33
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

fn gen_expr(rng: &mut Rng, depth: u32, vars: &[String]) -> String {
    if depth == 0 || rng.below(3) == 0 {
        if rng.below(2) == 0 {
            vars[rng.below(vars.len() as u64) as usize].clone()
        } else {
            rng.below(10).to_string()
        }
    } else {
        let op = ["+", "-", "*"][rng.below(3) as usize];
        let a = gen_expr(rng, depth - 1, vars);
        let b = gen_expr(rng, depth - 1, vars);
        format!("({a} {op} {b})")
    }
}

fn gen_program(rng: &mut Rng) -> (String, Vec<u64>) {
    let mut src = String::from("input a;\ninput b;\ninput c;\n");
    let mut vars: Vec<String> = vec!["a".into(), "b".into(), "c".into()];
    let lets = rng.below(4);
    for i in 0..lets {
        let name = format!("v{i}");
        let e = gen_expr(rng, 3, &vars);
        src.push_str(&format!("let {name} = {e};\n"));
        vars.push(name);
    }
    src.push_str(&format!("output {};\n", gen_expr(rng, 4, &vars)));
    let inputs = vec![rng.below(100), rng.below(100), rng.below(100)];
    (src, inputs)
}

#[test]
fn the_optimizer_is_equivalent_under_fuzzing() {
    let mut rng = Rng(0x0DDC0FFEE);
    for _ in 0..3000 {
        let (src, inputs) = gen_program(&mut rng);
        let opt = compile_source(&src).unwrap_or_else(|e| panic!("optimized:\n{src}\n{e:?}"));
        let raw = compile_source_unoptimized(&src).unwrap_or_else(|e| panic!("raw:\n{src}\n{e:?}"));
        let a = evaluate(&opt, &inputs, &[]).unwrap_or_else(|e| panic!("run opt:\n{src}\n{e:?}"));
        let b = evaluate(&raw, &inputs, &[]).unwrap_or_else(|e| panic!("run raw:\n{src}\n{e:?}"));
        assert_eq!(
            a, b,
            "the optimizer changed the output of:\n{src}\ninputs {inputs:?}"
        );
    }
}
