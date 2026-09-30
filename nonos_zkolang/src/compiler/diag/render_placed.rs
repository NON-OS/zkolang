/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! A label with the lines and columns it covers resolved against its file. */

use super::diagnostic::Label;
use crate::compiler::source::{SourceFile, SourceMap};

/** A label and where it is. */
pub(super) struct Placed<'a> {
    pub(super) label: &'a Label,
    pub(super) file: &'a SourceFile,
    /** The one-based line and column the label starts at. */
    pub(super) line: usize,
    pub(super) col: usize,
    /** The line holding the label's last character; `line` for an empty label. */
    pub(super) end_line: usize,
}

impl<'a> Placed<'a> {
    /** Resolve `label`, or `None` if its file is not in the map. */
    pub(super) fn new(map: &'a SourceMap, label: &'a Label) -> Option<Placed<'a>> {
        let file = map.file(label.span.file)?;
        let (line, col) = file.line_col(label.span.lo);
        let last = label.span.hi.saturating_sub(1).max(label.span.lo);
        let (end_line, _) = file.line_col(last);
        Some(Placed {
            label,
            file,
            line,
            col,
            end_line: end_line.max(line),
        })
    }

    /** Whether the label covers more than one line. */
    pub(super) fn is_multiline(&self) -> bool {
        self.end_line > self.line
    }

    /** The label's span as byte offsets into line `line`'s shown text. */
    pub(super) fn within(&self, line: usize) -> (usize, usize) {
        let off = self.file.line_offset(line) as usize;
        let lo = (self.label.span.lo as usize).saturating_sub(off);
        let hi = (self.label.span.hi as usize).saturating_sub(off);
        (lo, hi)
    }

    /** The mark drawn under the label: `^` for the primary label, `-` for the others. */
    pub(super) fn mark(&self) -> char {
        if self.label.primary {
            '^'
        } else {
            '-'
        }
    }
}
