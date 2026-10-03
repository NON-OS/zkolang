/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Every program `README.md` shows does what the README says it does. */

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use crate::readme_blocks::{blocks, Claim};
use crate::readme_files::Shown;
use crate::readme_holds::holds;

fn readme() -> String {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop();
    p.push("README.md");
    fs::read_to_string(p).expect("read README.md")
}

#[test]
fn every_program_the_readme_shows_does_what_it_says() {
    let blocks = blocks(&readme());
    let mut files = BTreeMap::new();
    for b in &blocks {
        if let Claim::File(path) = &b.claim {
            files.insert(path.clone(), b.src.clone());
        }
    }
    let shown = Shown(files);
    assert!(blocks.len() >= 10, "{} claimed blocks", blocks.len());
    for b in &blocks {
        if let Err(why) = holds(b, &shown) {
            panic!("README.md line {}: {why}", b.line);
        }
    }
}
