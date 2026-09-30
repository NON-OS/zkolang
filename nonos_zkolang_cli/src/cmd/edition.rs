/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Which edition a command compiles: the one `--edition` names, else the one the governing
 * manifest names, else 2025, the frozen compiler.
 */

use nonos_zkolang::compiler::diag::Diagnostics;
use nonos_zkolang::compiler::package::manifest;
use nonos_zkolang::compiler::source::SourceMap;

use super::disk::manifest_for;
use crate::line::Line;

/**
 * Whether the command compiles edition 2026: as `--edition` says, else as the manifest
 * that governs the file says (section 20.1). A manifest that does not parse is taken as
 * edition 2026, whose build reports what is wrong with it.
 */
pub(super) fn modern(line: &Line) -> Result<bool, String> {
    match line.value("--edition") {
        None => Ok(manifest_for(line.file).is_some_and(|(_, text)| {
            let mut map = SourceMap::new();
            let id = map.add(String::from("zkolang.toml"), text.clone());
            manifest(id, &text, &mut Diagnostics::new()).is_none_or(|m| m.edition == 2026)
        })),
        Some("2025") => Ok(false),
        Some("2026") => Ok(true),
        Some(e) => Err(format!(
            "--edition: `{e}` is not an edition; 2025 and 2026 are"
        )),
    }
}
