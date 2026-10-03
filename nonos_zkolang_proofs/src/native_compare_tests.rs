/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A program with ordered comparisons runs natively and agrees with the proof. The targets
 * read each comparison's bits as further inputs past the secrets, which only the prover's
 * advice fill knows, so such a program could not be run natively at all.
 */

use std::path::Path;
use std::process::Command;

use nonos_zkolang::{compile_source_full, prove_source_with_witness, to_asm, to_c, to_python};

const SRC: &str = "input a;\ninput b;\noutput a < b;\noutput if a < 10 { a } else { b };\n\
                   output (a < b) + (b < a);";
const RUNS: [[u64; 2]; 4] = [[3, 7], [7, 3], [5, 5], [12, 9]];

/** What a command prints for each input pair. */
fn printed(mut cmd: impl FnMut(&[u64; 2]) -> Command) -> Vec<String> {
    RUNS.iter()
        .map(|r| cmd(r).output().expect("run"))
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .collect()
}

fn build(dir: &Path, name: &str, text: String) -> std::path::PathBuf {
    let (src, bin) = (dir.join(name), dir.join(format!("{name}.bin")));
    std::fs::write(&src, text).expect("write");
    let built = Command::new("cc")
        .arg(&src)
        .arg("-o")
        .arg(&bin)
        .status()
        .expect("cc");
    assert!(built.success(), "{name} did not build");
    bin
}

#[test]
fn comparisons_run_natively_as_they_prove() {
    let line = |r: &[u64; 2]| {
        let out = prove_source_with_witness(SRC, r, &[])
            .expect("prove")
            .outputs;
        out.iter().map(u64::to_string).collect::<Vec<_>>().join(" ")
    };
    let want: Vec<String> = RUNS.iter().map(line).collect();
    let compiled = compile_source_full(SRC).expect("compile");
    let dir = std::env::temp_dir().join(format!("zkolang-cmp-{}", std::process::id()));
    std::fs::create_dir_all(&dir).expect("dir");
    for bin in [
        build(&dir, "p.c", to_c(&compiled)),
        build(&dir, "p.S", to_asm(&compiled)),
    ] {
        let got = printed(|r| {
            let mut c = Command::new(&bin);
            c.args(r.iter().map(u64::to_string));
            c
        });
        assert_eq!(got, want, "{}", bin.display());
    }
    std::fs::write(dir.join("m.py"), to_python(&compiled)).expect("write");
    let got = printed(|r| {
        let mut c = Command::new("python3");
        let call = format!("import m; print(' '.join(map(str, m.run({r:?}))))");
        c.current_dir(&dir).args(["-O", "-c", &call]);
        c
    });
    assert_eq!(got, want, "python");
}
