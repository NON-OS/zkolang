/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The left margin of a rendered diagnostic: line numbers, the `|` rule, and, when a label
 * spans lines, a connector column that draws its extent.
 */

use alloc::format;
use alloc::string::String;

/** The margin every snippet row of one diagnostic shares. */
pub(super) struct Gutter {
    /** One space per digit of the widest line number shown. */
    pub(super) pad: String,
    /** Whether a label spans lines, so every row gets a connector column. */
    pub(super) connectors: bool,
}

impl Gutter {
    /** The margin for a diagnostic whose largest line number is `max_line`. */
    pub(super) fn new(max_line: usize, connectors: bool) -> Gutter {
        Gutter {
            pad: " ".repeat(digits(max_line)),
            connectors,
        }
    }

    /** The connector column of a row no multi-line label passes through. */
    pub(super) fn plain(&self) -> &'static str {
        if self.connectors {
            "  "
        } else {
            ""
        }
    }

    /** A source row: line number `line`, then `conn`, then the shown text. */
    pub(super) fn source(&self, out: &mut String, line: usize, conn: &str, shown: &str) {
        let w = self.pad.len();
        out.push_str(format!("{line:>w$} | {conn}{shown}").trim_end());
        out.push('\n');
    }

    /** A row under a source row: `conn`, then marks and a message. */
    pub(super) fn mark(&self, out: &mut String, conn: &str, marks: &str) {
        out.push_str(format!("{} | {conn}{marks}", self.pad).trim_end());
        out.push('\n');
    }

    /** A row with nothing but the rule. */
    pub(super) fn rule(&self, out: &mut String) {
        out.push_str(&format!("{} |\n", self.pad));
    }
}

/** The number of decimal digits of `n`. */
fn digits(mut n: usize) -> usize {
    let mut d = 1;
    while n >= 10 {
        n /= 10;
        d += 1;
    }
    d
}
