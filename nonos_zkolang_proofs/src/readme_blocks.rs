/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The programs of `README.md`. Each is the fenced block after a line `<!-- what -->` that
 * says what must hold of it. `proves public=.. secret=.. outputs=..` builds, proves and
 * verifies with those outputs, from the files given by `root=PATH` when it names one;
 * `fails public=.. secret=..` builds and its run fails; `refused CODE` does not build, a
 * diagnostic of that code saying why; `tests N` checks and its `N` tests pass; and
 * `file PATH` is a file of a package another block builds.
 */

use crate::readme_claim::claim;
pub(crate) use crate::readme_claim::Claim;

/** A block of the README, the claim made of it, and the line the claim is on. */
pub(crate) struct Block {
    pub(crate) claim: Claim,
    pub(crate) src: String,
    pub(crate) line: usize,
}

/** Every block of `doc` a claim is made of, in order. */
pub(crate) fn blocks(doc: &str) -> Vec<Block> {
    let lines: Vec<&str> = doc.lines().collect();
    let mut out = Vec::new();
    for (i, l) in lines.iter().enumerate() {
        let inner = l.strip_prefix("<!-- ").and_then(|r| r.strip_suffix(" -->"));
        let words: Vec<&str> = inner.map_or(Vec::new(), |w| w.split_whitespace().collect());
        let Some(claim) = claim(&words) else {
            continue;
        };
        let fenced = lines.get(i + 1).is_some_and(|f| f.starts_with("```"));
        assert!(
            fenced,
            "README.md line {}: a claim with no block after it",
            i + 1
        );
        let body: Vec<&str> = lines[i + 2..]
            .iter()
            .take_while(|f| **f != "```")
            .copied()
            .collect();
        let src = body.join("\n") + "\n";
        out.push(Block {
            claim,
            src,
            line: i + 1,
        });
    }
    out
}
