/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The formatter's output: each line at its level, four spaces a level, with no trailing
 * whitespace; at most one blank line in a row, none at the start or the end, after an
 * opening brace or before a closing one; the lines of a block comment moved with its
 * first; a string's lines as they are.
 */

use alloc::string::String;

use super::indent::levels;
use super::line::Start;
use super::lines::lines;
use crate::compiler::syntax::lex::Lexed;
use crate::compiler::syntax::TokenKind;

/** `text`, whose tokens and comments are `lexed`, laid out. */
pub(super) fn lay_out(text: &str, lexed: &Lexed) -> String {
    let ls = lines(text, lexed);
    let lv = levels(&ls);
    let (mut out, mut blank, mut shift) = (String::new(), false, 0isize);
    let mut after_open = true;
    for (l, &level) in ls.iter().zip(&lv) {
        let raw = text.get(l.lo..l.hi).unwrap_or("");
        let body = raw.trim_start();
        let lead = raw.len() - body.len();
        let line = match l.start {
            Start::Blank => {
                blank = !after_open;
                continue;
            }
            Start::InString => String::from(raw),
            Start::InComment => moved(raw, shift),
            Start::Code | Start::Comment => {
                let width = level.saturating_mul(4);
                shift = isize::try_from(width)
                    .unwrap_or(isize::MAX)
                    .saturating_sub(isize::try_from(lead).unwrap_or(isize::MAX));
                let mut s = " ".repeat(width);
                s.push_str(body);
                s
            }
        };
        if blank && l.first != Some(TokenKind::RBrace) {
            out.push('\n');
        }
        blank = false;
        let keep_end = l.open_end || l.start == Start::InString;
        out.push_str(if keep_end { &line } else { line.trim_end() });
        out.push('\n');
        after_open = l.last == Some(TokenKind::LBrace) && !l.open_end;
    }
    out
}

/** A line inside a block comment, moved right by `shift` spaces, or left as far as it can. */
fn moved(raw: &str, shift: isize) -> String {
    if let Ok(right) = usize::try_from(shift) {
        let mut s = " ".repeat(right);
        s.push_str(raw.trim_end());
        return s;
    }
    let lead = raw.len() - raw.trim_start_matches(' ').len();
    let cut = lead.min(shift.unsigned_abs());
    String::from(raw.get(cut..).unwrap_or(raw).trim_end())
}
