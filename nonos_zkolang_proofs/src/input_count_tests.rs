/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A run must supply exactly the public inputs and secrets the program declares. */

use nonos_zkolang::{prove_source_with_witness, RunError};

#[test]
fn inputs_must_match_what_the_program_declares() {
    /*
     * Two public inputs declared, one supplied and a secret alongside: the second `input`
     * would read the secret, a value the public statement does not carry, and the proof
     * would claim a public input nobody bound. The run is refused instead.
     */
    let src = "input a; input b; output a + b;";
    let short = prove_source_with_witness(src, &[1], &[2]);
    assert_eq!(
        short,
        Err(RunError::InputCount {
            public_expected: 2,
            public_got: 1,
            secret_expected: 0,
            secret_got: 1,
        })
    );
    /* A declared secret passed as a public input would put it in the statement. */
    let revealed = prove_source_with_witness("input a; secret w; output a;", &[1, 2], &[]);
    assert!(matches!(revealed, Err(RunError::InputCount { .. })));
    /* Too many secrets would misplace the comparison advice. */
    let extra = prove_source_with_witness("secret w; output w < 5;", &[], &[1, 2]);
    assert!(matches!(extra, Err(RunError::InputCount { .. })));
    /* The exact counts prove. */
    let exact = prove_source_with_witness(src, &[1, 2], &[]).expect("run");
    assert!(exact.verified);
    assert_eq!(exact.outputs, vec![3]);
}
