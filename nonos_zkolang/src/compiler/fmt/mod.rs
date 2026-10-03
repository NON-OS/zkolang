/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The formatter: a file's lines laid out, its tokens and comments kept. */

mod brackets;
mod equiv;
mod format;
mod indent;
mod layout;
mod line;
mod lines;

pub use format::{format, FmtError};
