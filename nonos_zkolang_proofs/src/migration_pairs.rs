/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The pairs of `docs/migration-2026.md`: each a line `<!-- run public=.. secret=.. -->`
 * naming the inputs, then an edition 2025 program and an edition 2026 one, each fenced.
 */

/** One pair: the inputs, and the program in each edition. */
pub(crate) struct Pair {
    pub(crate) public: Vec<u64>,
    pub(crate) secret: Vec<u64>,
    pub(crate) old: String,
    pub(crate) new: String,
}

/** The pairs of the guide `doc`, in order. */
pub(crate) fn pairs(doc: &str) -> Vec<Pair> {
    let mut out = Vec::new();
    let mut lines = doc.lines();
    while let Some(l) = lines.next() {
        let Some(run) = l
            .strip_prefix("<!-- run")
            .and_then(|r| r.strip_suffix("-->"))
        else {
            continue;
        };
        let (mut public, mut secret) = (Vec::new(), Vec::new());
        for part in run.split_whitespace() {
            let (k, v) = part.split_once('=').expect("key=values");
            let vals = v.split(',').map(|x| x.parse().expect("a number")).collect();
            match k {
                "public" => public = vals,
                "secret" => secret = vals,
                _ => panic!("unknown key {k}"),
            }
        }
        let old = fence(&mut lines, "```zkolang-2025");
        let new = fence(&mut lines, "```zkolang-2026");
        out.push(Pair {
            public,
            secret,
            old,
            new,
        });
    }
    out
}

/** The program in the fence `open` that the next line opens. */
fn fence<'a>(lines: &mut impl Iterator<Item = &'a str>, open: &str) -> String {
    assert_eq!(lines.next(), Some(open), "a pair's fence");
    let mut s = String::new();
    for l in lines.by_ref() {
        if l == "```" {
            return s;
        }
        s.push_str(l);
        s.push('\n');
    }
    panic!("the fence {open} is not closed")
}
