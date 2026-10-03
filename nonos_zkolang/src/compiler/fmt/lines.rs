/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A file's lines as the formatter reads them: where each starts and ends, what it starts
 * with, its first and last tokens, and how many brackets are open before its first token.
 */

use alloc::vec::Vec;

use super::brackets::{closes, Open};
use super::line::{Line, Start};
use crate::compiler::syntax::lex::Lexed;
use crate::compiler::syntax::TokenKind;

/** A token, with its kind, or a comment, without: its bytes `lo..hi`. */
type Piece = (usize, usize, Option<TokenKind>);

/** The lines of `text`, whose tokens and comments are `lexed`, in one pass over them. */
pub(super) fn lines(text: &str, lexed: &Lexed) -> Vec<Line> {
    let tokens = lexed.tokens.iter().filter(|t| t.kind != TokenKind::Eof);
    let mut pieces: Vec<Piece> = tokens
        .map(|t| (t.span.lo as usize, t.span.hi as usize, Some(t.kind)))
        .chain(
            lexed
                .comments
                .iter()
                .map(|c| (c.span.lo as usize, c.span.hi as usize, None)),
        )
        .collect();
    pieces.sort_unstable_by_key(|p| p.0);
    let (mut out, mut lo, mut k, mut open) = (Vec::new(), 0, 0, Open::default());
    /* Pieces do not overlap, so only the last one begun can reach past a line's start. */
    let mut prev: Option<Piece> = None;
    for raw in text.split('\n') {
        let hi = lo + raw.trim_end_matches('\r').len();
        let across = prev.filter(|p| lo < p.1);
        let begin = k;
        while pieces.get(k).is_some_and(|p| p.0 < hi) {
            k += 1;
        }
        let on = pieces.get(begin..k).unwrap_or(&[]);
        prev = on.last().copied().or(prev);
        let start = match (across, on.first()) {
            (Some((_, _, Some(_))), _) => Start::InString,
            (Some(_), _) => Start::InComment,
            (None, Some((_, _, Some(_)))) => Start::Code,
            (None, Some(_)) => Start::Comment,
            (None, None) => Start::Blank,
        };
        let open_end = prev.is_some_and(|p| hi < p.1);
        let code: Vec<TokenKind> = on.iter().filter_map(|p| p.2).collect();
        let (first, last) = (code.first().copied(), code.last().copied());
        let lead = code.iter().take_while(|k| closes(Some(**k))).count();
        let (depth, base) = (open.levels(0), open.levels(lead));
        out.push(Line {
            lo,
            hi,
            start,
            first,
            last,
            depth,
            base,
            open_end,
        });
        open.take(out.len(), &code);
        lo += raw.len() + 1;
    }
    out
}
