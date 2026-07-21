/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Property-based fuzzing of the optimizer and the native backend. Thousands of random
//! arithmetic programs are generated; each is compiled with the optimizer and without, run
//! on random inputs, and required to agree, and a sample is also emitted as C and as
//! assembly, built, and required to match the VM. Where the curated tests cover the shapes a
//! bug is known to hide
//! in, this covers the shapes nobody thought of. The generated grammar uses only add,
//! subtract, multiply, bindings, and small constants, so every program always runs and stays
//! inside the register file, which isolates the transform under test as the only variable.

use nonos_zkolang::{compile_source, compile_source_unoptimized, evaluate, to_asm, to_c};

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

// A bit-valued expression, an equality test, which the VM computes with no witness and
// which is a valid select condition.
fn gen_bit(rng: &mut Rng, depth: u32, vars: &[String]) -> String {
    let op = ["==", "!="][rng.below(2) as usize];
    let a = gen_expr(rng, depth, vars);
    let b = gen_expr(rng, depth, vars);
    format!("({a} {op} {b})")
}

fn gen_expr(rng: &mut Rng, depth: u32, vars: &[String]) -> String {
    if depth == 0 || rng.below(3) == 0 {
        if rng.below(2) == 0 {
            vars[rng.below(vars.len() as u64) as usize].clone()
        } else {
            rng.below(10).to_string()
        }
    } else {
        // The select needs a bit condition, and the equality tests produce one, so route
        // selects through a generated bit and let equalities also appear as plain 0/1
        // values. Both exercise the fold arms the pure arithmetic grammar misses.
        match rng.below(5) {
            0 => {
                let c = gen_bit(rng, depth - 1, vars);
                let a = gen_expr(rng, depth - 1, vars);
                let b = gen_expr(rng, depth - 1, vars);
                format!("sel({c}, {a}, {b})")
            }
            1 => gen_bit(rng, depth - 1, vars),
            _ => {
                let op = ["+", "-", "*"][rng.below(3) as usize];
                let a = gen_expr(rng, depth - 1, vars);
                let b = gen_expr(rng, depth - 1, vars);
                format!("({a} {op} {b})")
            }
        }
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

// A program that carries two accumulators across a loop, rebinding them each iteration,
// the exact shape the loop-rebinding bug lived in. `c` is a loop-invariant input, `s` and
// `t` evolve, so the optimizer must propagate the first and never the second.
fn gen_loop_program(rng: &mut Rng) -> (String, Vec<u64>) {
    let mut src = String::from("input a;\ninput b;\ninput c;\nlet s = a;\nlet t = b;\n");
    let body_vars: Vec<String> = vec!["s".into(), "t".into(), "c".into(), "i".into()];
    let k = 2 + rng.below(7);
    src.push_str(&format!("for i in 0 .. {k} {{\n"));
    src.push_str(&format!("  let u = {};\n", gen_expr(rng, 3, &body_vars)));
    src.push_str("  let s = t;\n  let t = u;\n}\n");
    src.push_str("output s;\n");
    let inputs = vec![rng.below(50), rng.below(50), rng.below(50)];
    (src, inputs)
}

fn agree(src: &str, inputs: &[u64]) {
    let opt = compile_source(src).unwrap_or_else(|e| panic!("optimized:\n{src}\n{e:?}"));
    let raw = compile_source_unoptimized(src).unwrap_or_else(|e| panic!("raw:\n{src}\n{e:?}"));
    let a = evaluate(&opt, inputs, &[]).unwrap_or_else(|e| panic!("run opt:\n{src}\n{e:?}"));
    let b = evaluate(&raw, inputs, &[]).unwrap_or_else(|e| panic!("run raw:\n{src}\n{e:?}"));
    assert_eq!(
        a, b,
        "the optimizer changed the output of:\n{src}\ninputs {inputs:?}"
    );
}

#[test]
fn the_optimizer_is_equivalent_under_fuzzing() {
    let mut rng = Rng(0x0DDC0FFEE);
    for _ in 0..3000 {
        let (src, inputs) = gen_program(&mut rng);
        agree(&src, &inputs);
    }
}

#[test]
fn the_optimizer_is_equivalent_under_loop_fuzzing() {
    // The loop-carried-accumulator class. A propagation that treats an evolving name as a
    // constant, the bug that shipped once, fails here.
    let mut rng = Rng(0xBADF00D);
    for _ in 0..2000 {
        let (src, inputs) = gen_loop_program(&mut rng);
        agree(&src, &inputs);
    }
}

// Emit a program as C, compile it with the system compiler exactly as the shipping path
// does, run it on the inputs, and parse the field outputs it prints. A divergence from the
// VM is an emitter bug, because the native and the proven trace are the same op list.
fn native_c(src: &str, inputs: &[u64], tag: usize) -> Vec<u64> {
    let program = compile_source(src).expect("compile");
    let c = to_c(&program);
    let dir = std::env::temp_dir();
    let cpath = dir.join(format!("zkfuzz_{tag}.c"));
    let bpath = dir.join(format!("zkfuzz_{tag}.bin"));
    std::fs::write(&cpath, &c).expect("write c");
    let status = std::process::Command::new("cc")
        .arg(&cpath)
        .arg("-O2")
        .arg("-w")
        .arg("-o")
        .arg(&bpath)
        .status()
        .expect("cc");
    assert!(status.success(), "cc failed for a fuzzed program");
    let mut cmd = std::process::Command::new(&bpath);
    for v in inputs {
        cmd.arg(v.to_string());
    }
    let out = cmd.output().expect("run native");
    assert!(
        out.status.success(),
        "native run failed for a fuzzed program"
    );
    String::from_utf8(out.stdout)
        .unwrap()
        .split_whitespace()
        .map(|t| t.parse().unwrap())
        .collect()
}

#[test]
fn the_c_backend_agrees_with_the_vm_under_fuzzing() {
    // The native backend and the VM must compute the same field arithmetic on the same op
    // list. Where the curated backend tests cover a handful of shapes, this throws random
    // expression trees at the C emitter and checks each against the VM. Bounded by the cost
    // of invoking the C compiler once per program.
    let mut rng = Rng(0xB1305);
    for i in 0..100 {
        let (src, inputs) = gen_program(&mut rng);
        let program = compile_source(&src).unwrap_or_else(|e| panic!("compile:\n{src}\n{e:?}"));
        let vm = evaluate(&program, &inputs, &[]).unwrap_or_else(|e| panic!("vm:\n{src}\n{e:?}"));
        let native = native_c(&src, &inputs, i);
        assert_eq!(
            vm, native,
            "the C backend diverged from the VM on:\n{src}\ninputs {inputs:?}"
        );
    }
}

// Emit a program as x86_64 assembly, assemble and link it with the C runtime, run it, and
// parse the field outputs. The assembly emitter hand-writes the field arithmetic, so this is
// where a reduction or carry bug the higher backends do not share would show.
fn native_asm(src: &str, inputs: &[u64], tag: usize) -> Vec<u64> {
    let program = compile_source(src).expect("compile");
    let asm = to_asm(&program);
    let dir = std::env::temp_dir();
    let spath = dir.join(format!("zkfuzz_{tag}.S"));
    let bpath = dir.join(format!("zkfuzz_{tag}_asm.bin"));
    std::fs::write(&spath, &asm).expect("write asm");
    let status = std::process::Command::new("cc")
        .arg(&spath)
        .arg("-o")
        .arg(&bpath)
        .status()
        .expect("cc");
    assert!(status.success(), "assemble failed for a fuzzed program");
    let mut cmd = std::process::Command::new(&bpath);
    for v in inputs {
        cmd.arg(v.to_string());
    }
    let out = cmd.output().expect("run asm");
    assert!(out.status.success(), "asm run failed for a fuzzed program");
    String::from_utf8(out.stdout)
        .unwrap()
        .split_whitespace()
        .map(|t| t.parse().unwrap())
        .collect()
}

#[test]
fn the_asm_backend_agrees_with_the_vm_under_fuzzing() {
    // The assembly emitter is the lowest and hand-written of the backends, so it carries the
    // field arithmetic itself. Throw random expression trees at it and check each against the
    // VM. Bounded by the cost of assembling once per program.
    let mut rng = Rng(0xA55E33);
    for i in 0..80 {
        let (src, inputs) = gen_program(&mut rng);
        let program = compile_source(&src).unwrap_or_else(|e| panic!("compile:\n{src}\n{e:?}"));
        let vm = evaluate(&program, &inputs, &[]).unwrap_or_else(|e| panic!("vm:\n{src}\n{e:?}"));
        let native = native_asm(&src, &inputs, i);
        assert_eq!(
            vm, native,
            "the asm backend diverged from the VM on:\n{src}\ninputs {inputs:?}"
        );
    }
}
