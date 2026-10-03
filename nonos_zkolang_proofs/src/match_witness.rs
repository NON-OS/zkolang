/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*! Reading back the missing pattern E0400 names, for the `(E, u8)` of `match_gen`. */

use crate::match_gen::Pat;

/** The pattern `s` writes, if it is a pattern of `(E, u8)` that the checker writes. */
pub(crate) fn parse(s: &str) -> Option<Pat> {
    let s = s.trim();
    if s == "_" {
        return Some(Pat::Wild);
    }
    let inner = s.strip_prefix('(')?.strip_suffix(')')?;
    let mut depth = 0i32;
    let at = inner.char_indices().find(|&(_, c)| {
        depth += i32::from(c == '(') - i32::from(c == ')');
        c == ',' && depth == 0
    })?;
    let (e, n) = (&inner[..at.0], &inner[at.0 + 1..]);
    Some(Pat::Pair(
        Box::new(enum_pat(e.trim())?),
        Box::new(int_pat(n.trim())?),
    ))
}

fn enum_pat(s: &str) -> Option<Pat> {
    if s == "_" {
        return Some(Pat::Wild);
    }
    if s == "E::A" {
        return Some(Pat::A);
    }
    if let Some(b) = s.strip_prefix("E::B(").and_then(|r| r.strip_suffix(')')) {
        let p = match b {
            "_" => Pat::Wild,
            "true" => Pat::Bool(true),
            "false" => Pat::Bool(false),
            _ => return None,
        };
        return Some(Pat::B(Box::new(p)));
    }
    let n = s.strip_prefix("E::C(")?.strip_suffix(')')?;
    Some(Pat::C(Box::new(int_pat(n)?)))
}

fn int_pat(s: &str) -> Option<Pat> {
    if s == "_" {
        return Some(Pat::Wild);
    }
    match s.split_once("..=") {
        Some((a, b)) => Some(Pat::Range(a.parse().ok()?, b.parse().ok()?)),
        None => {
            let v = s.parse().ok()?;
            Some(Pat::Range(v, v))
        }
    }
}
