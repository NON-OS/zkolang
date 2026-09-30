/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Character classes for display. The tables cover the blocks source text meets in
 * comments and string literals; a character outside them takes one column, which is
 * what a terminal gives most characters.
 */

/** Characters with no visible form, or that change the direction of the text around them. */
pub(super) fn invisible(c: char) -> bool {
    matches!(c as u32,
        0x80..=0x9F | 0xAD | 0x34F | 0x61C | 0x115F | 0x1160 | 0x17B4 | 0x17B5 | 0x180E
        | 0x200B..=0x200F | 0x2028..=0x202E | 0x2060..=0x206F | 0x3164 | 0xFEFF
        | 0xFFA0 | 0xFFF9..=0xFFFB | 0xE0000..=0xE007F)
}

/** The number of columns a visible character takes: none for a combining mark, two for a wide one. */
pub(super) fn width(c: char) -> usize {
    let u = c as u32;
    if combining(u) {
        0
    } else if wide(u) {
        2
    } else {
        1
    }
}

/** Marks drawn over the character before them. */
fn combining(u: u32) -> bool {
    matches!(u,
        0x300..=0x36F | 0x483..=0x489 | 0x591..=0x5BD | 0x610..=0x61A | 0x64B..=0x65F
        | 0x1AB0..=0x1AFF | 0x1DC0..=0x1DFF | 0x20D0..=0x20FF | 0xFE00..=0xFE0F
        | 0xFE20..=0xFE2F | 0xE0100..=0xE01EF)
}

/** East Asian wide and fullwidth characters, and emoji. */
fn wide(u: u32) -> bool {
    matches!(u,
        0x1100..=0x115E | 0x2E80..=0x303E | 0x3041..=0x33FF | 0x3400..=0x4DBF
        | 0x4E00..=0x9FFF | 0xA000..=0xA4CF | 0xA960..=0xA97F | 0xAC00..=0xD7A3
        | 0xF900..=0xFAFF | 0xFE30..=0xFE4F | 0xFF00..=0xFF60 | 0xFFE0..=0xFFE6
        | 0x1F300..=0x1F64F | 0x1F900..=0x1F9FF | 0x20000..=0x3FFFD)
}
