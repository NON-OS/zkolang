// NONOS Operating System (AGPL-3.0-or-later)
//! Where the generators put their reference vectors.
//!
//! These write JSON the contracts check themselves against, and the contracts
//! live in another repo, so the destination is the caller's to name rather than
//! this crate's to assume. It used to be one person's home directory written
//! into the source, which meant the generators failed anywhere else, including
//! on every runner.
//!
//! Unset means skip the write. The generators assert on what they produced
//! either way, so skipping loses the file, not the check.

pub(crate) fn write_spec(name: &str, body: &str) -> bool {
    match std::env::var("NOX_SPEC_DIR") {
        Ok(dir) if !dir.is_empty() => {
            let path = std::path::Path::new(&dir).join(name);
            std::fs::write(&path, body)
                .unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
            std::println!("wrote {} bytes to {}", body.len(), path.display());
            true
        }
        _ => {
            std::println!("NOX_SPEC_DIR unset, not writing {name}");
            false
        }
    }
}
