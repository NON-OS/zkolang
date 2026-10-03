/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * What the format 7 commands share: the statement written as a gate pins it, and the
 * program image beside it.
 */

use std::fs;
use std::path::Path;

use nonos_zkolang_format7::{text, Statement};

use super::edition::modern;
use crate::line::Line;

/** Write `st` to `dir` as `program.bin`, the image, and `statement.txt`. */
pub(super) fn write(dir: &str, st: &Statement) -> Result<(), String> {
    let put = |name: &str, bytes: &[u8]| {
        let path = Path::new(dir).join(name);
        fs::write(&path, bytes).map_err(|e| format!("write {}: {e}", path.display()))
    };
    fs::create_dir_all(dir).map_err(|e| format!("create {dir}: {e}"))?;
    put("program.bin", &st.image)?;
    put("statement.txt", text(st).as_bytes())
}

/** That `line` compiles edition 2026, the only edition with a format 7 statement. */
pub(super) fn needs_2026(line: &Line) -> Result<(), String> {
    match modern(line)? {
        true => Ok(()),
        false => Err(String::from(
            "format 7 statements are of edition 2026; pass --edition 2026",
        )),
    }
}
