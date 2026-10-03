/*
 zKølang by NØNOS
 AGPL-3.0-or-later
*/

/*!
 * The environment constant propagation carries: each name bound so far, newest last,
 * with its constant value when it has one.
 */

use alloc::string::String;
use alloc::vec::Vec;

pub(super) type Env = Vec<(String, Option<u64>)>;

/** The constant the newest binding of `name` holds, if it holds one. */
pub(super) fn latest(env: &Env, name: &str) -> Option<u64> {
    env.iter()
        .rev()
        .find(|(n, _)| n == name)
        .and_then(|(_, v)| *v)
}
