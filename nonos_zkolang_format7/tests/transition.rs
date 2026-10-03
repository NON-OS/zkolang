/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The recorded transition is the step AIR's: replayed at points it gives what a real
 * program's AIR gives there, the same tape each time it is recorded, and a tape with one
 * operation changed does not.
 */

use nonos_stark::air::{Air, AirExt};
use nonos_zkolang::compiler::driver::{build, traced, Source};
use nonos_zkolang::compiler::source::SourceMap;
use nonos_zkolang::compiler::syntax::load::NoFiles;
use nonos_zkolang_format7::{probe, record, replay, Op};

const SQUARE: &str = "fn main(x: public u32) -> u32 {\n    x * x\n}\n";

#[test]
fn the_tape_replays_to_a_real_programs_transition() {
    let src = Source::file("square.zkl", SQUARE.into());
    let b = build(&mut SourceMap::new(), &NoFiles, src).expect("builds");
    let air = traced(&b, &[12], &[], 1).expect("runs").air;
    let t = record().expect("records");
    assert_eq!(t.n_frame, air.window_size() * air.trace_width());
    assert_eq!(t.outputs.len(), air.num_transition());
    for seed in [1, 2, 3] {
        let (frame, periodic) = probe(t.n_frame + seed, t.n_periodic);
        let frame = &frame[seed..];
        assert_eq!(periodic.len(), air.periodic_columns().len());
        let want = air.transition_ext(frame, &periodic);
        assert!(replay(&t, frame, &periodic) == want, "probe {seed}");
    }
}

#[test]
fn the_tape_is_the_same_each_time() {
    let (a, b) = (record().expect("records"), record().expect("records"));
    assert_eq!((a.ops, a.outputs), (b.ops, b.outputs));
}

#[test]
fn a_changed_operation_does_not_replay() {
    let good = record().expect("records");
    let mut bad = record().expect("records");
    let at = bad
        .ops
        .iter()
        .position(|op| matches!(op, Op::Mul(..)))
        .expect("a multiplication");
    if let Op::Mul(a, b) = bad.ops[at] {
        bad.ops[at] = Op::Add(a, b);
    }
    let (frame, periodic) = probe(good.n_frame, good.n_periodic);
    let (x, y) = (
        replay(&good, &frame, &periodic),
        replay(&bad, &frame, &periodic),
    );
    assert!(x.iter().zip(&y).any(|(a, b)| a != b));
}
