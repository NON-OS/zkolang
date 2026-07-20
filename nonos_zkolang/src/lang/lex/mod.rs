/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The lexical layer: the token alphabet and the scanner that produces it. The
//! token type lives apart from the scanner so the parser can name tokens without
//! depending on the scanning loop.

mod scan;
mod token;

pub use scan::lex;
pub use token::Tok;
