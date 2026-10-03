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

use nonos_zkolang::compiler::diag::{render, to_json, Diagnostics};
use nonos_zkolang::compiler::driver::{build, Built, Source};
use nonos_zkolang::compiler::source::SourceMap;

use super::disk::{manifest_for, Disk};
use crate::line::Line;

/** The text of the file `line` names, and the manifest that governs it, if one does. */
pub(super) type Loaded = (String, Option<(String, String)>);

/** Read the file `line` names and find the manifest that governs it. */
pub(super) fn load(line: &Line) -> Result<Loaded, String> {
    let src = fs::read_to_string(line.file).map_err(|e| format!("read {}: {e}", line.file))?;
    Ok((src, manifest_for(line.file)))
}

/** The program of `loaded`, rooted at the file `line` names. */
pub(super) fn source<'a>(line: &Line<'a>, loaded: &'a Loaded) -> Source<'a> {
    Source {
        root: (line.file, loaded.0.clone()),
        manifest: loaded.1.as_ref().map(|(p, t)| (p.as_str(), t.clone())),
    }
}

/**
 * Print `diags`, rendered to standard error, or with `--json` as one JSON array on standard
 * output; the error, naming how many there were, if any was an error.
 */
pub(super) fn show(line: &Line, map: &SourceMap, diags: &Diagnostics) -> Result<(), String> {
    if line.switch("--json") {
        println!("{}", to_json(map, diags.items()));
    } else {
        diags
            .items()
            .iter()
            .for_each(|d| eprint!("{}", render(map, d)));
    }
    match diags.items().iter().filter(|d| d.is_error()).count() {
        0 => Ok(()),
        1 => Err(format!("{}: 1 error; nothing was compiled", line.file)),
        n => Err(format!("{}: {n} errors; nothing was compiled", line.file)),
    }
}

/**
 * The program `line` names, built, and its source map. Its warnings are printed, and if it
 * does not build every diagnostic is, and the error counts them.
 */
pub(super) fn built(line: &Line) -> Result<(SourceMap, Built), String> {
    let loaded = load(line)?;
    let mut map = SourceMap::new();
    match build(&mut map, &Disk, source(line, &loaded)) {
        Ok(b) => {
            show(line, &map, &b.warnings)?;
            Ok((map, b))
        }
        Err(d) => {
            show(line, &map, &d)?;
            Err(format!("{}: nothing was compiled", line.file))
        }
    }
}
