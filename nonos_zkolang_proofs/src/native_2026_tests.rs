/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Edition 2026 run natively. The C and the Python target of a program print what the
 * reference run gives and fail where it fails: on every program the README runs, and on
 * one that takes each kind of leaf, at the edges of its type and past them.
 */

use nonos_zkolang::compiler::driver::{build, run, Source};
use nonos_zkolang::compiler::source::SourceMap;
use nonos_zkolang::compiler::syntax::load::NoFiles;

use crate::native_run::{expected, Native};
use crate::readme_blocks::Claim;
use crate::readme_holds::built;
use crate::readme_tests::readme;

#[test]
fn every_program_the_readme_runs_runs_natively_as_it_runs() {
    let (blocks, shown) = readme();
    let mut ran = 0;
    for b in &blocks {
        let (public, secret, root) = match &b.claim {
            Claim::Proves(public, secret, _, root) => (public, secret, root.clone()),
            Claim::Fails(public, secret) => (public, secret, None),
            _ => continue,
        };
        let p = built(b, &root, &shown).expect("builds");
        let want = expected(&run(&p, public, secret));
        let args: Vec<i128> = public.iter().chain(secret).copied().collect();
        let got = Native::of(&p, &format!("readme{}", b.line)).run(&args);
        assert_eq!(got, [want.clone(), want], "README.md line {}", b.line);
        ran += 1;
    }
    assert!(ran >= 8, "{ran} programs ran");
}

const LEAVES: &str = include_str!("../native/leaves.zkl");

#[test]
fn each_kind_of_leaf_runs_natively_as_it_runs() {
    let src = Source::file("leaves.zkl", LEAVES.into());
    let b = build(&mut SourceMap::new(), &NoFiles, src).expect("builds");
    let native = Native::of(&b, "leaves");
    let (lo, hi) = (i128::from(i64::MIN), i128::from(i64::MAX));
    let runs: [[i128; 5]; 8] = [
        [-7, 1000, -20, 7, 9],
        [
            0,
            u64::MAX.into(),
            i32::MAX.into(),
            255,
            18446744069414584320,
        ],
        [lo + 1, 5, i32::MIN.into(), 0, 3],
        [lo, 5, 1, 0, 3],
        [hi, 0, -6, 200, 1],
        [-1, u64::MAX.into(), -5, 0, 0],
        [hi + 1, 0, 0, 0, 0],
        [0, -1, 0, 256, 18446744069414584321],
    ];
    for r in &runs {
        let want = expected(&run(&b, &r[..2], &r[2..]));
        assert_eq!(native.run(r), [want.clone(), want], "{r:?}");
    }
}
