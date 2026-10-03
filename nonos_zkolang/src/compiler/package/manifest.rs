/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A package's manifest, `zkolang.toml` (section 4.1), checked: `[package]` with its `name`,
 * `version`, `edition` and optional `entry`, `[dependencies]` of path dependencies, and
 * `[cost]` thresholds. Anything else, or anything twice, is an error (E0902).
 */

use alloc::string::String;
use alloc::vec::Vec;

use super::manifest_check::Check;
use super::toml::parse_toml;
use crate::compiler::diag::Diagnostics;
use crate::compiler::source::{FileId, Span};

/** A path dependency: the name its dependent uses, and its directory. */
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Dep {
    pub name: String,
    pub path: String,
    pub span: Span,
}

/** A checked manifest. */
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct Manifest {
    pub name: String,
    pub version: String,
    /** `2025` or `2026`. */
    pub edition: u16,
    pub entry: Option<String>,
    pub deps: Vec<Dep>,
    pub unroll_warn: Option<u64>,
    pub dyn_index_warn: Option<u64>,
}

/** The manifest `text`, the file `file`, if it is well formed; its problems are in `diags`. */
pub fn manifest(file: FileId, text: &str, diags: &mut Diagnostics) -> Option<Manifest> {
    let errors = diags.error_count();
    let tables = parse_toml(file, text, diags);
    let mut c = Check {
        out: Manifest::default(),
        diags,
        seen: Vec::new(),
    };
    for t in &tables {
        c.table(t);
    }
    let whole = Span::new(file, 0, 0);
    c.required(whole);
    let out = c.out;
    (diags.error_count() == errors).then_some(out)
}
