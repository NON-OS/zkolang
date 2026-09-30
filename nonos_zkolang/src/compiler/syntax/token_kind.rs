/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The kinds of token the lexer produces. */

use super::keyword::Keyword;

/** What a token is. */
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TokenKind {
    Ident,
    /** An integer literal, validated by the lexer; its value is read by `int_literal`. */
    Int,
    /** A string literal, validated by the lexer; its value is read by `str_literal`. */
    Str,
    Kw(Keyword),
    /** The lone wildcard `_`. */
    Underscore,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Bang,
    Amp,
    Pipe,
    AmpAmp,
    PipePipe,
    Shl,
    Shr,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    CaretEq,
    AmpEq,
    PipeEq,
    ShlEq,
    ShrEq,
    Eq,
    EqEq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    Dot,
    DotDot,
    DotDotEq,
    Comma,
    Semi,
    Colon,
    ColonColon,
    Arrow,
    FatArrow,
    Pound,
    LParen,
    RParen,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    /** Text that begins no token. The lexer has already reported it. */
    Error,
    Eof,
}
