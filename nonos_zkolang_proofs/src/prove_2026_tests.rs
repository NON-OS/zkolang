/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Edition 2026 programs proved with the STARK, hiding the witness: a verified proof of an
 * honest run with its result, a wrong witness refused with the reference run's failure,
 * and inputs outside their types refused before any run.
 */

use nonos_stark::air::RATE;
use nonos_stark::field::Fp;
use nonos_zkolang::compiler::driver::abi::AbiError;
use nonos_zkolang::compiler::driver::{build, prove, Built, RunFailure};
use nonos_zkolang::compiler::interp::FailKind;
use nonos_zkolang::compiler::source::SourceMap;

/** The program `src`, built. */
pub(crate) fn built(src: &str) -> Built {
    let mut map = SourceMap::new();
    let id = map.add(String::from("p.zkl"), String::from(src));
    build(id, src).unwrap_or_else(|d| panic!("{:?}", d.items()))
}

/** A blinding seed for tests; a real prover draws a fresh one per proof. */
pub(crate) const SEED: [Fp; RATE] = [Fp::ONE; RATE];

#[test]
fn a_secret_square_root_is_proved() {
    let b = built("fn main(y: public u32, x: secret u32) -> public u32 {\n    assert x * x == y, \"x is a root\";\n    y + 1\n}\n");
    let p = prove(&b, &[49], &[7], &SEED).expect("proves");
    assert!(p.report.verified);
    assert_eq!(p.outputs, vec![50]);
    match prove(&b, &[49], &[6], &SEED) {
        Err(RunFailure::Fails(f)) => assert!(matches!(f.kind, FailKind::Assert(_))),
        other => panic!("{other:?}"),
    }
    let wide = prove(&b, &[1 << 32], &[7], &SEED).err();
    let range = AbiError::Range { position: 0 };
    assert_eq!(wide, Some(RunFailure::Inputs(range, false)));
    let none = prove(&b, &[49], &[], &SEED).err();
    let count = AbiError::Count {
        expected: 1,
        got: 0,
    };
    assert_eq!(none, Some(RunFailure::Inputs(count, true)));
}

/** `mix` below, computed on the field directly. */
fn mix(x: u64, k: u64) -> u64 {
    let (mut t, k) = (Fp::from_u64(x), Fp::from_u64(k));
    for i in 0..8 {
        let s = t + k + Fp::from_u64(i);
        t = s * s * s;
    }
    t.value()
}

#[test]
fn a_preimage_of_a_field_permutation_is_proved() {
    let src = "fn mix(x: field, k: field) -> field {\n    let mut t = x;\n    for i in 0..8 {\n        t = (t + k + i as field).pow(3);\n    }\n    t\n}\n\nfn main(h: public field, pre: secret field) -> public bool {\n    declassify(mix(pre, 7) == h)\n}\n";
    let b = built(src);
    let h = i128::from(mix(12345, 7));
    let p = prove(&b, &[h], &[12345], &SEED).expect("proves");
    assert_eq!(p.outputs, vec![1]);
    let p = prove(&b, &[h], &[12346], &SEED).expect("proves");
    assert_eq!(p.outputs, vec![0]);
}
