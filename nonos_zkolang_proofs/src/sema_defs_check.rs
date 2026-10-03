/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Collect a program's items, resolve its imports, and look names up in it, for tests. */

use nonos_zkolang::compiler::diag::Diagnostics;
use nonos_zkolang::compiler::sema::defs::{DefKind, Defs, PathError};
use nonos_zkolang::compiler::source::SourceMap;
use nonos_zkolang::compiler::syntax::ast::PathRoot;
use nonos_zkolang::compiler::syntax::lex::lex;
use nonos_zkolang::compiler::syntax::parse::parse_file;

/** Resolve `path` from the module named by `from` (empty for the root) in `src`. */
pub(crate) fn names(
    src: &str,
    lookups: &[(&str, &str)],
) -> (Vec<&'static str>, Vec<Result<DefKind, PathError>>) {
    let mut map = SourceMap::new();
    let id = map.add(String::from("t.zkl"), String::from(src));
    let mut diags = Diagnostics::new();
    let lexed = lex(id, src, &mut diags);
    let ast = parse_file(id, src, &lexed, &mut diags, &mut 0);
    assert!(!diags.has_errors(), "{:?}", diags.items());
    let (mut defs, imports) = Defs::collect(&ast, &mut diags);
    defs.resolve_imports(&imports, &mut diags);
    let found = lookups.iter().map(|(from, path)| {
        let from = defs
            .resolve(Defs::ROOT, PathRoot::Crate, &segs(from))
            .unwrap();
        defs.resolve(from, PathRoot::Plain, &segs(path))
            .map(|d| defs.get(d).unwrap().kind)
    });
    let found = found.collect();
    (diags.items().iter().map(|d| d.code.0).collect(), found)
}

/** The segments of a path written with `::`. */
fn segs(s: &str) -> Vec<&str> {
    s.split("::").filter(|x| !x.is_empty()).collect()
}
