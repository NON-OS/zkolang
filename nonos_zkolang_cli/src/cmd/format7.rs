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

use nonos_zkolang_format7::{shape, Statement};
use nonos_zkolang_format7::{EXTRA_BLOWUP_BITS, GRIND_BITS, MAX_PROOF_BYTES, QUERIES};

use super::edition::modern;
use crate::line::Line;

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

/** `st` as `name value` lines: every number a gate pins beside the image. */
pub(super) fn text(st: &Statement) -> String {
    let s = shape();
    let rows: [(&str, String); 14] = [
        ("format", String::from("7")),
        ("program_hash", hex(&st.image_hash)),
        ("periodic_root", hex(&st.periodic_root)),
        ("params", hex(&st.params)),
        ("queries", QUERIES.to_string()),
        ("grind_bits", GRIND_BITS.to_string()),
        ("extra_blowup_bits", EXTRA_BLOWUP_BITS.to_string()),
        ("trace_width", s.trace_width.to_string()),
        ("region_width", s.region_width.to_string()),
        ("window", s.window.to_string()),
        ("constraint_degree", s.constraint_degree.to_string()),
        ("challenge_lanes", s.challenge_lanes.to_string()),
        ("words", st.words.to_string()),
        ("max_proof_bytes", MAX_PROOF_BYTES.to_string()),
    ];
    rows.iter().map(|(k, v)| format!("{k} {v}\n")).collect()
}

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
