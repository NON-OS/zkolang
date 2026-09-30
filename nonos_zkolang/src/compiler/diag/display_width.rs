/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Character classes for display. Widths follow the Unicode tables in `width_marks` and
 * `width_wide`; any other character takes one column, as a terminal gives it.
 */

use super::width_marks::MARKS;
use super::width_wide::WIDE;

/**
 * Characters with no visible form, that change the direction of the text around them, or
 * that change how the character before them is drawn: the Unicode Default_Ignorable code
 * points, the C1 controls, and the line, paragraph and annotation separators.
 */
pub(super) fn invisible(c: char) -> bool {
    matches!(c as u32,
        0x80..=0x9F | 0xAD | 0x34F | 0x61C | 0x115F | 0x1160 | 0x17B4 | 0x17B5
        | 0x180B..=0x180F | 0x200B..=0x200F | 0x2028..=0x202E | 0x2060..=0x206F | 0x3164
        | 0xFE00..=0xFE0F | 0xFEFF | 0xFFA0 | 0xFFF0..=0xFFFB | 0x1BCA0..=0x1BCA3
        | 0x1D173..=0x1D17A | 0xE0000..=0xE0FFF)
}

/** The number of columns a visible character takes: none for a mark, two for a wide one. */
pub(super) fn width(c: char) -> usize {
    let u = c as u32;
    if in_table(MARKS, u) {
        0
    } else if in_table(WIDE, u) {
        2
    } else {
        1
    }
}

/** Whether `c` is shown as written, with a form of its own that takes columns. */
pub(crate) fn has_form(c: char) -> bool {
    !c.is_control() && !invisible(c) && width(c) > 0
}

/** Whether `u` lies in one of the ranges of `table`, a start and an end each, sorted. */
fn in_table(table: &[u32], u: u32) -> bool {
    let (mut lo, mut hi) = (0, table.len() / 2);
    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        let start = table.get(2 * mid).copied().unwrap_or(u32::MAX);
        let end = table.get(2 * mid + 1).copied().unwrap_or(0);
        if u < start {
            hi = mid;
        } else if u > end {
            lo = mid + 1;
        } else {
            return true;
        }
    }
    false
}
