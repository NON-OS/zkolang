/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The references `zkolang doc` makes (section 17.2): the standard library's, which
 * `docs/stdlib.md` must be word for word, and a sample crate's, which `doc/sample.md` is.
 */

use std::fs;

use nonos_zkolang::compiler::diag::Diagnostics;
use nonos_zkolang::compiler::doc::render_doc;
use nonos_zkolang::compiler::source::SourceMap;
use nonos_zkolang::compiler::std_crate::load_std;
use nonos_zkolang::compiler::syntax::load::{load, NoFiles};

use crate::golden_corpus::repo_root;

#[test]
fn the_standard_library_reference_is_up_to_date() {
    let (mut map, mut diags) = (SourceMap::new(), Diagnostics::new());
    let std = load_std(&mut map, &mut diags);
    assert!(!diags.has_errors(), "std does not parse");
    let made = render_doc(&map, "std", &std);
    let written = fs::read_to_string(repo_root().join("docs/stdlib.md")).expect("docs/stdlib.md");
    assert!(
        written == made,
        "docs/stdlib.md is not what `zkolang doc --std` makes; write its output there"
    );
}

#[test]
fn a_crate_reference_lists_its_public_items() {
    let dir = repo_root().join("nonos_zkolang_proofs/doc");
    let src = fs::read_to_string(dir.join("sample.zkl")).expect("sample.zkl");
    let (mut map, mut diags) = (SourceMap::new(), Diagnostics::new());
    let ast = load(&NoFiles, ("sample.zkl", src), &mut map, &mut diags);
    assert!(!diags.has_errors(), "sample.zkl does not parse");
    let want = fs::read_to_string(dir.join("sample.md")).expect("sample.md");
    assert_eq!(render_doc(&map, "sample", &ast), want);
}
