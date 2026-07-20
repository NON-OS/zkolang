/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

//! The lexical layer: the token alphabet and the scanner that produces it. The
//! scanning is split by category, a dispatch loop over the character classifier, the
//! word and number readers, and the symbol reader.

mod classify;
mod number;
mod scan;
mod symbol;
mod token;
mod word;

pub use scan::lex;
pub use token::Tok;
