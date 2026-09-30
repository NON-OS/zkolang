/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The integer literal token: its text is checked and a malformed one reported. */

use alloc::format;
use alloc::string::String;

use super::number::{int_literal, IntLitError};
use crate::compiler::diag::{Code, Diagnostic, Diagnostics};
use crate::compiler::source::Span;
use crate::compiler::syntax::token::TokenKind;

/** Validate an integer literal, reporting a malformed one. */
pub(super) fn number(w: &str, span: Span, diags: &mut Diagnostics) -> TokenKind {
    let Err(e) = int_literal(w) else {
        return TokenKind::Int;
    };
    let (prefix, radix) = match w.get(..2) {
        Some("0x") => ("0x", 16),
        Some("0o") => ("0o", 8),
        Some("0b") => ("0b", 2),
        _ => ("", 10),
    };
    let (code, message, label, help) = match e {
        IntLitError::NoDigits => (
            Code::BAD_NUMBER,
            "missing digits after the radix prefix",
            format!("no digit right after `{prefix}`"),
            None,
        ),
        IntLitError::BadDigit => {
            let bad = w[prefix.len()..]
                .chars()
                .find(|c| c.to_digit(10).is_some_and(|d| d >= radix));
            let label = match bad {
                Some(d) => format!("`{d}` is not a digit in base {radix}"),
                None => format!("a digit here is not a digit in base {radix}"),
            };
            (
                Code::BAD_NUMBER,
                "digit out of range for the radix",
                label,
                None,
            )
        }
        IntLitError::BadSuffix => (
            Code::BAD_NUMBER,
            "unknown literal suffix",
            String::from("the suffix names no integer type"),
            Some("a suffix names an integer type: u8 u16 u32 u64 i8 i16 i32 i64 usize"),
        ),
        IntLitError::TooLarge => (
            Code::NUMBER_TOO_LARGE,
            "integer literal too large",
            String::from("larger than 2^64 - 1"),
            Some("the largest literal is 18446744073709551615"),
        ),
    };
    let d = Diagnostic::error(code, message, span, label);
    diags.push(match help {
        Some(h) => d.with_help(h),
        None => d,
    });
    TokenKind::Error
}
