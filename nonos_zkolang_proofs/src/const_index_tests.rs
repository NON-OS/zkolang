/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A constant index folds in the field in every build. */

use nonos_zkolang::{compile_source, compile_source_unoptimized, evaluate};

#[test]
fn a_constant_index_folds_in_the_field_in_every_build() {
    /*
     * A literal is a field element, so `p + 1` is one. The optimizer folds an index in the
     * field; the unoptimized build folded it over i128, disagreed about which entry it
     * named, and overflowed on a large enough product, a panic in a debug build.
     */
    let wrapped = "const T = [7, 8, 9];\noutput T[18446744069414584321 + 1];";
    for ops in [
        compile_source(wrapped).expect("compile"),
        compile_source_unoptimized(wrapped).expect("compile"),
    ] {
        assert_eq!(evaluate(&ops, &[], &[]).expect("run"), vec![8]);
    }
    let huge = "const T = [7, 8, 9];\n\
                output T[18446744073709551615 * 18446744073709551615 * 18446744073709551615];";
    assert!(compile_source(huge).is_err());
    assert!(compile_source_unoptimized(huge).is_err());
    let negative = "const T = [7, 8, 9];\noutput T[0 - 1];";
    assert!(compile_source(negative).is_err());
    assert!(compile_source_unoptimized(negative).is_err());
}
