/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Which edition a command compiles: 2025, the frozen compiler, unless `--edition 2026`. */

use crate::line::Line;

/** Whether the command line asks for edition 2026. */
pub(super) fn modern(line: &Line) -> Result<bool, String> {
    match line.value("--edition") {
        None | Some("2025") => Ok(false),
        Some("2026") => Ok(true),
        Some(e) => Err(format!(
            "--edition: `{e}` is not an edition; 2025 and 2026 are"
        )),
    }
}
