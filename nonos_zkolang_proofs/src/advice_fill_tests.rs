/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * An honest program with ordered comparisons always gets its advice. The fill stopped
 * after eight passes, leaving a chain of nine dependent comparisons with stale bits, and
 * its first pass, where every comparison reads as true, refused a select whose condition
 * is a bit only once the comparisons settle. Both made true statements unprovable.
 */

use nonos_zkolang::{prove_source_with_witness, run};

fn outputs(src: &str, public: &[u64]) -> Vec<u64> {
    let r = prove_source_with_witness(src, public, &[]).expect("prove");
    assert!(r.verified);
    r.outputs
}

#[test]
fn a_long_chain_of_comparisons_settles() {
    let mut src = String::from("input a;\nlet x0 = a;\n");
    for k in 1..11 {
        src.push_str(&format!("let x{k} = x{} < 1;\n", k - 1));
    }
    src.push_str("output x10;");
    assert_eq!(outputs(&src, &[5]), vec![1]);
    let mut max = String::from("include \"order.zkl\";\ninput a0;\nlet m = a0;\n");
    for k in 1..10 {
        max.push_str(&format!("input a{k};\nlet m = max(m, a{k});\n"));
    }
    max.push_str("output m;");
    let r = run(&max, &[9, 1, 2, 3, 4, 5, 6, 7, 8, 0], &[]).expect("prove");
    assert!(r.verified);
    assert_eq!(r.outputs, vec![9]);
}

#[test]
fn a_select_on_settled_comparisons_is_provable() {
    let outside = "input x;\nlet out = (x < 10) + (x > 20);\noutput if out { 0 } else { x };";
    assert_eq!(outputs(outside, &[15]), vec![15]);
    assert_eq!(outputs(outside, &[5]), vec![0]);
    assert_eq!(outputs(outside, &[25]), vec![0]);
    let eq =
        "input a;\ninput b;\nlet eq = 1 - (a < b) - (b < a);\noutput if eq { 100 } else { 200 };";
    assert_eq!(outputs(eq, &[3, 3]), vec![100]);
    assert_eq!(outputs(eq, &[3, 4]), vec![200]);
}
