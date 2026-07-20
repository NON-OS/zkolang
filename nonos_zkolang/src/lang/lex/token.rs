/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The token alphabet: keywords, operators, punctuation, an identifier, a literal.

use alloc::string::String;

/// One lexical token.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Tok {
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
    Ident(String),
    Num(u64),
    Plus,
    Minus,
    Star,
    Slash,
    Assign,
    EqEq,
    BangEq,
    Bang,
    AmpAmp,
    PipePipe,
    LParen,
    RParen,
    Comma,
    Semi,
    DotDot,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
}
