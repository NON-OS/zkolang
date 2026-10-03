/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A zKølang run proven in STARKs format 7 and verified by `nox_verify`, the verifier the
 * STARKs gates link, against a statement made from the program alone and public words a
 * verifier computes for itself. Other outputs, other inputs, a damaged proof and another
 * program's statement are refused, and a failing run has no proof.
 */

use nonos_zkolang::compiler::driver::{build, seed_of, Built, RunFailure, Source, SEED_BYTES};
use nonos_zkolang::compiler::source::SourceMap;
use nonos_zkolang::compiler::syntax::load::NoFiles;
use nonos_zkolang_format7::{prove, statement, verify, words, Error, Refusal};

const SUM: &str = "fn main(x: public u32, y: secret u32) -> u32 {
    assert y < 100, \"y is small\";
    declassify(x * x + y)
}
";

fn built(src: &str) -> Built {
    let src = Source::file("sum.zkl", src.into());
    build(&mut SourceMap::new(), &NoFiles, src).expect("builds")
}

#[test]
fn a_run_proves_in_format_7_and_nox_verify_accepts_it() {
    let b = built(SUM);
    let p = prove(&b, &[12], &[5], &seed_of(&[7; SEED_BYTES])).expect("proves");
    assert_eq!(p.outputs, [149]);
    let st = statement(&b).expect("a statement");
    assert!(
        st == p.statement,
        "the program's statement is the one proven"
    );
    let w = words(&b, &st, &[12], &p.outputs).expect("words");
    assert_eq!(w, p.words);
    assert!(p.bytes.len() < 96 * 1024, "{} bytes", p.bytes.len());
    verify(&st, &p.bytes, &w).expect("nox_verify accepts the proof");

    let refused = |st, bytes: &[u8], w: &[u64]| verify(st, bytes, w).map_err(|e| e.0);
    let more = words(&b, &st, &[12], &[150]).expect("words");
    assert_eq!(
        refused(&st, &p.bytes, &more),
        Err(Refusal::Proof),
        "another output"
    );
    let other = words(&b, &st, &[13], &[149]).expect("words");
    assert_eq!(
        refused(&st, &p.bytes, &other),
        Err(Refusal::Proof),
        "another input"
    );
    let mut bad = p.bytes.clone();
    let at = bad.len() / 2;
    bad[at] ^= 1;
    assert!(refused(&st, &bad, &w).is_err(), "a damaged proof");
    let square = statement(&built("fn main(x: public u32) -> u32 {\n    x * x\n}\n"));
    let square = square.expect("a statement");
    assert!(
        refused(&square, &p.bytes, &w).is_err(),
        "another program's statement"
    );
}

#[test]
fn a_failing_run_has_no_proof() {
    let r = prove(&built(SUM), &[12], &[200], &seed_of(&[7; SEED_BYTES]));
    assert!(matches!(r, Err(Error::Run(RunFailure::Fails(_)))));
}
