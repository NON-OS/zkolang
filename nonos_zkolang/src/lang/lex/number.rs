/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Scan a decimal literal.

use super::token::Tok;
use crate::lang::CompileError;

/// Read a decimal literal from `start`, returning the number token and the index
/// just past it. A literal too large for the field's 64-bit representative is a typed
/// error at the literal's offset.
pub(super) fn scan_number(b: &[u8], start: usize) -> Result<(Tok, usize), CompileError> {
    let mut i = start;
    let mut value: u64 = 0;
    while i < b.len() && b[i].is_ascii_digit() {
        let digit = (b[i] - b'0') as u64;
        value = value
            .checked_mul(10)
            .and_then(|v| v.checked_add(digit))
            .ok_or(CompileError::NumberTooLarge { at: start })?;
        i += 1;
    }
    Ok((Tok::Num(value), i))
}
