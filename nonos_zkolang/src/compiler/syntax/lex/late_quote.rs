/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Where a string that runs past the end of its line is taken to end. */

/**
 * The offset of the quote that closes, on the next line, a string whose line ended at
 * `at`: the first quote on that line, when what follows it ends a statement or argument.
 * A quote further down, or after the start of a comment, is not taken, so an unterminated
 * string swallows at most a line and never a comment's text.
 */
pub(super) fn late_quote(b: &[u8], at: usize) -> Option<usize> {
    let skip = if b.get(at..at + 2) == Some(b"\r\n") {
        2
    } else {
        1
    };
    let start = at + skip;
    let line = b.get(start..)?;
    let len = line
        .iter()
        .position(|&c| c == b'\n' || c == b'\r')
        .unwrap_or(line.len());
    let q = start + line.get(..len)?.iter().position(|&c| c == b'"')?;
    /* A quote inside a comment closes nothing. */
    let before = b.get(start..q)?;
    if before.windows(2).any(|w| w == b"//" || w == b"/*") {
        return None;
    }
    let after = b.get(q + 1..)?.iter().find(|&&c| c != b' ' && c != b'\t');
    let ends = matches!(
        after,
        None | Some(b';' | b',' | b')' | b']' | b'}' | b'\n' | b'\r')
    );
    ends.then_some(q)
}
