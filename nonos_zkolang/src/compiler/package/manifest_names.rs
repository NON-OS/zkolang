/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The forms a manifest's names and versions take. */

use alloc::format;
use alloc::string::String;

use crate::compiler::syntax::{Keyword, RESERVED};

/** Whether `s` may name a crate: an identifier, no keyword, and neither `std` nor `crate`. */
pub(super) fn crate_name(s: &str) -> Result<(), String> {
    let mut chars = s.chars();
    let first = chars
        .next()
        .is_some_and(|c| c.is_ascii_alphabetic() || c == '_');
    let rest = chars.all(|c| c.is_ascii_alphanumeric() || c == '_');
    if !first || !rest || s == "_" {
        return Err(format!("`{s}` is not an identifier"));
    }
    if s == "std" || Keyword::from_word(s).is_some() || RESERVED.contains(&s) {
        return Err(format!("`{s}` is reserved and names no crate"));
    }
    Ok(())
}

/** Whether `s` is a version `major.minor.patch`, each part decimal digits. */
pub(super) fn version(s: &str) -> Result<(), String> {
    let parts: alloc::vec::Vec<&str> = s.split('.').collect();
    let digits = |p: &&str| !p.is_empty() && p.bytes().all(|b| b.is_ascii_digit());
    if parts.len() == 3 && parts.iter().all(digits) {
        Ok(())
    } else {
        Err(format!("`{s}` is not a version `major.minor.patch`"))
    }
}

/** The edition `s` names, `"2025"` or `"2026"`. */
pub(super) fn edition(s: &str) -> Result<u16, String> {
    match s {
        "2025" => Ok(2025),
        "2026" => Ok(2026),
        _ => Err(format!("`{s}` is no edition; \"2025\" and \"2026\" are")),
    }
}
