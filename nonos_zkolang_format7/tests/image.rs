/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A program's image read back by the rules the STARKs verifiers parse it by: its tape is
 * the recorded transition, its boundaries resolved against a run's public words are the
 * AIR's, it is the same for every run of the program, and a damaged one does not read.
 */

mod support;

use nonos_stark::air::Air;
use nonos_zkolang::compiler::driver::{build, traced, Source, Traced};
use nonos_zkolang::compiler::source::SourceMap;
use nonos_zkolang::compiler::syntax::load::NoFiles;
use nonos_zkolang_format7::{image, record};
use support::read::read;
use support::source::Src;

const BOTH: &str = "fn main(a: public u32, b: public u32, s: secret u32) -> (u32, bool) {
    assert s > a, \"too small\";
    declassify((a + b * a, s < b))
}
";

fn ran(public: &[i128], secret: &[i128]) -> Traced {
    let src = Source::file("both.zkl", BOTH.into());
    let b = build(&mut SourceMap::new(), &NoFiles, src).expect("builds");
    traced(&b, public, secret, 1).expect("runs")
}

fn image_of(t: &Traced) -> Vec<u8> {
    image(&record().expect("records"), &t.air, t.inputs).expect("an image")
}

#[test]
fn an_image_reads_back_as_the_air() {
    let (t, tape) = (ran(&[3, 40], &[7]), record().expect("records"));
    let (back, log_t, bnd) = read(&image_of(&t)).expect("reads");
    assert_eq!(log_t, t.air.log_trace_len());
    assert_eq!((back.ops, back.outputs), (tape.ops, tape.outputs));
    assert_eq!(
        (back.n_frame, back.n_periodic),
        (tape.n_frame, tape.n_periodic)
    );
    let word = |s: &Src| match s {
        Src::Value(v) => *v,
        Src::Word(k) => t.publics[*k],
    };
    let resolved: Vec<_> = bnd.iter().map(|(c, r, s)| (*c, *r, word(s))).collect();
    assert_eq!(resolved, t.air.boundary());
    assert!(bnd.iter().any(|b| matches!(b.2, Src::Word(_))));
}

#[test]
fn an_image_is_the_programs_not_the_runs() {
    assert_eq!(
        image_of(&ran(&[3, 40], &[7])),
        image_of(&ran(&[9, 2], &[100]))
    );
}

#[test]
fn a_damaged_image_does_not_read() {
    let good = image_of(&ran(&[3, 40], &[7]));
    assert!(read(&good[..good.len() - 1]).is_none(), "truncated");
    assert!(
        read(&[&good[..], &[0]].concat()).is_none(),
        "a trailing byte"
    );
}
