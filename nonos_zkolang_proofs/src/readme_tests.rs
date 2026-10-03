/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Every program `README.md` shows does what the README says it does. */

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use crate::readme_blocks::{blocks, Block, Claim};
use crate::readme_files::Shown;
use crate::readme_holds::holds;

/** The README's blocks, and the files of its packages. */
pub(crate) fn readme() -> (Vec<Block>, Shown) {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.push("README.md");
    let blocks = blocks(&fs::read_to_string(p).expect("read README.md"));
    let mut files = BTreeMap::new();
    for b in &blocks {
        if let Claim::File(path) = &b.claim {
            files.insert(path.clone(), b.src.clone());
        }
    }
    (blocks, Shown(files))
}

#[test]
fn every_program_the_readme_shows_does_what_it_says() {
    let (blocks, shown) = readme();
    assert!(blocks.len() >= 10, "{} claimed blocks", blocks.len());
    for b in &blocks {
        if let Err(why) = holds(b, &shown) {
            panic!("README.md line {}: {why}", b.line);
        }
    }
}
