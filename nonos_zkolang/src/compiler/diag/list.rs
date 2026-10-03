/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The diagnostics a compilation collects. Stages push into one list and keep going, so a
 * single run reports every independent problem it finds.
 */

use alloc::vec::Vec;

use super::diagnostic::{Diagnostic, Severity};

/** An ordered collection of diagnostics. */
#[derive(Clone, Debug, Default)]
pub struct Diagnostics {
    pub(super) items: Vec<Diagnostic>,
}

impl Diagnostics {
    /** An empty list. */
    pub fn new() -> Diagnostics {
        Diagnostics { items: Vec::new() }
    }

    /** Record a diagnostic. */
    pub fn push(&mut self, d: Diagnostic) {
        self.items.push(d);
    }

    /** Record every diagnostic of another list. */
    pub fn extend(&mut self, other: Diagnostics) {
        self.items.extend(other.items);
    }

    /** The number of errors recorded. */
    pub fn error_count(&self) -> usize {
        self.items
            .iter()
            .filter(|d| d.severity == Severity::Error)
            .count()
    }

    /** Whether any error was recorded. */
    pub fn has_errors(&self) -> bool {
        self.items.iter().any(|d| d.severity == Severity::Error)
    }

    /** The diagnostics in the order recorded. */
    pub fn items(&self) -> &[Diagnostic] {
        &self.items
    }

    /** Consume the list. */
    pub fn into_vec(self) -> Vec<Diagnostic> {
        self.items
    }

    /**
     * Order by file and position, errors before warnings at one place, and drop exact
     * duplicates, which recovery can produce when two stages trip on one mistake.
     */
    pub fn sort(&mut self) {
        self.items.sort_by(|a, b| {
            let (sa, sb) = (a.span(), b.span());
            (sa.file, sa.lo, a.severity).cmp(&(sb.file, sb.lo, b.severity))
        });
        self.items.dedup();
    }
}
