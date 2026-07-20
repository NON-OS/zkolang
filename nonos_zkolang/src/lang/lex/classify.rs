/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Character classes for identifiers.

/// The bytes that may start an identifier.
pub(super) fn is_ident_start(b: u8) -> bool {
    b.is_ascii_alphabetic() || b == b'_'
}

/// The bytes that may continue an identifier.
pub(super) fn is_ident_continue(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_'
}
