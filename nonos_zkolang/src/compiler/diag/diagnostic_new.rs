/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The constructors of a diagnostic: an error or a warning with its primary label. */

use alloc::string::String;
use alloc::vec::Vec;

use super::codes::Code;
use super::diagnostic::{Diagnostic, Label, Severity};
use crate::compiler::source::Span;

impl Diagnostic {
    /** An error with its primary span and label. */
    pub fn error(
        code: Code,
        message: impl Into<String>,
        span: Span,
        label: impl Into<String>,
    ) -> Diagnostic {
        Diagnostic::new(Severity::Error, code, message.into(), span, label.into())
    }

    /** A warning with its primary span and label. */
    pub fn warning(
        code: Code,
        message: impl Into<String>,
        span: Span,
        label: impl Into<String>,
    ) -> Diagnostic {
        Diagnostic::new(Severity::Warning, code, message.into(), span, label.into())
    }

    fn new(
        severity: Severity,
        code: Code,
        message: String,
        span: Span,
        label: String,
    ) -> Diagnostic {
        Diagnostic {
            severity,
            code,
            message,
            labels: alloc::vec![Label {
                span,
                message: label,
                primary: true,
            }],
            notes: Vec::new(),
            help: None,
        }
    }
}
