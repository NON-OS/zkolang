/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * What the edition 2026 commands share: the program built from the file the command line
 * names and the module files it declares, its diagnostics rendered against the sources,
 * and typed input values.
 */

use std::fs;

use nonos_zkolang::compiler::diag::{render, to_json};
use nonos_zkolang::compiler::driver::{build, Built, Source};
use nonos_zkolang::compiler::source::SourceMap;

use super::disk::{manifest_for, Disk};
use crate::line::Line;

/**
 * The program `line` names, built, and its source map. Its warnings are printed to
 * standard error, or with `--json` as one JSON array on standard output; if it does not
 * build, every diagnostic is, and the error counts them.
 */
pub(super) fn built(line: &Line) -> Result<(SourceMap, Built), String> {
    let src = fs::read_to_string(line.file).map_err(|e| format!("read {}: {e}", line.file))?;
    let mut map = SourceMap::new();
    let manifest = manifest_for(line.file);
    let manifest = manifest.as_ref().map(|(p, t)| (p.as_str(), t.clone()));
    let src = Source {
        root: (line.file, src),
        manifest,
    };
    let (built, diags) = match build(&mut map, &Disk, src) {
        Ok(b) => {
            let warnings = b.warnings.clone();
            (Some(b), warnings)
        }
        Err(d) => (None, d),
    };
    if line.switch("--json") {
        println!("{}", to_json(&map, diags.items()));
    } else {
        diags
            .items()
            .iter()
            .for_each(|d| eprint!("{}", render(&map, d)));
    }
    let errors = diags.items().iter().filter(|d| d.is_error()).count();
    match built {
        Some(b) => Ok((map, b)),
        None => {
            let errors = if errors == 1 {
                String::from("1 error")
            } else {
                format!("{errors} errors")
            };
            Err(format!("{}: {errors}; nothing was compiled", line.file))
        }
    }
}
