/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Loading a crate (section 4.2): its root file parsed, then each `mod name;` given the
 * items of its own file, `name.zkl` or `name/mod.zkl` in its parent module's directory,
 * so the rest of the compiler sees every module inline. Node ids run on across the files,
 * and each file's spans name it, so diagnostics point into the file they are about.
 */

use alloc::string::{String, ToString};

use super::files::{dir_of, Files};
use crate::compiler::diag::Diagnostics;
use crate::compiler::source::SourceMap;
use crate::compiler::syntax::ast::SourceAst;
use crate::compiler::syntax::lex::lex;
use crate::compiler::syntax::parse::parse_file;

/** What loading reads from and writes to. */
pub(super) struct Loader<'l> {
    pub files: &'l dyn Files,
    pub map: &'l mut SourceMap,
    pub diags: &'l mut Diagnostics,
    pub next_id: u32,
}

/**
 * The crate whose root file is `path`, of text `text`, each module in its own file read
 * from `files` and loaded into its declaration. Problems are in `diags`.
 */
pub fn load(
    files: &dyn Files,
    (path, text): (&str, String),
    map: &mut SourceMap,
    diags: &mut Diagnostics,
) -> SourceAst {
    let mut l = Loader {
        files,
        map,
        diags,
        next_id: 0,
    };
    let mut ast = l.parse(path, text);
    l.splice(&mut ast.items, dir_of(path), 0);
    ast
}

impl Loader<'_> {
    /** Parse `text`, the file at `path`, adding it to the source map. */
    pub(super) fn parse(&mut self, path: &str, text: String) -> SourceAst {
        let id = self.map.add(path.to_string(), text.clone());
        let lexed = lex(id, &text, self.diags);
        parse_file(id, &text, &lexed, self.diags, &mut self.next_id)
    }
}
