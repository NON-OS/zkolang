// NONOS Operating System (AGPL-3.0-or-later)
//! A content-addressed cache for the outer periodic root.
//!
//! The structure emit spends 97 to 99 per cent of its runtime in one call.
//! Committing the outer periodic tree at the deployment rate took 3 h 35 m at
//! the settlement point and 9 h 21 m at e64, against about three minutes for
//! every other field in both files put together. So a change to any piece of
//! metadata cost most of a day of Keccak to republish a number that had not
//! moved, and one of the three re-runs this cost us was pure waste.
//!
//! The key is the structure file's own bytes. That file is written before the
//! root is computed and does not contain it, and it carries
//! `periodic_root_poseidon`, which is a commitment to the periodic columns
//! themselves. So the digest moves when the columns move, when the domain
//! moves, when the rate moves and when the coset shift moves, which is the
//! complete set of things the root depends on. Nothing else has to be listed,
//! and nothing can be forgotten from a list that does not exist.
//!
//! A miss is therefore the thing that says the root is stale, rather than
//! somebody remembering that it might be.

use crate::crypto::stark::hash::keccak256;
use std::path::{Path, PathBuf};

/// Keccak-256 over the structure file, rendered hex. The same hash the rest of
/// the settlement path uses, so this adds no dependency and no second opinion
/// about what hashing means here.
pub fn digest(structure_json: &[u8]) -> String {
    keccak256(structure_json)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// Where an entry lives. Beside the emissions rather than in a temp directory,
/// because a cache that does not survive a reboot is not a cache for a job
/// measured in hours.
fn entry(dir: &Path, key: &str) -> PathBuf {
    dir.join(format!("{key}.root"))
}

pub fn cache_dir(out: &str) -> PathBuf {
    Path::new(out)
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(".rootcache")
}

/// The stored root for this structure, if one exists.
pub fn get(dir: &Path, key: &str) -> Option<String> {
    let hex = std::fs::read_to_string(entry(dir, key)).ok()?;
    let hex = hex.trim().to_ascii_lowercase();
    valid(&hex).then_some(hex)
}

/// Store a root against this structure. Refuses anything that is not 32 bytes
/// of hex, because a cache entry is a constant a contract gets baked with and
/// a malformed one would be read back as authoritative.
pub fn put(dir: &Path, key: &str, root_hex: &str) -> std::io::Result<()> {
    let hex = root_hex.trim().to_ascii_lowercase();
    if !valid(&hex) {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!(
                "a periodic root must be 64 hex characters, got {}",
                hex.len()
            ),
        ));
    }
    std::fs::create_dir_all(dir)?;
    std::fs::write(entry(dir, key), format!("{hex}\n"))
}

fn valid(hex: &str) -> bool {
    hex.len() == 64 && hex.bytes().all(|b| b.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The key must move when the structure moves and must not otherwise, or
    /// the cache either never hits or hits when it should not. The second is
    /// the dangerous direction: it hands back a root for a circuit that no
    /// longer exists, and a contract baked with it refuses every proof.
    #[test]
    fn the_key_tracks_the_structure() {
        let a = br#"{"point":"settlement","trace_width":749}"#;
        let b = br#"{"point":"settlement","trace_width":747}"#;
        assert_eq!(digest(a), digest(a), "the same structure must key the same");
        assert_ne!(
            digest(a),
            digest(b),
            "a different shape must key differently"
        );
        assert_eq!(digest(a).len(), 64);
    }

    #[test]
    fn a_stored_root_comes_back_and_a_malformed_one_is_refused() {
        let dir = std::env::temp_dir().join(format!("rootcache-{}", std::process::id()));
        let key = digest(b"{}");
        let root = "f1b9a6309e1644949edbb0b086c5e303f328cca525b7c8a9aced9c77c802b383";
        put(&dir, &key, root).expect("store");
        assert_eq!(get(&dir, &key).as_deref(), Some(root));
        assert!(get(&dir, "nothing-stored-here").is_none());
        assert!(
            put(&dir, "short", "abcd").is_err(),
            "a malformed root must be refused rather than stored"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }
}
