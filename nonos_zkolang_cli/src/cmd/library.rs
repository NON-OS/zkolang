/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `check` of a crate with no `fn main`, a library: its items are checked and nothing is
 * built, since nothing in it runs.
 */

use nonos_zkolang::compiler::driver::check;
use nonos_zkolang::compiler::source::SourceMap;

use super::disk::Disk;
use super::modern::{load, show, source};
use crate::line::Line;
use crate::out::paint;

/**
 * Check the program `line` names: `true` once a library has been checked and reported,
 * `false` for a program with a `main`, which the caller builds.
 */
pub(super) fn library(line: &Line) -> Result<bool, String> {
    let loaded = load(line)?;
    let mut map = SourceMap::new();
    let (has_main, warnings) = match check(&mut map, &Disk, source(line, &loaded)) {
        Ok(checked) => checked,
        Err(d) => return show(line, &map, &d).map(|()| true),
    };
    if has_main {
        return Ok(false);
    }
    show(line, &map, &warnings)?;
    if !line.switch("--json") {
        println!(
            "{}  a library: no `fn main`, so nothing to run",
            paint("ok", "1;32")
        );
    }
    Ok(true)
}
