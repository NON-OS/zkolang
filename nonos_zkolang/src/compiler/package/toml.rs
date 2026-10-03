/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The TOML subset a manifest is written in (section 4.1): `[section]` headers, `key = value`
 * lines, `#` comments, and values that are strings, integers or inline tables of them.
 * Each line is read on its own, so a bad line is reported (E0902) and the next is read.
 */

use alloc::string::String;
use alloc::vec::Vec;

use super::toml_cursor::Cursor;
use crate::compiler::diag::Diagnostics;
use crate::compiler::source::{FileId, Span};

/** A value. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Value {
    Str(String),
    Int(u64),
    Table(Vec<Entry>),
}

/** `key = value`, with where each is written. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Entry {
    pub key: String,
    pub span: Span,
    pub value: Value,
    pub value_span: Span,
}

/** A section: its name, empty for the lines before any header, and its entries. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Table {
    pub name: String,
    pub span: Span,
    pub entries: Vec<Entry>,
}

/** Parse the manifest `text`, the file `file`; each bad line is reported in `diags`. */
pub fn parse_toml(file: FileId, text: &str, diags: &mut Diagnostics) -> Vec<Table> {
    let mut tables = alloc::vec![Table {
        name: String::new(),
        span: Span::new(file, 0, 0),
        entries: Vec::new(),
    }];
    let mut c = Cursor::new(file, text);
    while !c.done() {
        match c.line() {
            Ok(Some(super::toml_cursor::Line::Header(name, span))) => tables.push(Table {
                name,
                span,
                entries: Vec::new(),
            }),
            Ok(Some(super::toml_cursor::Line::Entry(e))) => {
                if let Some(t) = tables.last_mut() {
                    t.entries.push(e);
                }
            }
            Ok(None) => {}
            Err(d) => {
                diags.push(d);
                c.skip_line();
            }
        }
    }
    tables
}
