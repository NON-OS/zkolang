/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Whether a block of `README.md` does what its claim says. */

use nonos_zkolang::compiler::driver::{build, prove, run, seed_of, test, Built, RunFailure};
use nonos_zkolang::compiler::driver::{Source, SEED_BYTES};
use nonos_zkolang::compiler::source::SourceMap;
use nonos_zkolang::compiler::syntax::load::NoFiles;

use crate::readme_blocks::{Block, Claim};
use crate::readme_files::{manifest_of, Shown};

/** The block `b` as a program of one file. */
fn one(b: &Block) -> Source<'static> {
    Source::file("README.md", b.src.clone())
}

pub(crate) fn built(b: &Block, root: &Option<String>, shown: &Shown) -> Result<Built, String> {
    let mut map = SourceMap::new();
    let r = match root {
        None => build(&mut map, &NoFiles, one(b)),
        Some(path) => {
            let manifest = manifest_of(path);
            let text = |p: &str| shown.0.get(p).cloned().ok_or(format!("no file {p}"));
            let src = Source {
                root: (path.as_str(), text(path)?),
                manifest: Some((manifest.as_str(), text(&manifest)?)),
            };
            build(&mut map, shown, src)
        }
    };
    r.map_err(|d| format!("{:?}", d.items().first().map(|x| &x.message)))
}

pub(crate) fn holds(b: &Block, shown: &Shown) -> Result<(), String> {
    match &b.claim {
        Claim::Proves(public, secret, outputs, root) => {
            let seed = seed_of(&[7; SEED_BYTES]);
            let p = prove(&built(b, root, shown)?, public, secret, &seed);
            let p = p.map_err(|e| format!("{e:?}"))?;
            let ok = p.report.verified && &p.outputs == outputs;
            ok.then_some(()).ok_or(format!("proved {:?}", p.outputs))
        }
        Claim::Fails(public, secret) => match run(&built(b, &None, shown)?, public, secret) {
            Err(RunFailure::Fails(_)) => Ok(()),
            other => Err(format!("ran to {other:?}")),
        },
        Claim::Refused(code) => match build(&mut SourceMap::new(), &NoFiles, one(b)) {
            Err(d) if d.items().iter().any(|x| x.code.0 == code) => Ok(()),
            _ => Err(format!("not refused with {code}")),
        },
        Claim::Tests(n) => {
            let r = test(&mut SourceMap::new(), &NoFiles, one(b));
            let (reports, _) = r.map_err(|_| String::from("does not check"))?;
            let passed = reports
                .iter()
                .filter(|t| t.failure.items().is_empty())
                .count();
            (reports.len() == *n && passed == *n)
                .then_some(())
                .ok_or(format!("{passed} passed"))
        }
        Claim::File(_) => Ok(()),
    }
}
