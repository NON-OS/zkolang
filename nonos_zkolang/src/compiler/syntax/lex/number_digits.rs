/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The digits of an integer literal, after its radix prefix. */

use super::number::IntLitError;

/**
 * Read the radix prefix and digits at the start of a literal's bytes. Returns their value
 * and the offset of whatever follows them, which is the suffix.
 */
pub(super) fn read_digits(bytes: &[u8]) -> Result<(u64, usize), IntLitError> {
    let (radix, mut i) = match bytes {
        [b'0', b'x', ..] => (16u64, 2),
        [b'0', b'o', ..] => (8, 2),
        [b'0', b'b', ..] => (2, 2),
        _ => (10, 0),
    };
    /* A radix prefix is followed by a digit: `0x_1` is malformed (spec section 2.3). */
    if radix != 10 && bytes.get(2) == Some(&b'_') {
        return Err(IntLitError::NoDigits);
    }
    let mut value: u64 = 0;
    let mut digits = 0usize;
    while i < bytes.len() {
        let c = bytes[i];
        if c == b'_' {
            i += 1;
            continue;
        }
        let d = match c {
            b'0'..=b'9' => u64::from(c - b'0'),
            b'a'..=b'f' if radix == 16 => u64::from(c - b'a') + 10,
            b'A'..=b'F' if radix == 16 => u64::from(c - b'A') + 10,
            _ => break,
        };
        if d >= radix {
            return Err(IntLitError::BadDigit);
        }
        value = value
            .checked_mul(radix)
            .and_then(|v| v.checked_add(d))
            .ok_or(IntLitError::TooLarge)?;
        digits += 1;
        i += 1;
    }
    if digits == 0 {
        return Err(IntLitError::NoDigits);
    }
    Ok((value, i))
}
