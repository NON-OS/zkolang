/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * `zkolang fmt`: a file laid out by line, its tokens and comments kept. A file that does
 * not lex and parse cleanly is not formatted, and a layout that would change a token or a
 * comment is refused, a fault of the formatter, rather than written.
 */

use alloc::string::String;

use super::equiv::same;
use super::layout::lay_out;
use crate::compiler::diag::Diagnostics;
use crate::compiler::source::FileId;
use crate::compiler::syntax::lex::lex;
use crate::compiler::syntax::parse::parse_file;

/** Why a file was not formatted. */
#[derive(Debug)]
pub enum FmtError {
    /** The file does not lex or parse; the diagnostics say why. */
    Syntax(Diagnostics),
    /** The layout would have changed a token or a comment: a fault of the formatter. */
    Changed,
}

/** `text`, formatted. */
pub fn format(text: &str) -> Result<String, FmtError> {
    let mut diags = Diagnostics::new();
    let lexed = lex(FileId(0), text, &mut diags);
    parse_file(FileId(0), text, &lexed, &mut diags, &mut 0);
    if diags.has_errors() || !lexed.strays.is_empty() {
        return Err(FmtError::Syntax(diags));
    }
    let out = lay_out(text, &lexed);
    let mut again = Diagnostics::new();
    let relexed = lex(FileId(0), &out, &mut again);
    match !again.has_errors() && same(text, &lexed, &out, &relexed) {
        true => Ok(out),
        false => Err(FmtError::Changed),
    }
}
