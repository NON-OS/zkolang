/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The tables a source file is indexed by. A line ends at a line feed, a carriage return,
 * or the two together (spec section 2.1). Columns count characters; the character count
 * before an offset is read from a mark every `MARK_STRIDE` bytes plus at most that many
 * bytes counted, so a column costs the same wherever it falls in a long line.
 */

use alloc::vec::Vec;

/** Bytes between two character-count marks. */
pub(super) const MARK_STRIDE: usize = 256;

/** The byte offset each line starts at. */
pub(super) fn line_starts(b: &[u8]) -> Vec<u32> {
    let mut starts = Vec::new();
    starts.push(0);
    for (i, &c) in b.iter().enumerate() {
        let ends = c == b'\n' || (c == b'\r' && b.get(i + 1) != Some(&b'\n'));
        if ends {
            starts.push(u32::try_from(i + 1).unwrap_or(u32::MAX));
        }
    }
    starts
}

/**
 * The number of characters before byte `k * MARK_STRIDE`, for each `k`. A file of ASCII
 * gets no marks: its character counts are its byte offsets.
 */
pub(super) fn char_marks(b: &[u8]) -> Vec<u32> {
    if b.is_ascii() {
        return Vec::new();
    }
    let mut marks = Vec::with_capacity(b.len() / MARK_STRIDE + 1);
    let mut chars: u32 = 0;
    for chunk in b.chunks(MARK_STRIDE) {
        marks.push(chars);
        chars = chars.saturating_add(starts_in(chunk));
    }
    marks.push(chars);
    marks
}

/** The number of characters that start in `bytes`: every byte but a continuation byte. */
pub(super) fn starts_in(bytes: &[u8]) -> u32 {
    let n = bytes.iter().filter(|&&c| c & 0xC0 != 0x80).count();
    u32::try_from(n).unwrap_or(u32::MAX)
}
