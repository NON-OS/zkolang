/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Structs through the ABI: a struct parameter takes one value per field, in order, each
 * checked against its field's type; a struct result gives one value per field; and the
 * run is proved hiding the secret struct.
 */

use nonos_zkolang::compiler::driver::abi::AbiError;
use nonos_zkolang::compiler::driver::{prove, run, RunFailure};

use crate::prove_2026_tests::{built, SEED};

const SRC: &str = "struct Point {\n    x: u32,\n    y: i64,\n}\n\nfn main(p: public Point, s: secret Point) -> public Point {\n    declassify(Point { x: p.x + s.x, y: p.y - s.y })\n}\n";

#[test]
fn a_struct_goes_in_and_comes_out_field_by_field() {
    let b = built(SRC);
    assert_eq!(run(&b, &[3, -5], &[4, 10]), Ok(vec![7, -15]));
    let p = prove(&b, &[3, -5], &[4, 10], &SEED).expect("proves");
    assert!(p.report.verified);
    assert_eq!(p.outputs, vec![7, -15]);
    let range = AbiError::Range { position: 0 };
    assert_eq!(
        run(&b, &[1 << 32, 0], &[0, 0]),
        Err(RunFailure::Inputs(range, false))
    );
    let count = AbiError::Count {
        expected: 2,
        got: 1,
    };
    assert_eq!(run(&b, &[1, 2], &[3]), Err(RunFailure::Inputs(count, true)));
}
