/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The pins of the step AIR's boundaries. Resolved against a run's public statement each
 * is the value `boundary()` binds, and they are the same for every run of a program, so a
 * circuit read apart from its statement names the public words without their values.
 */

use nonos_stark::air::Air;
use nonos_zkolang::compiler::driver::{build, traced, Source, Traced};
use nonos_zkolang::compiler::source::SourceMap;
use nonos_zkolang::compiler::syntax::load::NoFiles;
use nonos_zkolang::Pin;

const BOTH: &str = "fn main(a: public u32, b: public u32, s: secret u32) -> (u32, bool) {
    assert s > a, \"too small\";
    declassify((a + b * a, s < b))
}
";

fn ran(public: &[i128], secret: &[i128]) -> Traced {
    let src = Source::file("both.zkl", BOTH.into());
    let b = build(&mut SourceMap::new(), &NoFiles, src).expect("builds");
    traced(&b, public, secret).expect("runs")
}

#[test]
fn every_pin_resolves_to_the_value_it_pins() {
    let t = ran(&[3, 40], &[7]);
    let (pins, values) = (t.air.boundary_pins(), t.air.boundary());
    assert_eq!(pins.len(), values.len());
    let (mut inputs, mut outputs) = (0, 0);
    for (&(c, r, pin), &(c2, r2, v)) in pins.iter().zip(&values) {
        assert_eq!((c, r), (c2, r2));
        let word = match pin {
            Pin::Value(x) => x,
            Pin::Input(k) => {
                inputs += 1;
                t.publics[5 + k]
            }
            Pin::Output(k) => {
                outputs += 1;
                t.publics[5 + t.inputs + k]
            }
        };
        assert_eq!(word, v, "column {c} row {r}");
    }
    assert!(
        inputs >= 2 && outputs == 2,
        "{inputs} inputs, {outputs} outputs"
    );
    assert_eq!(t.inputs, 2);
    assert_eq!(t.outputs, [123, 1]);
}

#[test]
fn the_pins_are_the_programs_not_the_runs() {
    let (a, b) = (ran(&[3, 40], &[7]), ran(&[9, 2], &[100]));
    assert_eq!(a.air.boundary_pins(), b.air.boundary_pins());
    assert_ne!(a.air.boundary(), b.air.boundary());
}
