/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The cost report (section 15.2) puts the rows of a loop's body on the body's line. */

use nonos_zkolang::compiler::driver::cost;

use crate::cost_tests::PROGRAM;
use crate::prove_2026_tests::built;

#[test]
fn the_line_with_the_most_rows_is_the_loop_body() {
    let b = built(PROGRAM);
    let c = cost(&b);
    let lo = PROGRAM.find("y = y * y + x").unwrap_or(0) as u32;
    let hi = lo + PROGRAM[lo as usize..].find('\n').unwrap_or(0) as u32;
    let on_line: usize = c
        .spans
        .iter()
        .filter(|(s, _)| s.lo >= lo && s.lo < hi)
        .map(|(_, n)| n)
        .sum();
    let most = c.spans.values().copied().max().unwrap_or(0);
    assert!(
        on_line * 2 > c.rows - c.overhead,
        "{on_line} of {} rows: {c:?}",
        c.rows
    );
    assert!(most > 0);
}
