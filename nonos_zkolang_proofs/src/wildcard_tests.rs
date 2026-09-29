/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A wildcard in a destructuring `let` and the registers it may hand back. */

use nonos_zkolang::prove_source_with_inputs;

#[test]
fn a_wildcard_does_not_free_a_register_a_later_name_binds() {
    /*
     * A function may return one register twice. Discarding the first copy must not hand
     * the register back to the pool while the second copy's name still holds it, or the
     * next allocation overwrites that name.
     */
    let top = "fn twice(x) { let t = x * 2; return (t, t); }\n\
               input a;\nlet (_, y) = twice(a);\nlet z = a + 100;\noutput y;\noutput z;";
    let report = prove_source_with_inputs(top, &[5]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![10, 105], "y was overwritten by z");

    /* The same destructure inside a block body. */
    let local = "fn twice(x) { let t = x * 2; return (t, t); }\n\
                 fn keep(a) { let (_, y) = twice(a); let z = a + 100; return y + z; }\n\
                 input a;\noutput keep(a);";
    let report = prove_source_with_inputs(local, &[5]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![115], "y was overwritten by z");
}
