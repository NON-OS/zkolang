/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Sharing repeated subexpressions costs time close to the size of the statement. It
 * compared every candidate subtree against the whole statement node by node, again after
 * every hoist, so a sum of eight thousand terms took about a minute in a debug build;
 * equal subtrees now share one structural id.
 */

use std::time::{Duration, Instant};

use nonos_zkolang::{compile_source, compile_source_unoptimized, evaluate};

/** A balanced sum of `x * k` for `k` in `lo..hi`, nested as deep as its halving. */
fn balanced(lo: u64, hi: u64) -> String {
    if hi - lo == 1 {
        return format!("x * {lo}");
    }
    let mid = (lo + hi) / 2;
    format!("({} + {})", balanced(lo, mid), balanced(mid, hi))
}

#[test]
fn a_large_statement_is_shared_quickly() {
    let sum = balanced(1, 1 << 14);
    let src = format!("input x;\noutput {sum};\noutput {sum} + ({sum});");
    let start = Instant::now();
    let ops = compile_source(&src).expect("compile");
    assert!(
        start.elapsed() < Duration::from_secs(30),
        "{:?}",
        start.elapsed()
    );
    let plain = compile_source_unoptimized(&src).expect("compile");
    let want = evaluate(&plain, &[3], &[]).expect("run");
    assert_eq!(evaluate(&ops, &[3], &[]).expect("run"), want);
    /* Sharing is per statement: the second computes its repeated sum once. */
    assert!(
        ops.len() < plain.len() * 3 / 4,
        "{} of {}",
        ops.len(),
        plain.len()
    );
}
