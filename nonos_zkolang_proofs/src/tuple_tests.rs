/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Multiple return values. A function can return several values at once, and a `let (a, b)`
//! destructures them. This is what lets a compare-swap, the primitive sorting is built
//! from, be a function at all: it returns the smaller and the larger of a pair. Each
//! program here is compiled and proven, and the errors that keep destructuring honest, a
//! name count that does not match the arity and a tuple used where one value is required,
//! are pinned too.

use nonos_zkolang::{compile_source, prove_source_with_inputs, CompileError};

// The compare-swap, shared by the tests below: order a pair, smaller first, as a tuple.
const MINMAX: &str = "fn minmax(a, b) {\n    let ordered = a < b;\n    return (sel(ordered, a, b), sel(ordered, b, a));\n}\n";

#[test]
fn a_function_returns_two_values() {
    let src =
        format!("{MINMAX}input a;\ninput b;\nlet (lo, hi) = minmax(a, b);\noutput lo;\noutput hi;");
    let report = prove_source_with_inputs(&src, &[7, 3]).expect("run");
    assert!(report.verified, "a two-value return was rejected");
    assert_eq!(report.outputs, vec![3, 7], "the smaller then the larger");
}

#[test]
fn a_sorting_network_orders_three_values() {
    // Three compare-swaps sort three inputs, which was impossible before a function could
    // return the two values a compare-swap produces.
    let src = format!(
        "{MINMAX}input x;\ninput y;\ninput z;\n\
         let (m0, hi) = minmax(y, z);\n\
         let (lo, m1) = minmax(x, m0);\n\
         let (mid, top) = minmax(m1, hi);\n\
         output lo;\noutput mid;\noutput top;"
    );
    let cases = [
        ([3u64, 1, 2], [1u64, 2, 3]),
        ([40, 10, 25], [10, 25, 40]),
        ([9, 8, 7], [7, 8, 9]),
        ([5, 5, 5], [5, 5, 5]),
    ];
    for (input, want) in cases {
        let report = prove_source_with_inputs(&src, &input).expect("run");
        assert!(report.verified, "a sort was rejected");
        assert_eq!(report.outputs, want.to_vec(), "inputs came out unsorted");
    }
}

#[test]
fn a_direct_tuple_destructures() {
    // The right side need not be a call: a tuple literal destructures the same way.
    let src = "input a;\ninput b;\nlet (sum, diff) = (a + b, a - b);\noutput sum;\noutput diff;";
    let report = prove_source_with_inputs(src, &[10, 4]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![14, 6]);
}

#[test]
fn a_block_body_destructures_the_tuple_it_calls() {
    // A function computes the median of three by destructuring the compare-swap it calls,
    // three times, inside its own body. This composition needs a block that can bind more
    // than one name, so it is the proof the feature reaches inside a function.
    let src = format!(
        "{MINMAX}fn median3(a, b, c) {{\n\
         \x20   let (lo1, hi1) = minmax(a, b);\n\
         \x20   let (lo2, hi2) = minmax(hi1, c);\n\
         \x20   let (m, mid) = minmax(lo1, lo2);\n\
         \x20   return mid;\n\
         }}\ninput x;\ninput y;\ninput z;\noutput median3(x, y, z);"
    );
    let cases = [
        ([3u64, 1, 2], 2u64),
        ([5, 9, 1], 5),
        ([100, 50, 75], 75),
        ([7, 7, 7], 7),
    ];
    for (input, want) in cases {
        let report = prove_source_with_inputs(&src, &input).expect("run");
        assert!(report.verified, "a median was rejected");
        assert_eq!(report.outputs, vec![want], "wrong median");
    }
}

#[test]
fn a_wildcard_ignores_a_returned_value() {
    // `_` in a destructuring binds nothing, so you can take one of the two values a
    // compare-swap returns without naming the other.
    let src = format!(
        "{MINMAX}input x;\ninput y;\nlet (_, hi) = minmax(x, y);\nlet (lo, _) = minmax(x, y);\noutput hi;\noutput lo;"
    );
    let report = prove_source_with_inputs(&src, &[3, 7]).expect("run");
    assert!(report.verified);
    assert_eq!(report.outputs, vec![7, 3], "the max, then the min");
}

#[test]
fn the_name_count_must_match_the_arity() {
    // minmax returns two values; naming three of them is an error the compiler reports.
    let three = format!("{MINMAX}input a;\ninput b;\nlet (x, y, z) = minmax(a, b);\noutput x;");
    assert!(
        matches!(
            compile_source(&three),
            Err(CompileError::TupleArity {
                names: 3,
                values: 2
            })
        ),
        "a wrong name count was accepted"
    );
}

#[test]
fn a_tuple_is_not_a_scalar() {
    // A tuple cannot be bound to one name or used where a single value is required.
    let bind = format!("{MINMAX}input a;\ninput b;\nlet x = minmax(a, b);\noutput x;");
    assert!(
        matches!(compile_source(&bind), Err(CompileError::TupleNotScalar)),
        "a tuple was bound to a single name"
    );
    let used = "input a;\ninput b;\noutput (a, b);";
    assert!(
        matches!(compile_source(used), Err(CompileError::TupleNotScalar)),
        "a tuple was used as a value"
    );
}
