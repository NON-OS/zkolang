/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Run the edition 2026 front end over one source, as a `.zkl` test sees it, and the
 * checker too for a program under `sema/`.
 */

use nonos_zkolang::compiler::diag::{render, Diagnostics};
use nonos_zkolang::compiler::sema::check;
use nonos_zkolang::compiler::source::SourceMap;
use nonos_zkolang::compiler::syntax::lex::lex;
use nonos_zkolang::compiler::syntax::parse::parse_file;

/** What the front end reported for a source: each diagnostic's line and code, sorted. */
pub(crate) struct Reported {
    pub(crate) lines: Vec<(usize, String)>,
    pub(crate) rendered: String,
}

/** Lex and parse `src` as the file `name`, collecting every diagnostic. */
pub(crate) fn report(name: &str, src: &str) -> Reported {
    let mut map = SourceMap::new();
    let id = map.add(name.to_string(), src.to_string());
    let mut diags = Diagnostics::new();
    let lexed = lex(id, src, &mut diags);
    let mut next_id = 0;
    let ast = parse_file(id, src, &lexed, &mut diags, &mut next_id);
    /* A program under `sema/` that parses is checked too. */
    if name.starts_with("sema") && !diags.has_errors() {
        diags.extend(check(&mut map, &ast).1);
    }
    let mut lines = Vec::new();
    let mut rendered = String::new();
    for d in diags.items() {
        let line = map.locate(d.span()).map_or(0, |(_, line, _)| line);
        lines.push((line, d.code.0.to_string()));
        rendered.push_str(&render(&map, d));
        rendered.push('\n');
    }
    lines.sort();
    Reported { lines, rendered }
}
