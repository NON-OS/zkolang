/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A statement as text: one `name value` line for each number a gate pins beside the
 * image, hashes in hex, `head` the words every run of the program begins with.
 */

use super::params::{shape, EXTRA_BLOWUP_BITS, GRIND_BITS, MAX_PROOF_BYTES, QUERIES};
use super::statement::Statement;

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

/** `st` as `name value` lines. */
pub fn text(st: &Statement) -> String {
    let s = shape();
    let head: Vec<String> = st.head.iter().map(u64::to_string).collect();
    let rows: [(&str, String); 15] = [
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
        ("head", head.join(",")),
        ("max_proof_bytes", MAX_PROOF_BYTES.to_string()),
    ];
    rows.iter().map(|(k, v)| format!("{k} {v}\n")).collect()
}
