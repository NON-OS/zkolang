/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The typed input values an edition 2026 command line gives. */

use crate::line::Line;

/**
 * The comma-separated values given for `flag`, one per scalar of the parameters, as signed
 * decimals; none if the flag is absent. A value that is not a decimal integer is refused
 * by name, since dropping it would move every later value onto another parameter.
 */
pub(super) fn values(line: &Line, flag: &str) -> Result<Vec<i128>, String> {
    let list = line.value(flag).unwrap_or("");
    if list.trim().is_empty() {
        return Ok(Vec::new());
    }
    list.split(',')
        .map(|t| {
            let t = t.trim();
            t.parse::<i128>()
                .map_err(|_| format!("{flag}: `{t}` is not a decimal integer"))
        })
        .collect()
}
