/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Running the command line on the argument tests' programs, each run in a directory of
 * its own: the tests run at once, and a run must not rewrite a file another run reads.
 */

use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

/** How many directories `files` has made; each run gets its own, as tests run at once. */
static RUNS: AtomicUsize = AtomicUsize::new(0);

fn files() -> PathBuf {
    let run = RUNS.fetch_add(1, Ordering::Relaxed);
    let name = format!("zkolang-args-{}-{run}", std::process::id());
    let dir = std::env::temp_dir().join(name);
    std::fs::create_dir_all(&dir).expect("temp dir");
    let long = format!(
        "input x;\n{}output x;",
        "for i in 0..40000 { let x = x + i; }\n".repeat(2)
    );
    let files = [
        ("sq.zkl", "input x;\noutput x * x;"),
        ("-sq.zkl", "output 4;"),
        ("long.zkl", &long),
    ];
    for (path, text) in files {
        std::fs::write(dir.join(path), text).expect("write");
    }
    dir
}

pub fn zk(args: &[&str]) -> (bool, String) {
    let out = Command::new(env!("CARGO_BIN_EXE_zkolang"))
        .current_dir(files())
        .args(args)
        .output()
        .expect("run zkolang");
    let text =
        String::from_utf8_lossy(&out.stdout).to_string() + &String::from_utf8_lossy(&out.stderr);
    (out.status.success(), text)
}
