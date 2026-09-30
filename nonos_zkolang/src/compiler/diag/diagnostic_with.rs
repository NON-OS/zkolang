/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The builder methods that add secondary labels, notes and a help line to a diagnostic. */

use alloc::string::String;

use super::diagnostic::{Diagnostic, Label};
use crate::compiler::source::Span;

impl Diagnostic {
    /** Add a secondary label. */
    pub fn with_label(mut self, span: Span, message: impl Into<String>) -> Diagnostic {
        self.labels.push(Label {
            span,
            message: message.into(),
            primary: false,
        });
        self
    }

    /** Add a note. */
    pub fn with_note(mut self, note: impl Into<String>) -> Diagnostic {
        self.notes.push(note.into());
        self
    }

    /** Set the help line. */
    pub fn with_help(mut self, help: impl Into<String>) -> Diagnostic {
        self.help = Some(help.into());
        self
    }
}
