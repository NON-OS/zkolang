/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `doc`: a crate's reference, made from its doc comments (section 17.2), as Markdown on
 * standard output; `--std` gives the standard library's.
 */

use std::fs;

use nonos_zkolang::compiler::diag::{render, Diagnostics};
use nonos_zkolang::compiler::doc::render_doc;
use nonos_zkolang::compiler::source::SourceMap;
use nonos_zkolang::compiler::std_crate::load_std;
use nonos_zkolang::compiler::syntax::load::load;

use super::disk::Disk;
use crate::line::Line;

const USAGE: &str = "usage: zkolang doc <file> | zkolang doc --std";

/** Print the reference of the crate `args` names, or of `std`. */
pub(crate) fn doc(args: &[String]) -> Result<(), String> {
    let (mut map, mut diags) = (SourceMap::new(), Diagnostics::new());
    if matches!(args, [flag] if flag == "--std") {
        let std = load_std(&mut map, &mut diags);
        print!("{}", render_doc(&map, "std", &std));
        return Ok(());
    }
    let line = Line::parse(args, &[], USAGE)?;
    let src = fs::read_to_string(line.file).map_err(|e| format!("read {}: {e}", line.file))?;
    let ast = load(&Disk, (line.file, src), &mut map, &mut diags);
    if diags.has_errors() {
        diags
            .items()
            .iter()
            .for_each(|d| eprint!("{}", render(&map, d)));
        return Err(format!(
            "{}: it does not parse; no reference was made",
            line.file
        ));
    }
    let stem = std::path::Path::new(line.file).file_stem();
    let name = stem.map_or_else(
        || String::from("crate"),
        |s| s.to_string_lossy().into_owned(),
    );
    print!("{}", render_doc(&map, &name, &ast));
    Ok(())
}
