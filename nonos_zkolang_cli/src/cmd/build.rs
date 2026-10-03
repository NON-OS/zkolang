/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! `build`: emit a native backend. */

use std::fs;

use nonos_zkolang::compiler::native;
use nonos_zkolang::{to_asm, to_c, to_python};

use super::edition::modern;
use super::modern::built;
use super::prepare::compiled;
use crate::line::Line;

const USAGE: &str =
    "usage: zkolang build <file> [--edition 2025|2026] [--target c|asm|python] [--out f]";

pub(crate) fn build(args: &[String]) -> Result<(), String> {
    let line = Line::parse(args, &["--target", "--out", "--edition"], USAGE)?;
    let target = line.value("--target").unwrap_or("c");
    let out = if modern(&line)? {
        let (_, b) = built(&line)?;
        match target {
            "c" => native::to_c(&b),
            "python" => native::to_python(&b),
            t => {
                return Err(format!(
                    "unknown target {t} for edition 2026, expected c or python"
                ))
            }
        }
    } else {
        let (_, program) = compiled(&line)?;
        match target {
            "c" => to_c(&program),
            "asm" => to_asm(&program),
            "python" => to_python(&program),
            t => return Err(format!("unknown target {t}, expected c, asm, or python")),
        }
    };
    match line.value("--out") {
        Some(path) => {
            fs::write(path, out).map_err(|e| format!("write {path}: {e}"))?;
            println!("wrote {path}");
        }
        None => print!("{out}"),
    }
    Ok(())
}
