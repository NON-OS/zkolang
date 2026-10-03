/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Write the format 7 fixture: a proof of `fixture/sum.zkl` on public 12 and secret 5,
 * beside the program's image, its statement and the run's public words. The seed is
 * fixed so that the bytes reproduce; a proof meant to hide its secret takes a fresh one.
 * `cargo run --release -p nonos_zkolang_format7 --example fixture`.
 */

use std::fs;
use std::path::Path;

use nonos_zkolang::compiler::driver::{build, seed_of, Source, SEED_BYTES};
use nonos_zkolang::compiler::source::SourceMap;
use nonos_zkolang::compiler::syntax::load::NoFiles;
use nonos_zkolang_format7::{prove, text, verify};

fn main() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("fixture");
    let src = fs::read_to_string(dir.join("sum.zkl")).expect("read the fixture's program");
    let b = build(
        &mut SourceMap::new(),
        &NoFiles,
        Source::file("sum.zkl", src),
    );
    let b = b.expect("the fixture's program builds");
    let p = prove(&b, &[12], &[5], &seed_of(&[7; SEED_BYTES])).expect("proves");
    verify(&p.statement, &p.bytes, &p.words).expect("nox_verify accepts it");
    let words: Vec<String> = p.words.iter().map(u64::to_string).collect();
    let (statement, words) = (text(&p.statement), words.join("\n") + "\n");
    let files: [(&str, &[u8]); 4] = [
        ("program.bin", &p.statement.image),
        ("statement.txt", statement.as_bytes()),
        ("proof.bin", &p.bytes),
        ("words.txt", words.as_bytes()),
    ];
    for (name, bytes) in files {
        fs::write(dir.join(name), bytes).expect("write the fixture");
    }
    println!("outputs {:?}, proof {} bytes", p.outputs, p.bytes.len());
}
