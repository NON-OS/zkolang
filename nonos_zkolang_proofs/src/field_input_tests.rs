/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * An input the field cannot hold is refused. The library entry points reduced it modulo
 * p, so a caller passing 2^64 - 1 got a verified proof that the input was 4294967294.
 */

use nonos_zkolang::{compile_source, evaluate, prove_source_with_witness, run, RunError};

const P: u64 = 0xFFFF_FFFF_0000_0001;

#[test]
fn an_input_past_the_modulus_is_refused() {
    let src = "input a;\nsecret s;\nassert a == 4294967294;\noutput a + s;";
    let refused = |position| Err(RunError::InputNotInField { position });
    assert_eq!(
        prove_source_with_witness(src, &[u64::MAX], &[0]).map(|r| r.outputs),
        refused(0)
    );
    assert_eq!(
        prove_source_with_witness(src, &[4294967294], &[P]).map(|r| r.outputs),
        refused(1)
    );
    assert_eq!(run(src, &[P], &[0]).map(|r| r.outputs), refused(0));
    let ops = compile_source(src).expect("compile");
    assert_eq!(evaluate(&ops, &[P + 4294967294], &[0]), refused(0));
    /* The largest element still binds. */
    assert_eq!(
        evaluate(&ops, &[4294967294], &[P - 1]),
        Ok(vec![4294967293])
    );
}
