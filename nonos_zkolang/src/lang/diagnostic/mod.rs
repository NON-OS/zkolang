/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Rendering a compile error as a diagnostic. An error that carries a byte offset is
 * shown with its line and column and a caret under the offending place in the source,
 * the way a compiler points at a mistake; an error without a location is shown as its
 * message alone.
 */

mod locate;
mod message;
mod render;
mod span_of;

pub use render::render;
