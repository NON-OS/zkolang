/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The integer literal token: its text is checked and a malformed one reported. */

use super::number::{int_literal, IntLitError};
use crate::compiler::diag::{Code, Diagnostic, Diagnostics};
use crate::compiler::source::Span;
use crate::compiler::syntax::token::TokenKind;

/** Validate an integer literal, reporting a malformed one. */
pub(super) fn number(w: &str, span: Span, diags: &mut Diagnostics) -> TokenKind {
    match int_literal(w) {
        Ok(_) => TokenKind::Int,
        Err(e) => {
            let (code, msg, help) = match e {
                IntLitError::NoDigits => (
                    Code::BAD_NUMBER,
                    "missing digits after the radix prefix",
                    None,
                ),
                IntLitError::BadDigit => {
                    (Code::BAD_NUMBER, "digit out of range for the radix", None)
                }
                IntLitError::BadSuffix => (
                    Code::BAD_NUMBER,
                    "unknown literal suffix",
                    Some("a suffix names an integer type: u8 u16 u32 u64 i8 i16 i32 i64 usize"),
                ),
                IntLitError::TooLarge => (
                    Code::NUMBER_TOO_LARGE,
                    "integer literal too large",
                    Some("the largest literal is 18446744073709551615"),
                ),
            };
            let d = Diagnostic::error(code, msg, span, "");
            diags.push(match help {
                Some(h) => d.with_help(h),
                None => d,
            });
            TokenKind::Error
        }
    }
}
