/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! The claim a line `<!-- what -->` of `README.md` makes of the block after it. */

/** What must hold of a block. */
pub(crate) enum Claim {
    Proves(Vec<i128>, Vec<i128>, Vec<i128>, Option<String>),
    Fails(Vec<i128>, Vec<i128>),
    Refused(String),
    Tests(usize),
    File(String),
}

fn arg<'a>(args: &[&'a str], key: &str) -> Option<&'a str> {
    args.iter()
        .find_map(|a| a.strip_prefix(key).and_then(|v| v.strip_prefix('=')))
}

fn values(args: &[&str], key: &str) -> Vec<i128> {
    arg(args, key)
        .map(|v| v.split(',').map(|x| x.parse().expect("a number")).collect())
        .unwrap_or_default()
}

pub(crate) fn claim(words: &[&str]) -> Option<Claim> {
    let (what, args) = words.split_first()?;
    let (public, secret) = (values(args, "public"), values(args, "secret"));
    Some(match *what {
        "proves" => {
            let root = arg(args, "root").map(String::from);
            Claim::Proves(public, secret, values(args, "outputs"), root)
        }
        "fails" => Claim::Fails(public, secret),
        "refused" => Claim::Refused(args.first()?.to_string()),
        "tests" => Claim::Tests(args.first()?.parse().ok()?),
        "file" => Claim::File(args.first()?.to_string()),
        _ => return None,
    })
}
