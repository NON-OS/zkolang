/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Where a raw or byte string ends: after its closing quote and `#`s. */

/**
 * The offset after the raw or byte string whose opening quote is at `quote`, closed by a
 * quote and `hashes` `#`s. A byte string ends at its line and takes `\` escapes; a raw
 * string may span lines.
 */
pub(super) fn foreign_end(
    b: &[u8],
    quote: usize,
    len: usize,
    raw: bool,
    hashes: usize,
) -> Option<usize> {
    /* One with no closing quote ends at its line, so it takes no more than that. */
    let rest = b.get(quote + 1..len)?;
    let line_end = quote
        + 1
        + rest
            .iter()
            .position(|c| *c == b'\n' || *c == b'\r')
            .unwrap_or(rest.len());
    let mut i = quote + 1;
    let end = loop {
        match b.get(i) {
            None => break line_end,
            _ if i >= len => break line_end,
            Some(b'\n' | b'\r') if !raw => break i,
            Some(b'\\') if !raw => i += 2,
            Some(b'"')
                if b.get(i + 1..i + 1 + hashes)
                    .is_some_and(|h| h.iter().all(|c| *c == b'#')) =>
            {
                break i + 1 + hashes
            }
            Some(_) => i += 1,
        }
    };
    Some(end)
}
