/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The scanner: one pass over the bytes, dispatching on the first byte of each token. It is
 * total: any text produces a token stream ending in `Eof`, with every problem reported. A
 * malformed literal becomes an `Error` token, and text that stands for nothing is left out
 * with its span kept, so the parser always has something to recover from.
 */

use alloc::vec::Vec;

use super::block_comment::block_comment;
use super::comment::{scan_line_comment, Comment};
use super::dispatch::scan_token;
use crate::compiler::diag::Diagnostics;
use crate::compiler::source::{FileId, Span};
use crate::compiler::syntax::token::{Token, TokenKind};

/** A file's tokens and comments. */
#[derive(Clone, Debug, Default)]
pub struct Lexed {
    pub tokens: Vec<Token>,
    pub comments: Vec<Comment>,
    /** Text reported and left out of the token stream, in order. */
    pub strays: Vec<Span>,
}

/** The longest source file the compiler reads, so every offset fits a span. */
pub const MAX_SOURCE_LEN: usize = (u32::MAX - 1) as usize;

/** Tokenize one file. Problems are pushed to `diags`; the stream always ends in `Eof`. */
pub fn lex(file: FileId, text: &str, diags: &mut Diagnostics) -> Lexed {
    let mut out = Lexed::default();
    let b = text.as_bytes();
    let len = b.len().min(MAX_SOURCE_LEN);
    let span = |lo: usize, hi: usize| Span::new(file, lo as u32, hi as u32);
    /* A byte-order mark some editors write before the first character is not source. */
    let mut i = if text.starts_with('\u{feff}') { 3 } else { 0 };
    while i < len {
        let c = b[i];
        if c == b' ' || c == b'\t' || c == b'\n' || c == b'\r' {
            i += 1;
            continue;
        }
        if c == b'/' && matches!(b.get(i + 1), Some(b'/' | b'*')) {
            let (comment, next) = match b.get(i + 1) {
                Some(b'/') => scan_line_comment(b, i, len, file),
                _ => block_comment(b, i, len, file, diags),
            };
            out.comments.push(comment);
            i = next;
            continue;
        }
        let start = i;
        let (kind, next) = scan_token(text, start, len, file, diags);
        i = next;
        match kind {
            Some(kind) => out.tokens.push(Token {
                kind,
                span: span(start, i),
            }),
            None => out.strays.push(span(start, i)),
        }
    }
    out.tokens.push(Token {
        kind: TokenKind::Eof,
        span: span(len, len),
    });
    out
}
