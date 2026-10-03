/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Any text is built as an edition 2026 program, and a program that builds is run on inputs
 * taken from the same bytes. Building never panics, and a run never finds the compiled
 * machine program and the reference run disagreeing: that would be a compiler bug.
 */

#![no_main]

use libfuzzer_sys::fuzz_target;
use nonos_zkolang::compiler::driver::{build, run, RunFailure, Source};
use nonos_zkolang::compiler::source::SourceMap;
use nonos_zkolang::compiler::syntax::load::NoFiles;
use nonos_zkolang_fuzz::{text, values};

fuzz_target!(|data: &[u8]| {
    let mut map = SourceMap::new();
    let Ok(built) = build(&mut map, &NoFiles, Source::file("f.zkl", text(data))) else {
        return;
    };
    let public = values(data, built.public.len());
    let secret = values(&data[..data.len() / 2], built.secret.len());
    let result = run(&built, &public, &secret);
    assert!(
        !matches!(result, Err(RunFailure::Disagree)),
        "the compiled program and the reference run disagree"
    );
});
