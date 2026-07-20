/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! Scan a word into a keyword or an identifier.

use alloc::string::String;

use super::classify::is_ident_continue;
use super::token::Tok;

/// Read an identifier from `start`, returning the keyword it spells or an identifier
/// token, and the index just past it. Keywords are matched here, so an identifier can
/// never be one of them.
pub(super) fn scan_word(src: &str, b: &[u8], start: usize) -> (Tok, usize) {
    let mut i = start;
    while i < b.len() && is_ident_continue(b[i]) {
        i += 1;
    }
    let tok = match &src[start..i] {
        "let" => Tok::Let,
        "assert" | "prove" => Tok::Assert,
        "input" | "public" => Tok::Input,
        "secret" | "witness" => Tok::Secret,
        "output" | "reveal" => Tok::Output,
        "inv" => Tok::Inv,
        "sel" => Tok::Sel,
        "for" => Tok::For,
        "in" => Tok::In,
        "if" => Tok::If,
        "else" => Tok::Else,
        "fn" => Tok::Fn,
        "const" => Tok::Const,
        "match" => Tok::Match,
        w => Tok::Ident(String::from(w)),
    };
    (tok, i)
}
