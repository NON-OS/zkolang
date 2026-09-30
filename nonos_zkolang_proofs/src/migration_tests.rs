/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Every pair of `docs/migration-2026.md` (section 20.3): the edition 2025 program, proved
 * by the frozen compiler, and the edition 2026 one, built without a warning and run, give
 * the same outputs on the inputs the pair names.
 */

use nonos_zkolang::compiler::driver::{build, run, Source};
use nonos_zkolang::compiler::source::SourceMap;
use nonos_zkolang::compiler::syntax::load::NoFiles;
use nonos_zkolang::{expand_with_stdlib, prove_source_with_witness};

use crate::golden_corpus::repo_root;
use crate::migration_pairs::pairs;

#[test]
fn every_pair_of_the_migration_guide_agrees() {
    let doc = std::fs::read_to_string(repo_root().join("docs/migration-2026.md"));
    let pairs = pairs(&doc.expect("docs/migration-2026.md"));
    assert!(pairs.len() >= 14, "{} pairs", pairs.len());
    let wide = |v: &[u64]| v.iter().map(|&x| i128::from(x)).collect::<Vec<i128>>();
    for (k, p) in pairs.iter().enumerate() {
        let src = expand_with_stdlib(&p.old).expect("expand");
        let old = prove_source_with_witness(&src, &p.public, &p.secret);
        let old = old.unwrap_or_else(|e| panic!("pair {k}, edition 2025: {e:?}"));
        assert!(old.verified, "pair {k}");
        let file = Source::file("m.zkl", p.new.clone());
        let b = build(&mut SourceMap::new(), &NoFiles, file);
        let b = b.unwrap_or_else(|d| panic!("pair {k}, edition 2026: {:?}", d.items()));
        assert!(
            b.warnings.items().is_empty(),
            "pair {k}: {:?}",
            b.warnings.items()
        );
        let new = run(&b, &wide(&p.public), &wide(&p.secret));
        let new = new.unwrap_or_else(|e| panic!("pair {k}, edition 2026 run: {e:?}"));
        let new: Vec<u64> = new
            .iter()
            .map(|&x| u64::try_from(x).expect("a slot"))
            .collect();
        assert_eq!(old.outputs, new, "pair {k}");
    }
}
