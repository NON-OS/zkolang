/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Every advice value of a compiled program is pinned (section 21.3): for programs using
 * the bit-level lowering (bitwise operators in lockstep, shifts of 64-bit halves, variable
 * exponents, wrapping arithmetic), each accepted witness stops being accepted when any
 * advice value moves by one, or two neighbours move by `+2, -1` or `-2, +1`.
 */

use crate::compile_run::{compile, slots};
use crate::prop_gen::Rng;
use crate::prop_model::TYPES;
use crate::sema_check::checked;
use nonos_zkolang::compiler::driver::witness;

const PROGRAMS: [(&str, &str); 6] = [
    ("fn main(a: public u8, b: public u8) -> public u8 { (a & b) ^ (a | 3u8) }", "u8"),
    ("fn main(a: public i16, b: public i16) -> public i16 { (a ^ b) | (a & -5i16) }", "i16"),
    ("fn main(a: public u64, b: public u64) -> public u64 { (a << u32::wrapping_from(b % 64u64)) ^ b }", "u64"),
    ("fn main(a: public i64, b: public i64) -> public i64 { a >> u32::wrapping_from((b % 64i64).max(0)) }", "i64"),
    ("fn main(a: public u32, b: public u32) -> public u32 { (a % 7u32).pow(b % 12u32) }", "u32"),
    ("fn main(a: public i32, b: public i32) -> public i32 { a.wrapping_mul(b).wrapping_sub(a) }", "i32"),
];

#[test]
fn every_advice_value_of_a_compiled_program_is_pinned() {
    let mut r = Rng(0x0A11_ADF1_CE00_D0E5);
    let (mut checked_runs, mut problems) = (0, Vec::new());
    for (case, (src, name)) in PROGRAMS.iter().enumerate() {
        let t = *TYPES.iter().find(|t| t.name == *name).expect("a type");
        let (program, codes) = checked(&format!("{src}\n"));
        assert!(codes.is_empty(), "{src}: {codes:?}");
        let c = compile(&program);
        for _ in 0..6 {
            let (a, b) = (r.value(t), r.value(t));
            let mut inputs = slots(a, t.bits);
            inputs.extend(slots(b, t.bits));
            let Ok(full) = witness(&c, &inputs) else {
                continue;
            };
            let n = inputs.len();
            if nonos_zkolang::Vm::new()
                .run(&c.machine.ops, &full, n)
                .is_err()
            {
                continue;
            }
            checked_runs += 1;
            problems.extend(crate::ssa_mutate::unpinned(&c.machine.ops, &full, n, case));
        }
    }
    assert!(problems.is_empty(), "{problems:#?}");
    assert!(checked_runs >= 18, "only {checked_runs} runs were accepted");
}
