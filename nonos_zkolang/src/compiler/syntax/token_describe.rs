/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! How a diagnostic names each kind of token. */

use super::token_kind::TokenKind;

impl TokenKind {
    /** How a diagnostic names this kind of token. */
    pub fn describe(self) -> &'static str {
        use TokenKind::*;
        match self {
            Ident => "an identifier",
            Int => "an integer literal",
            Str => "a string literal",
            Kw(k) => k.as_str(),
            Underscore => "`_`",
            Plus => "`+`",
            Minus => "`-`",
            Star => "`*`",
            Slash => "`/`",
            Percent => "`%`",
            Caret => "`^`",
            Bang => "`!`",
            Amp => "`&`",
            Pipe => "`|`",
            AmpAmp => "`&&`",
            PipePipe => "`||`",
            Shl => "`<<`",
            Shr => "`>>`",
            PlusEq => "`+=`",
            MinusEq => "`-=`",
            StarEq => "`*=`",
            SlashEq => "`/=`",
            PercentEq => "`%=`",
            CaretEq => "`^=`",
            AmpEq => "`&=`",
            PipeEq => "`|=`",
            ShlEq => "`<<=`",
            ShrEq => "`>>=`",
            Eq => "`=`",
            EqEq => "`==`",
            Ne => "`!=`",
            Lt => "`<`",
            Gt => "`>`",
            Le => "`<=`",
            Ge => "`>=`",
            Dot => "`.`",
            DotDot => "`..`",
            DotDotEq => "`..=`",
            Comma => "`,`",
            Semi => "`;`",
            Colon => "`:`",
            ColonColon => "`::`",
            Arrow => "`->`",
            FatArrow => "`=>`",
            Pound => "`#`",
            LParen => "`(`",
            RParen => "`)`",
            LBracket => "`[`",
            RBracket => "`]`",
            LBrace => "`{`",
            RBrace => "`}`",
            Error => "invalid text",
            Eof => "the end of the file",
        }
    }
}
