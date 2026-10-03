/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Enums through the ABI: an enum takes and gives one value per slot, its tag and then its
 * payload as laid out. Slots that lay out no value of the enum are refused before a run,
 * and the compiled program itself rejects them, so a prover that skips the ABI cannot
 * pass them. The run is proved hiding a secret enum.
 */

use nonos_zkolang::compiler::driver::abi::AbiError;
use nonos_zkolang::compiler::driver::{prove, run, RunFailure};

use crate::compile_run::fp;
use crate::prove_2026_tests::{built, SEED};

const SRC: &str = "enum Coin {\n    Heads,\n    Tails(u8),\n    Edge { w: i64, up: bool },\n}\n\nfn flip(c: Coin) -> Coin {\n    match c {\n        Coin::Heads => Coin::Tails(1),\n        Coin::Tails(n) => Coin::Edge { w: 0 - (n as i64), up: n > 3 },\n        Coin::Edge { w, .. } => if w < 0 { Coin::Heads } else { Coin::Tails(7) },\n    }\n}\n\nfn main(p: public Coin, s: secret Coin) -> public (Coin, Coin) {\n    declassify((flip(p), flip(s)))\n}\n";

/** `-5` as an `i64`'s two slots. */
const MINUS_FIVE: [i128; 2] = [4294967291, 4294967295];

#[test]
fn an_enum_goes_in_and_comes_out_slot_by_slot() {
    let b = built(SRC);
    let edge = [2, MINUS_FIVE[0], MINUS_FIVE[1], 1];
    let want: Vec<i128> = [1, 1, 0, 0].into_iter().chain(edge).collect();
    assert_eq!(run(&b, &[0, 0, 0, 0], &[1, 5, 0, 0]), Ok(want.clone()));
    let p = prove(&b, &[0, 0, 0, 0], &[1, 5, 0, 0], &SEED).expect("proves");
    assert!(p.report.verified);
    assert_eq!(p.outputs, want);
    let back = vec![0, 0, 0, 0, 2, 4294967289, 4294967295, 1];
    assert_eq!(run(&b, &edge, &[1, 7, 0, 0]), Ok(back));
}

#[test]
fn slots_that_lay_out_no_value_are_refused() {
    let b = built(SRC);
    let at = |position, secret| Err(RunFailure::Inputs(AbiError::Range { position }, secret));
    assert_eq!(run(&b, &[3, 0, 0, 0], &[0, 0, 0, 0]), at(0, false));
    assert_eq!(run(&b, &[0, 1, 0, 0], &[0, 0, 0, 0]), at(0, false));
    assert_eq!(run(&b, &[0, 0, 0, 0], &[1, 5, 9, 0]), at(0, true));
    assert_eq!(run(&b, &[0, 0, 0, 0], &[1, 300, 0, 0]), at(0, true));
    assert_eq!(run(&b, &[0, 0, 0, 0], &[2, 0, 0, 2]), at(0, true));
    assert_eq!(run(&b, &[0, 0, 0, 0], &[2, 1 << 32, 0, 0]), at(0, true));
}

#[test]
fn the_compiled_program_rejects_them_too() {
    let b = built(SRC);
    let ok: Vec<i128> = vec![0, 0, 0, 0, 1, 5, 0, 0];
    let slots = |v: &[i128]| v.iter().map(|&x| fp(x)).collect::<Vec<_>>();
    assert!(crate::compile_run::run(&b.compiled, &slots(&ok), 4).is_some());
    let bad: [[i128; 8]; 5] = [
        [3, 0, 0, 0, 1, 5, 0, 0],
        [0, 1, 0, 0, 1, 5, 0, 0],
        [0, 0, 0, 0, 1, 5, 9, 0],
        [0, 0, 0, 0, 1, 300, 0, 0],
        [0, 0, 0, 0, 2, 0, 0, 2],
    ];
    for v in bad {
        assert!(
            crate::compile_run::run(&b.compiled, &slots(&v), 4).is_none(),
            "{v:?}"
        );
    }
}
