/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * Names close to one that does not resolve (section 4.4): those at most a third of its
 * length of edits away, and at least one, where a name that differs only in case is the
 * closest of all.
 */

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use super::near_distance::distance;

/** Help naming the names among `names` closest to `name`, at most three, if any is close. */
pub fn did_you_mean<'n>(name: &str, names: impl IntoIterator<Item = &'n str>) -> Option<String> {
    let len = name.chars().count();
    let limit = len.max(3) / 3;
    let mut close: Vec<(usize, &str)> = Vec::new();
    for n in names {
        if n == name || n.chars().count().abs_diff(len) > limit {
            continue;
        }
        let d = if n.eq_ignore_ascii_case(name) {
            0
        } else {
            distance(name, n)
        };
        if d <= limit {
            close.push((d, n));
        }
    }
    close.sort_unstable();
    close.dedup();
    let least = close.first()?.0;
    let near: Vec<String> = close
        .iter()
        .take_while(|c| c.0 == least)
        .take(3)
        .map(|c| format!("`{}`", c.1))
        .collect();
    match near.split_last()? {
        (last, []) => Some(format!("did you mean {last}?")),
        (last, rest) => Some(format!("did you mean {} or {last}?", rest.join(", "))),
    }
}
