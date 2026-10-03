/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Integer literals: `123`, `1_000`, `0xff`, `0o17`, `0b1010`, each with an optional type
 * suffix such as `u8` or `usize`. The lexer validates a literal's shape and the parser
 * reads its value through the same function, so the two cannot disagree.
 */

use super::number_digits::read_digits;
use crate::compiler::syntax::int_ty::IntTy;

/** A literal's value and suffix. The value always fits in 64 bits. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct IntLit {
    pub value: u64,
    pub suffix: Option<IntTy>,
}

/** Why a literal's text is malformed. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum IntLitError {
    /** A radix prefix with no digits after it. */
    NoDigits,
    /** A digit the radix does not have. */
    BadDigit,
    /** Letters after the digits that are not a type suffix. */
    BadSuffix,
    /** A value past 2^64 - 1. */
    TooLarge,
}

/** Read an integer literal's text: the word the lexer took, digits and suffix together. */
pub fn int_literal(text: &str) -> Result<IntLit, IntLitError> {
    let (value, i) = read_digits(text.as_bytes())?;
    let rest = &text[i..];
    let suffix = if rest.is_empty() {
        None
    } else {
        match IntTy::from_name(rest) {
            Some(t) => Some(t),
            None => {
                return Err(if rest.bytes().all(|b| b.is_ascii_digit() || b == b'_') {
                    IntLitError::BadDigit
                } else {
                    IntLitError::BadSuffix
                })
            }
        }
    };
    Ok(IntLit { value, suffix })
}
