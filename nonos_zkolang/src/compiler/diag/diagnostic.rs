/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! One diagnostic: its severity, code, message, labels, notes and help line. */

use alloc::string::String;
use alloc::vec::Vec;

use super::codes::Code;
use crate::compiler::source::Span;

/** How serious a diagnostic is. An error stops compilation; a warning does not. */
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Severity {
    Error,
    Warning,
}

/**
 * A span with a message. The primary label marks where the problem is; secondary labels
 * mark what the primary one relates to.
 */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Label {
    pub span: Span,
    pub message: String,
    pub primary: bool,
}

/** One error or warning. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Diagnostic {
    pub severity: Severity,
    pub code: Code,
    pub message: String,
    pub labels: Vec<Label>,
    pub notes: Vec<String>,
    pub help: Option<String>,
}

impl Diagnostic {
    /** The primary span. */
    pub fn span(&self) -> Span {
        self.labels
            .iter()
            .find(|l| l.primary)
            .map(|l| l.span)
            .unwrap_or(Span::DUMMY)
    }

    /** Whether this is an error. */
    pub fn is_error(&self) -> bool {
        self.severity == Severity::Error
    }
}
