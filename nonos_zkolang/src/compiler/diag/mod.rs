/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Diagnostics: what went wrong, where, and what to do about it. Every error and warning
 * carries a stable code, a message, a primary span with a label, optional secondary
 * labels, notes and one help line, and renders in the style of rustc.
 */

mod codes;
mod codes_program;
mod codes_syntax;
mod codes_types;
mod codes_warnings;
mod diagnostic;
mod diagnostic_new;
mod diagnostic_with;
mod display;
mod display_width;
mod json;
mod list;
mod list_mark;
mod near;
mod near_distance;
mod quote;
mod render;
mod render_cells;
mod render_file;
mod render_files;
mod render_gap;
mod render_gutter;
mod render_line;
mod render_multi;
mod render_multi_end;
mod render_placed;
mod render_window;
mod width_marks;
mod width_wide;

pub use codes::{explain, Code};
pub use diagnostic::{Diagnostic, Label, Severity};
pub(crate) use display_width::has_form;
pub use json::to_json;
pub use list::Diagnostics;
pub use near::did_you_mean;
pub use quote::quote;
pub use render::render;
