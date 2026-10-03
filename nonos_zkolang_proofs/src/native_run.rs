/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * A built program of edition 2026 run as native code: its C target compiled with every
 * warning an error and run, and its Python target run as a script, each on the command
 * line `zkolang run` takes.
 */

use std::path::PathBuf;
use std::process::{Command, Output};

use nonos_zkolang::compiler::driver::{Built, RunFailure};
use nonos_zkolang::compiler::native::{to_c, to_python};

/** What a run gave: the line of outputs it printed, or the status it exited with. */
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Ran {
    Printed(String),
    Status(Option<i32>),
}

fn ran(o: Output) -> Ran {
    match o.status.success() {
        true => Ran::Printed(String::from_utf8_lossy(&o.stdout).trim().into()),
        false => Ran::Status(o.status.code()),
    }
}

/** What a native run must give where the reference run gave `r`. */
pub(crate) fn expected(r: &Result<Vec<i128>, RunFailure>) -> Ran {
    match r {
        Ok(out) => Ran::Printed(
            out.iter()
                .map(i128::to_string)
                .collect::<Vec<_>>()
                .join(" "),
        ),
        Err(RunFailure::Fails(_)) => Ran::Status(Some(3)),
        Err(RunFailure::Inputs(..)) => Ran::Status(Some(1)),
        Err(e) => panic!("the reference run stopped: {e:?}"),
    }
}

/** The compiled C target and the Python target of a program. */
pub(crate) struct Native(PathBuf, PathBuf);

impl Native {
    /** `b`'s targets, written under a directory named for `name` and this process. */
    pub(crate) fn of(b: &Built, name: &str) -> Native {
        let dir = std::env::temp_dir().join(format!("zkl-native-{}-{name}", std::process::id()));
        std::fs::create_dir_all(&dir).expect("dir");
        let (src, bin, py) = (dir.join("p.c"), dir.join("p"), dir.join("p.py"));
        std::fs::write(&src, to_c(b)).expect("write");
        std::fs::write(&py, to_python(b)).expect("write");
        let mut cc = Command::new("cc");
        cc.args(["-O1", "-Wall", "-Wextra", "-Werror", "-o"]);
        let out = cc.arg(&bin).arg(&src).output().expect("cc");
        let why = String::from_utf8_lossy(&out.stderr);
        assert!(out.status.success(), "{name} does not compile:\n{why}");
        Native(bin, py)
    }

    /** What the C target and the Python target give on the inputs `args`. */
    pub(crate) fn run(&self, args: &[i128]) -> [Ran; 2] {
        let args: Vec<String> = args.iter().map(i128::to_string).collect();
        let c = Command::new(&self.0).args(&args).output().expect("run");
        let mut py = Command::new("python3");
        let py = py.arg("-O").arg(&self.1).args(&args).output();
        [ran(c), ran(py.expect("python3"))]
    }
}
