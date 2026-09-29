/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Read a program and splice in its includes. */

use std::fs;
use std::path::{Component, Path};

use nonos_zkolang::{expand_includes_from, stdlib_source, Included};

/** The key prefix of a file taken from the standard library built into the binary. */
const EMBEDDED: &str = "<stdlib>/";

/**
 * Read `file` and expand its includes. Each include resolves from the directory of the
 * file that writes it, then from a `stdlib` folder there or in any directory above it,
 * then from the standard library built into the binary, so a program resolves the same
 * way from any working directory. A path may only descend from those places: an
 * absolute path, or one that climbs with `..`, is refused.
 */
pub(crate) fn load(file: &str) -> Result<String, String> {
    let path = fs::canonicalize(file).map_err(|e| format!("read {file}: {e}"))?;
    let root = path.to_str().ok_or(format!("{file}: path is not UTF-8"))?;
    let src = fs::read_to_string(&path).map_err(|e| format!("read {file}: {e}"))?;
    let mut failed = None;
    let mut resolve = |from: &str, name: &str| {
        let found = resolve(from, name);
        found.map_err(|why| failed = Some(why)).ok()
    };
    let expanded = expand_includes_from(root, &src, &mut resolve);
    expanded.map_err(|e| failed.unwrap_or_else(|| format!("include error: {e:?}")))
}

fn resolve(from: &str, name: &str) -> Result<Included, String> {
    let descends = Path::new(name)
        .components()
        .all(|c| matches!(c, Component::Normal(_) | Component::CurDir));
    if !descends {
        return Err(format!(
            "include \"{name}\" in {from}: the path must not be absolute or climb with `..`"
        ));
    }
    if !from.starts_with(EMBEDDED) {
        let dir = Path::new(from).parent().unwrap_or(Path::new(""));
        let stdlibs = dir.ancestors().map(|d| d.join("stdlib").join(name));
        for candidate in std::iter::once(dir.join(name)).chain(stdlibs) {
            let Ok(found) = fs::canonicalize(&candidate) else {
                continue;
            };
            let (Some(key), Ok(text)) = (found.to_str(), fs::read_to_string(&found)) else {
                continue;
            };
            let key = key.to_string();
            return Ok(Included { key, text });
        }
    }
    let text =
        stdlib_source(name).ok_or(format!("include \"{name}\" in {from}: file not found"))?;
    let key = format!("{EMBEDDED}{name}");
    Ok(Included {
        key,
        text: text.to_string(),
    })
}
