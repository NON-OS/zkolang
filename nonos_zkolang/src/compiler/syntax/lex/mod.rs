/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The lexer: source text to tokens and comments. */

mod block_comment;
mod comment;
mod describe;
mod dispatch;
mod number;
mod number_digits;
#[cfg(test)]
mod number_tests;
mod number_token;
mod punct;
mod punct_one;
mod quote_char;
mod scan;
mod stray;
mod stray_report;
mod string;
mod string_report;
mod string_scan;
mod word;

pub use comment::{Comment, CommentKind};
pub use number::{int_literal, IntLit, IntLitError};
pub use scan::{lex, Lexed, MAX_SOURCE_LEN};
pub use string::str_literal;
