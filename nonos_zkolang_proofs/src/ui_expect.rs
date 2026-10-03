/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The diagnostics a `.zkl` test expects. A comment `/*~ E0100 */` on a line says one
 * diagnostic with that code starts on that line; several codes may share one comment, and
 * a line may carry several comments. A file with none expects no diagnostic at all.
 */

/** Every (line, code) the source expects, sorted, lines counted from 1. */
pub(crate) fn expected(src: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    for (i, line) in src.lines().enumerate() {
        let mut rest = line;
        while let Some(at) = rest.find("/*~") {
            let tail = &rest[at + 3..];
            let Some(end) = tail.find("*/") else {
                break;
            };
            out.extend(
                tail[..end]
                    .split_whitespace()
                    .map(|c| (i + 1, c.to_string())),
            );
            rest = &tail[end + 2..];
        }
    }
    out.sort();
    out
}
