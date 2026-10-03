/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `fmt`: lay out an edition 2026 file in place, its tokens and comments kept; with
 * `--check`, only report whether it is laid out already.
 */

use nonos_zkolang::compiler::diag::render;
use nonos_zkolang::compiler::fmt::{format, FmtError};
use nonos_zkolang::compiler::source::SourceMap;

use crate::line::Line;

const USAGE: &str = "usage: zkolang fmt <file> [--check]";

/** Lay out the file `args` names, or check that it is laid out. */
pub(crate) fn fmt(args: &[String]) -> Result<(), String> {
    let line = Line::parse_with(args, (&[], &["--check"]), USAGE)?;
    let path = line.file;
    let text = std::fs::read_to_string(path).map_err(|e| format!("{path}: {e}"))?;
    let out = match format(&text) {
        Ok(out) => out,
        Err(FmtError::Syntax(diags)) => {
            let mut map = SourceMap::new();
            map.add(path.to_string(), text);
            diags
                .items()
                .iter()
                .for_each(|d| eprint!("{}", render(&map, d)));
            return Err(format!("{path}: not formatted, since it does not parse"));
        }
        Err(FmtError::Changed) => {
            return Err(format!(
                "{path}: not formatted, since the layout would change a token or a comment"
            ))
        }
    };
    if out == text {
        println!("{path}: formatted");
    } else if line.switch("--check") {
        return Err(format!(
            "{path}: not formatted; `zkolang fmt {path}` lays it out"
        ));
    } else {
        std::fs::write(path, out).map_err(|e| format!("{path}: {e}"))?;
        println!("{path}: laid out");
    }
    Ok(())
}
