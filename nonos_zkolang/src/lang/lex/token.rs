/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The token alphabet. The set is small on purpose: a handful of keywords, the
//! arithmetic and comparison operators, and the punctuation the grammar needs.
//! Everything else the source could contain is a lexing error, not a silent skip.

use alloc::string::String;

// One lexical token.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Tok {
    // Keywords. These are the words the grammar reserves; an identifier can never
    // be one of them, because the ident branch of the scanner matches them first.
    Let,
    Assert,
    Input,
    Secret,
    Output,
    Inv,
    Sel,
    For,
    In,
    If,
    Else,
    Fn,
    Const,
    // A user name and a decimal literal, the two tokens that carry a value.
    Ident(String),
    Num(u64),
    // Arithmetic operators. `Slash` is field division, which lowers to a multiply
    // by an inverse, so dividing by zero is unprovable rather than a crash.
    Plus,
    Minus,
    Star,
    Slash,
    // Comparison and assignment. `Assign` is the `=` of a `let`; `EqEq` and
    // `BangEq` are the `==` and `!=` that produce a zero or one bit.
    Assign,
    EqEq,
    BangEq,
    // Punctuation. `DotDot` is the range in a `for`, and the braces delimit its
    // body.
    LParen,
    RParen,
    Comma,
    Semi,
    DotDot,
    LBrace,
    RBrace,
    // The brackets that delimit a constant table and index into it.
    LBracket,
    RBracket,
}
