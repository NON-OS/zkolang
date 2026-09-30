/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Collecting a program's crates: its own, its dependencies and `std`, each a root; each
 * crate's dependencies named as it names them; then the imports, the items and the `impl`
 * blocks of each, before the checks that follow.
 */

use super::super::cx::Sema;
use super::super::defs::Defs;
use super::check::CrateRef;
use crate::compiler::diag::Diagnostics;
use crate::compiler::source::SourceMap;
use crate::compiler::std_crate::load_std;
use crate::compiler::tir::TProgram;

/** Check `crates`, the program's first, beside `std`, loaded into `map`; for tests if `testing`. */
pub(super) fn check_mode(
    map: &mut SourceMap,
    crates: &[CrateRef],
    testing: bool,
) -> (TProgram, Diagnostics) {
    let mut sema = Sema::default();
    let std = load_std(map, &mut sema.diags);
    let Some(first) = crates.first() else {
        return (TProgram::default(), sema.diags);
    };
    for c in crates {
        sema.check_attrs(&c.ast.items, &c.ast.inner_attrs, c.ast.span);
    }
    sema.check_attrs(&std.items, &std.inner_attrs, std.span);
    let (mut defs, mut imports) =
        Defs::collect_with(first.ast, Some(&std), testing, &mut sema.diags);
    let mut roots = alloc::vec![Defs::ROOT];
    for c in crates.iter().skip(1) {
        roots.push(defs.collect_crate(c.name, c.ast, &mut imports, &mut sema.diags));
    }
    for (c, &root) in crates.iter().zip(&roots) {
        let named = c
            .deps
            .iter()
            .filter_map(|(n, i)| Some((n.clone(), *roots.get(*i)?)));
        defs.deps.insert(root, named.collect());
    }
    sema.defs = defs;
    sema.defs.resolve_imports(&imports, &mut sema.diags);
    sema.register();
    for (c, &root) in crates.iter().zip(&roots) {
        sema.impls(&c.ast.items, root);
    }
    if let Some(root) = sema.defs.std {
        sema.impls(&std.items, root);
    }
    sema.finish()
}
