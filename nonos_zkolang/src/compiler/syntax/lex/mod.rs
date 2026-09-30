/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The lexer: source text to tokens and comments. */

mod block_comment;
mod comment;
mod describe;
mod dispatch;
mod late_quote;
mod lexed;
mod number;
mod number_digits;
#[cfg(test)]
mod number_tests;
mod number_token;
mod punct;
mod punct_one;
mod quote_char;
mod quote_report;
mod raw_ident;
mod reserved_help;
mod scan;
mod stray;
mod stray_report;
mod string;
mod string_report;
mod string_scan;
mod word;

pub use comment::{Comment, CommentKind};
pub use lexed::Lexed;
pub use number::{int_literal, IntLit, IntLitError};
pub use scan::{lex, MAX_SOURCE_LEN};
pub use string::str_literal;
